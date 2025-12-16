use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::time::{SystemTime, UNIX_EPOCH};

use std::fmt;

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

/// Log level / channel.
#[derive(Debug, Clone)]
pub enum Channel {
    Error,
    Warning,
    Info,
    Debug,
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
    pub timestamp_unix: i64,
    pub sender_type: ActorType,
    pub sender_id: u64,
    pub receiver_type: ActorType,
    pub receiver_id: String,
    pub event_type: EventType,
    pub channel: Channel,
    pub payload: Payload,
}

impl LogEvent {
    /// Helper: create an event with the current UNIX timestamp.
    pub fn new(
        sender_type: ActorType,
        sender_id: impl Into<u64>,
        receiver_type: ActorType,
        receiver_id: impl Into<String>,
        event_type: EventType,
        channel: Channel,
        payload: Payload,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

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
        use Channel::{Error, Warning, Info, Debug, Trace};

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
