//! Logging module for structured log events within the game application.
//!
//! This module defines standardized log channels, event types, and data
//! structures to facilitate consistent logging across different components
//! of the game, such as planets, explorers, and the orchestrator.
//!
//! It provides mechanisms to create log events with timestamps, participants,
//! and payloads, as well as utilities to emit these events using the `log` crate
//! for integration with various logging backends.
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use std::fmt;

use crate::utils::ID;

/// Sender or receiver classification for a log event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorType {
    /// Planet entity
    Planet,
    /// Explorer entity
    Explorer,
    /// Orchestrator entity
    Orchestrator,
    /// User command
    User,
    /// System-wide
    Broadcast,
    /// Self-directed (same sender and receiver)
    SelfActor,
}

/// Standardized log channels shared across the application.
/// Note: "event" here means a series of messages with a specific effect
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Channel {
    /// Anything that leads to a panic
    Error,
    /// Unexpected behavior that doesn’t stop the game/lead to a panic
    Warning,
    /// Important events, to be emitted by the Orchestrator once the last ack message in the conversation is recieved.
    /// The events this level should be used for are:
    /// - [`Planet`](`crate::components::planet`) creation,destruction,start,stop
    /// - [`Explorer`](crate#explorer) movement,death,start/stop
    Info,
    /// All other events that are not covered by [`Channel::Info`]
    Debug,
    /// All messages
    Trace,
}

/// High-level event categories.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    /// Message between planet and orchestrator
    MessagePlanetToOrchestrator,
    /// Message between orchestrator and planet
    MessageOrchestratorToPlanet,
    /// Message between planet and explorer
    MessagePlanetToExplorer,
    /// Message between orchestrator and explorer
    MessageOrchestratorToExplorer,
    /// Message between explorer and planet
    MessageExplorerToPlanet,
    /// Message between explorer and orchestrator
    MessageExplorerToOrchestrator,

    /// Internal planet action
    InternalPlanetAction,
    /// Internal explorer action
    InternalExplorerAction,
    /// Internal orchestrator action
    InternalOrchestratorAction,

    /// User command to planet
    UserToPlanet,
    /// User command to explorer
    UserToExplorer,
    /// User command to orchestrator
    UserToOrchestrator,
}

/// Simple key–value payload: string → string.
pub type Payload = BTreeMap<String, String>;

/// Participant in a log event. Either side of an interaction can be absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Participant {
    /// Entity role that produced or received the event.
    pub actor_type: ActorType,
    /// Stable identifier for the actor.
    ///
    /// It is suggested to use id 0 for [`ActorType::Orchestrator`].
    pub id: ID,
}

impl Participant {
    /// Create a new participant with a concrete type and identifier.
    pub fn new(actor_type: ActorType, id: impl Into<ID>) -> Self {
        Self {
            actor_type,
            id: id.into(),
        }
    }
}

/// Bundle of data emitted through the logging system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEvent {
    /// UNIX timestamp in seconds when the event was created.
    pub timestamp_unix: u64,
    /// Optional sender of the event.
    pub sender: Option<Participant>,
    /// Optional receiver of the event.
    pub receiver: Option<Participant>,
    /// High-level event category.
    pub event_type: EventType,
    /// Logging channel / severity level.
    pub channel: Channel,
    /// Arbitrary key–value payload.
    pub payload: Payload,
}

impl LogEvent {
    /// Create an event with the current UNIX timestamp and optional participants.
    ///
    /// The timestamp uses `SystemTime::now().duration_since(UNIX_EPOCH)` and
    /// falls back to `0` if the clock is earlier than the Unix epoch. This
    /// avoids panics on misconfigured systems while still producing a stable,
    /// clearly out-of-band value.
    #[must_use]
    pub fn new(
        sender: Option<Participant>,
        receiver: Option<Participant>,
        event_type: EventType,
        channel: Channel,
        payload: Payload,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();

        Self {
            timestamp_unix: now,
            sender,
            receiver,
            event_type,
            channel,
            payload,
        }
    }

    /// Convenience: broadcast from a known sender to no specific receiver.
    #[must_use]
    pub fn broadcast(
        sender: Participant,
        event_type: EventType,
        channel: Channel,
        payload: Payload,
    ) -> Self {
        Self::new(Some(sender), None, event_type, channel, payload)
    }

    /// Convenience: emit an event without sender or receiver (e.g. system state).
    #[must_use]
    pub fn system(event_type: EventType, channel: Channel, payload: Payload) -> Self {
        Self::new(None, None, event_type, channel, payload)
    }

    /// Convenience: emit an event where sender and receiver are the same actor.
    #[must_use]
    pub fn self_directed(
        actor: Participant,
        event_type: EventType,
        channel: Channel,
        payload: Payload,
    ) -> Self {
        Self::new(
            Some(actor.clone()),
            Some(actor),
            event_type,
            channel,
            payload,
        )
    }

