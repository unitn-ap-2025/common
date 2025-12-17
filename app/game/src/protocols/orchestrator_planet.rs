//! # Orchestrator and Planet protocol messages
//!
//! Defines the types of messages exchanged of the full duplex communication channel
//! between the Orchestrator and the Planets
//! For a more detailed view of the interactions between these two entities, visit the diagrams at: TODO: add link to diagrams

use crate::components::asteroid::Asteroid;
use crate::components::planet::DummyPlanetState;
use crate::components::rocket::Rocket;
use crate::components::sunray::Sunray;
use crate::protocols::planet_explorer::PlanetToExplorer;
use crate::utils::ID;
use crossbeam_channel::Sender;
use enum_as_inner::EnumAsInner;
use strum_macros::EnumDiscriminants;

/// This enum describes all possible messages from the Orchestrator to a Planet
#[derive(Debug, EnumAsInner, EnumDiscriminants)]
#[strum_discriminants(name(OrchestratorToPlanetKind))]
pub enum OrchestratorToPlanet {
    /// This variant is used to send a [Sunray] to a planet
    /// **Expected Response**: [`PlanetToOrchestrator::SunrayAck`]
    /// **Use Case**: sending a [Sunray] to charge [`energy_cell`]
    Sunray(Sunray),
    /// This variant is used to send an [Asteroid] to a planet
    /// **Expected Response**: [`PlanetToOrchestrator::AsteroidAck`]
    /// **Use Case**: sending an [Asteroid] to attack a [Planet]
    Asteroid(Asteroid),
    /// This variant is used to start a Planet AI and restart it if it is stopped
    /// **Expected Response**: [`PlanetToOrchestrator::StartPlanetAIResult`]
    /// **Use Case**: Starting the Planet AI at game start or restart the AI in case it is stopped
    StartPlanetAI,
    /// This variant is used to pause the planet Ai
    /// **Expected Response**: [`PlanetToOrchestrator::StopPlanetAIResult`]
    /// **Use Case**: Freezing Planet ability to respond to every message
    /// A planet in this state will only answer with [`PlanetToOrchestrator::Stopped`]
    StopPlanetAI,
    /// This variant is used to kill (or destroy) the planet
    /// **Expected Response**: [`PlanetToOrchestrator::KillPlanetResult`]
    /// **Use Case**: Instantly kill a Planet
    KillPlanet,
    /// This variant is used to obtain a Planet Internal State
    /// **Expected Response**: [`PlanetToOrchestrator::InternalStateResponse`]
    /// **Use Case**: The GUI can use this message to obtain the relevant info of the planet to be shown
    InternalStateRequest,
    /// This variant is used to advertise an incoming explorer to a planet
    /// **Expected Response**: [`PlanetToOrchestrator::IncomingExplorerResponse`]
    /// **Use Case**: Moving an explorer to this planet
    IncomingExplorerRequest {
        ///The incoming explorer's id
        explorer_id: ID,
        ///The new sender half of the [`crossbeam_channel`] for the planet to communicate with the incoming explorer
        new_sender: Sender<PlanetToExplorer>,
    },
    /// This variant is used to advertise an outgoing explorer to a planet
    /// **Expected Response**: [`PlanetToOrchestrator::OutgoingExplorerResponse`]
    /// **Use Case**: Asking the planet to delete the [Sender] to the outgoing explorer
    OutgoingExplorerRequest {
        ///The outgoing explorer's id
        explorer_id: ID,
    },
}

