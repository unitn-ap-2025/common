//! # Logging infrastructure and specification
//!
//! Defines a common framework to make logging compatible between implementers.

use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::time::{SystemTime, UNIX_EPOCH};

use std::fmt;

use crate::utils::ID;

/// Who is sending / receiving this event.
#[derive(Debug, Clone)]
pub enum ActorType {
    Planet,
    Explorer,
    Orchestrator,
    User,
    Broadcast,
    SelfActor,
}

/// Provides standardiezd log levels for all implementers to use
/// Note: "event" here means a series of messages with a specific effect
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum EventType {
    // Messages between entities
    MessagePlanetToOrchestrator,
    MessagePlanetToExplorer,
    MessageOrchestratorToExplorer,
    MessageOrchestratorToPlanet,
    MessageExplorerToPlanet,
    MessageExplorerToOrchestrator,

    // Internal operations
    InternalPlanetAction,
    InternalExplorerAction,
    InternalOrchestratorAction,

    // User commands
    UserToPlanet,
    UserToExplorer,
    UserToOrchestrator,
}

/// Simple key–value payload: string → string.
pub type Payload = BTreeMap<String, String>;

#[derive(Debug, Clone)]
pub struct LogEvent {
    pub timestamp_unix: u64,
    pub sender_type: ActorType,
    pub sender_id: ID,
    pub receiver_type: ActorType,
    pub receiver_id: ID,
    pub event_type: EventType,
    pub channel: Channel,
    pub payload: Payload,
}

impl LogEvent {
    /// Helper: create an event with the current UNIX timestamp.
    pub fn new(
        sender_type: ActorType,
        sender_id: impl Into<ID>,
        receiver_type: ActorType,
        receiver_id: impl Into<ID>,
        event_type: EventType,
        channel: Channel,
        payload: Payload,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            timestamp_unix: now,
            sender_type,
            sender_id: sender_id.into(),
            receiver_type,
            receiver_id: receiver_id.into(),
            event_type,
            channel,
            payload,
        }
    }

    #[must_use]
    pub fn id_from_str(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    /// Emit this event using the `log` crate.
    ///
    /// If no logger is initialized by the final binary,
    /// this will just be a no-op (which is fine for a library).
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
        write!(
            f,
            "LogEvent {{ ts: {}, sender: {:?}#{}, receiver: {:?}/{}, event: {:?}, channel: {:?}, payload: {:?} }}",
            self.timestamp_unix,
            self.sender_type,
            self.sender_id,
            self.receiver_type,
            self.receiver_id,
            self.event_type,
            self.channel,
            self.payload
        )
    }
}