    #[must_use]
    /// Generate a deterministic identifier from an arbitrary string.
    pub fn id_from_str(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    /// Emit this event using the `log` crate.
    ///
    /// Uses the `Debug` representation to preserve all structured fields. If no
    /// logger is initialized by the final binary this is a no-op, which is fine
    /// for library consumers.
    pub fn emit(&self) {
        use Channel::{Debug, Error, Info, Trace, Warning};

        match self.channel {
            Error => log::error!("{self:?}"),
            Warning => log::warn!("{self:?}"),
            Info => log::info!("{self:?}"),
            Debug => log::debug!("{self:?}"),
            Trace => log::trace!("{self:?}"),
        }
    }
}

impl fmt::Display for LogEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sender = self.sender.as_ref().map_or_else(
            || "none".to_string(),
            |p| format!("{:?}#{}", p.actor_type, p.id),
        );

        let receiver = self.receiver.as_ref().map_or_else(
            || "none".to_string(),
            |p| format!("{:?}#{}", p.actor_type, p.id),
        );

        write!(
            f,
            "LogEvent {{ ts: {}, sender: {}, receiver: {}, event: {:?}, channel: {:?}, payload: {:?} }}",
            self.timestamp_unix, sender, receiver, self.event_type, self.channel, self.payload
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{Level, Log, Metadata, Record};
    use std::sync::{Mutex, Once};

    static LOGGER: TestLogger = TestLogger {
        messages: Mutex::new(Vec::new()),
    };
    static LOGGER_INIT: Once = Once::new();

    struct TestLogger {
        messages: Mutex<Vec<(Level, String)>>,
    }

    impl Log for TestLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let mut guard = self.messages.lock().expect("logger mutex poisoned");
                guard.push((record.level(), format!("{}", record.args())));
            }
        }

        fn flush(&self) {}
    }

    fn init_logger() {
        LOGGER_INIT.call_once(|| {
            log::set_logger(&LOGGER).expect("failed to install test logger");
            log::set_max_level(log::LevelFilter::Trace);
        });

        LOGGER
            .messages
            .lock()
            .expect("logger mutex poisoned")
            .clear();
    }

    fn sample_payload() -> Payload {
        let mut payload = Payload::new();
        payload.insert("key".into(), "value".into());
        payload
    }

    fn sample_participant(actor_type: ActorType, id: ID) -> Participant {
        Participant::new(actor_type, id)
    }

    #[test]
    fn new_populates_timestamp_and_participants() {
        let sender = sample_participant(ActorType::User, 1);
        let receiver = sample_participant(ActorType::Planet, 2);

        let event = LogEvent::new(
            Some(sender.clone()),
            Some(receiver.clone()),
            EventType::MessageExplorerToPlanet,
            Channel::Info,
            sample_payload(),
        );

        assert!(event.timestamp_unix > 0);
        assert_eq!(event.sender, Some(sender));
        assert_eq!(event.receiver, Some(receiver));
    }

    #[test]
    fn id_from_str_is_deterministic() {
        let id1 = LogEvent::id_from_str("example");
        let id2 = LogEvent::id_from_str("example");
        let id3 = LogEvent::id_from_str("different");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn broadcast_event_has_no_receiver() {
        let event = LogEvent::broadcast(
            sample_participant(ActorType::Explorer, 7),
            EventType::MessageExplorerToOrchestrator,
            Channel::Debug,
            sample_payload(),
        );

        assert!(event.receiver.is_none());
        assert!(event.sender.is_some());
    }

    #[test]
    fn system_event_has_no_participants() {
        let event = LogEvent::system(
            EventType::InternalOrchestratorAction,
            Channel::Trace,
            sample_payload(),
        );

        assert!(event.sender.is_none());
        assert!(event.receiver.is_none());
    }

    #[test]
    fn self_directed_event_sets_both_sides() {
        let actor = sample_participant(ActorType::Planet, 3);
        let event = LogEvent::self_directed(
            actor.clone(),
            EventType::InternalPlanetAction,
            Channel::Warning,
            sample_payload(),
        );

        assert_eq!(event.sender, Some(actor.clone()));
        assert_eq!(event.receiver, Some(actor));
    }

    #[test]
    fn display_formats_optional_participants() {
        let mut event = LogEvent::system(
            EventType::InternalExplorerAction,
            Channel::Info,
            sample_payload(),
        );

        event.timestamp_unix = 42;
        let rendered = format!("{event}");

        assert!(rendered.contains("ts: 42"));
        assert!(rendered.contains("sender: none"));
        assert!(rendered.contains("receiver: none"));
    }

    #[test]
    fn emit_writes_to_logger_with_channel_level() {
        init_logger();

        let mut event = LogEvent::broadcast(
            sample_participant(ActorType::User, 9),
            EventType::UserToExplorer,
            Channel::Error,
            sample_payload(),
        );

        event.timestamp_unix = 7;
        event.emit();

        let guard = LOGGER.messages.lock().expect("logger mutex poisoned");

        let (level, message) = guard.last().expect("expected a logged message");
        assert_eq!(*level, Level::Error);
        assert!(message.contains("LogEvent"));
        assert!(message.contains("sender:"));
    }
}