/// This enum describes all possible messages from a Planet to the Orchestrator
#[derive(Debug, EnumAsInner, EnumDiscriminants)]
#[strum_discriminants(name(PlanetToOrchestratorKind))]
pub enum PlanetToOrchestrator {
    /// This variant is used to acknowledge the obtained [Sunray]
    /// Response to [`OrchestratorToPlanet::Sunray`]
    SunrayAck {
        ///ID of the planet sending the message
        planet_id: ID,
    },
    /// This variant is used to acknowledge the obtained [Asteroid] and notify the orchestrator
    /// if the planet has a rocket to defend itself
    /// Response to [`OrchestratorToPlanet::Asteroid`]
    AsteroidAck {
        ///ID of the planet sending the message
        planet_id: ID,
        ///Optional rocket returned to the Orchestrator to decide if planet can deflect the asteroid
        rocket: Option<Rocket>,
    },
    /// This variant is used to acknowledge the starting of the Planet Ai
    /// Response to [`OrchestratorToPlanet::StartPlanetAI`]
    StartPlanetAIResult {
        ///ID of the planet sending the message
        planet_id: ID,
    },
    /// This variant is used to acknowledge the stopping of the Planet Ai, in this state a planet will only respond
    /// to incoming messages with a [`PlanetToOrchestrator::Stopped`]
    /// Response to [`OrchestratorToPlanet::StopPlanetAI`]
    StopPlanetAIResult {
        ///ID of the planet sending the message
        planet_id: ID,
    },
    /// This variant is used to acknowledge the killing of a planet, in this case the planet thread will be terminated
    /// and the planet will be deleted from the galaxy
    /// Response to [`OrchestratorToPlanet::KillPlanet`]
    KillPlanetResult { planet_id: ID },
    /// This variant is used to send back the Planet State
    /// Response to [`OrchestratorToPlanet::InternalStateRequest`]
    InternalStateResponse {
        ///ID of the planet sending the message
        planet_id: ID,
        ///A struct containing the relevant information of a Planet to be shown by the GUI
        planet_state: DummyPlanetState,
    },
    /// This variant is used to acknowledge the incoming explorer reception
    /// Response to [`OrchestratorToPlanet::IncomingExplorerRequest`]
    IncomingExplorerResponse {
        ///ID of the planet sending the message
        planet_id: ID,
        ///Incoming explorer's ID
        explorer_id: ID,
        ///Result of the operation:
        /// [Ok] if the [Sender] to the incoming explorer has been correctly set up
        /// [Err(String)] if an error occurred
        res: Result<(), String>,
    },
    /// This variant is used to acknowledge that an explorer is leaving the planet
    /// Response to [`OrchestratorToPlanet::OutgoingExplorerRequest`]
    OutgoingExplorerResponse {
        ///ID of the planet sending the message
        planet_id: ID,
        ///Incoming explorer's ID
        explorer_id: ID,
        ///Result of the operation:
        /// [Ok] if the [Sender] to the outgoing explorer has been correctly deleted
        /// [Err(String)] if an error occurred
        res: Result<(), String>,
    },
    /// This variant is used by planets that are currently in a *stopped* state
    /// to acknowledge any message coming from the Orchestrator (except for [`OrchestratorToPlanet::StartPlanetAI`])
    Stopped {
        ///ID of the planet sending the message
        planet_id: ID,
    },
}
impl PlanetToOrchestrator {
    /// Helper method to extract the `planet_id` field from any message variant
    /// without needing to match a specific one.
    #[must_use]
    pub fn planet_id(&self) -> ID {
        match self {
            PlanetToOrchestrator::SunrayAck { planet_id, .. }
            | PlanetToOrchestrator::AsteroidAck { planet_id, .. }
            | PlanetToOrchestrator::StartPlanetAIResult { planet_id, .. }
            | PlanetToOrchestrator::StopPlanetAIResult { planet_id, .. }
            | PlanetToOrchestrator::KillPlanetResult { planet_id, .. }
            | PlanetToOrchestrator::InternalStateResponse { planet_id, .. }
            | PlanetToOrchestrator::IncomingExplorerResponse { planet_id, .. }
            | PlanetToOrchestrator::OutgoingExplorerResponse { planet_id, .. }
            | PlanetToOrchestrator::Stopped { planet_id, .. } => *planet_id,
        }
    }
}
