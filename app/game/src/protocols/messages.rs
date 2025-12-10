//! # Communication protocol messages
//!
//! Defines the types of messages exchanged between the different
//! components using [crossbeam_channel] channels.

use crate::components::asteroid::Asteroid;
use crate::components::planet::DummyPlanetState;
use crate::components::resource::{
    BasicResource, BasicResourceType, ComplexResource, ComplexResourceRequest, ComplexResourceType,
    GenericResource,
};
use crate::components::rocket::Rocket;
use crate::components::sunray::Sunray;
use crossbeam_channel::Sender;
use std::collections::HashSet;

/// Messages sent by the `Orchestrator` to a `Planet`.
#[derive(Debug)]
pub enum OrchestratorToPlanet {
    /// This variant is used to send a [Sunray] to a planet
    Sunray(Sunray),
    /// This variant is used to send an [Asteroid] to a planet
    Asteroid(Asteroid),
    /// This variant is used to start a Planet Ai and restart it if it is stopped
    StartPlanetAI,
    /// This variant is used to pause the planet Ai
    StopPlanetAI,
    /// This variant is used to kill (or destroy) the planet
    KillPlanet,
    /// This variant is used to obtain a Planet Internal State
    InternalStateRequest,
    /// This variant is used to send the new [Sender] of the incoming explorer, see the sequence diagram for more info
    IncomingExplorerRequest {
        explorer_id: u32,
        new_mpsc_sender: Sender<PlanetToExplorer>,
    },
    /// This variant is used to notify the planet to drop the [Sender] of the outgoing explorer
    OutgoingExplorerRequest { explorer_id: u32 },
}

/// Messages sent by a `Planet` to the `Orchestrator`.
#[derive(Debug)]
pub enum PlanetToOrchestrator {
    /// This variant is used to acknowledge the obtained [Sunray]
    SunrayAck { planet_id: u32 },
    /// This variant is used to acknowledge the obtained [Asteroid] and notify the orchestrator
    /// if the planet has been destroyed or not.
    AsteroidAck {
        planet_id: u32,
        rocket: Option<Rocket>,
    },
    /// This variant is used to acknowledge the start of the Planet Ai
    StartPlanetAIResult { planet_id: u32 },
    /// This variant is used to acknowledge the stop of the Planet Ai
    StopPlanetAIResult { planet_id: u32 },
    /// This variant is used to acknowledge the killing of a planet
    KillPlanetResult { planet_id: u32 },
    /// This variant is used to send back the Planet State
    InternalStateResponse {
        planet_id: u32,
        planet_state: DummyPlanetState,
    },
    /// This variant is used to acknowledge the incoming explorer
    /// Encapsulates a [Result] with a possible [Err] String representing an error occurred
    IncomingExplorerResponse {
        planet_id: u32,
        res: Result<(), String>,
    },
    /// This variant is used to acknowledge the outgoing explorer
    /// Encapsulates a [Result] with a possible [Err] String representing an error occurred
    OutgoingExplorerResponse {
        planet_id: u32,
        res: Result<(), String>,
    },
    /// This variant is used by planets that are currently in a *stopped* state
    /// to acknowledge any message coming from the Orchestrator (except for [OrchestratorToPlanet::StartPlanetAI])
    Stopped { planet_id: u32 },
}

impl PlanetToOrchestrator {
    /// Helper method to extract the `planet_id` field from any message variant
    /// without needing to match a specific one.
    pub fn planet_id(&self) -> u32 {
        match self {
            PlanetToOrchestrator::SunrayAck { planet_id, .. } => *planet_id,
            PlanetToOrchestrator::AsteroidAck { planet_id, .. } => *planet_id,
            PlanetToOrchestrator::StartPlanetAIResult { planet_id, .. } => *planet_id,
            PlanetToOrchestrator::StopPlanetAIResult { planet_id, .. } => *planet_id,
            PlanetToOrchestrator::KillPlanetResult { planet_id, .. } => *planet_id,
            PlanetToOrchestrator::InternalStateResponse { planet_id, .. } => *planet_id,
            PlanetToOrchestrator::IncomingExplorerResponse { planet_id, .. } => *planet_id,
            PlanetToOrchestrator::OutgoingExplorerResponse { planet_id, .. } => *planet_id,
            PlanetToOrchestrator::Stopped { planet_id, .. } => *planet_id,
        }
    }
}

/// Messages sent by the `Orchestrator` to an `Explorer`.
#[derive(Debug)]
pub enum OrchestratorToExplorer {
    /// This variant is used to start the Explorer AI
    StartExplorerAI,
    /// This variant is used to reset the Explorer AI
    ResetExplorerAI,
    /// This variant is used to kill the Explorer AI
    KillExplorerAI,
    /// This variant is used to send a [Sender] to the new planet
    MoveToPlanet {
        sender_to_new_planet: Option<Sender<ExplorerToPlanet>>,
    }, //none if explorer asks to move to a non-adjacent planet,
    /// This variant is used to ask the ID of the Planet in which the Explorer is currently located
    CurrentPlanetRequest,
    /// This variant is used to enforce the Explorer to ask the supported Resources on the Planet
    SupportedResourceRequest,
    /// This variant is used to enforce the Explorer to ask the supported Combinations on the Planet
    SupportedCombinationRequest,
    /// This variant is used to enforce the Explorer to ask the Planet to Generate a [BasicResource]
    GenerateResourceRequest { to_generate: BasicResourceType },
    /// This variant is used to enforce the Explorer to ask the Planet to Generate a [ComplexResource] using the [ComplexResourceRequest]
    CombineResourceRequest(ComplexResourceRequest),
    /// This variant is used to ask the content of the Explorer Bag
    BagContentRequest,
    /// This variant is used to send to the Explorer its neighbors' IDs
    NeighborsResponse { neighbors: Vec<u32> },
}

/// Messages sent by an `Explorer` to the `Orchestrator`.
#[derive(Debug)]
pub enum ExplorerToOrchestrator<T> {
    /// Acknowledge of [OrchestratorToExplorer::StartExplorerAI]
    StartExplorerAIResult { explorer_id: u32 },
    /// Acknowledge of [OrchestratorToExplorer::KillExplorerAI]
    KillExplorerAIResult { explorer_id: u32 },
    /// Acknowledge of [OrchestratorToExplorer::ResetExplorerAI]
    ResetExplorerAIResult { explorer_id: u32 },
    /// Acknowledge of [OrchestratorToExplorer::MoveToPlanet]
    MovedToPlanetResult { explorer_id: u32 },
    /// This variant is used to send the ID of the current planet on which the Explorer is located
    CurrentPlanetResult { explorer_id: u32, planet_id: u32 },
    /// This variant is used to send the list of the available [BasicResourceType] in the planet
    SupportedResourceResult {
        explorer_id: u32,
        supported_resources: HashSet<BasicResourceType>,
    },
    /// This variant is used to send the list of the available [ComplexResourceType] in the planet
    SupportedCombinationResult {
        explorer_id: u32,
        combination_list: HashSet<ComplexResourceType>,
    },
    /// This variant holds a [Result] for the Orchestrator to know if the requested [BasicResource] has been generated
    GenerateResourceResponse {
        explorer_id: u32,
        generated: Result<(), ()>,
    },
    /// This variant holds a [Result] for the Orchestrator to know if the requested [ComplexResource] has been generated
    CombineResourceResponse {
        explorer_id: u32,
        generated: Result<(), ()>,
    },
    /// This message is for passing around the bag content and has been implemented with a generic type to let the group the freedom to implement the methods on it
    ///
    /// ## Example
    /// ```ignore
    /// use std::collections::HashMap;
    /// use common_game::components::resource::{ComplexResourceType, BasicResourceType};
    /// use common_game::protocols::messages::ExplorerToOrchestrator;
    ///
    /// pub struct DummyBag {
    ///     pub complex: HashMap<ComplexResourceType, u32>,
    ///     pub basic: HashMap<BasicResourceType, u32>,
    /// }
    ///
    /// let message = ExplorerToOrchestrator::BagContentResponse {
    ///     explorer_id: 1,
    ///     bag_content: DummyBag {
    ///         complex: HashMap::new(),
    ///         basic: HashMap::new(),
    ///     }
    /// };
    /// ```
    ///
    BagContentResponse { explorer_id: u32, bag_content: T },
    /// This variant asks the Orchestrator for the list of neighbors Planets to travel to
    NeighborsRequest {
        explorer_id: u32,
        current_planet_id: u32,
    },
    /// This variant asks the Orchestrator to be sent to the specified Planet
    TravelToPlanetRequest {
        explorer_id: u32,
        current_planet_id: u32,
        dst_planet_id: u32,
    },
}

impl<T> ExplorerToOrchestrator<T> {
    /// Helper method to extract the `explorer_id` field from any message variant
    /// without needing to match a specific one.
    pub fn explorer_id(&self) -> u32 {
        match self {
            Self::StartExplorerAIResult { explorer_id, .. } => *explorer_id,
            Self::KillExplorerAIResult { explorer_id, .. } => *explorer_id,
            Self::ResetExplorerAIResult { explorer_id, .. } => *explorer_id,
            Self::MovedToPlanetResult { explorer_id, .. } => *explorer_id,
            Self::CurrentPlanetResult { explorer_id, .. } => *explorer_id,
            Self::SupportedResourceResult { explorer_id, .. } => *explorer_id,
            Self::SupportedCombinationResult { explorer_id, .. } => *explorer_id,
            Self::GenerateResourceResponse { explorer_id, .. } => *explorer_id,
            Self::CombineResourceResponse { explorer_id, .. } => *explorer_id,
            Self::BagContentResponse { explorer_id, .. } => *explorer_id,
            Self::NeighborsRequest { explorer_id, .. } => *explorer_id,
            Self::TravelToPlanetRequest { explorer_id, .. } => *explorer_id,
        }
    }
}

/// Messages sent by an `Explorer` to a `Planet`.
#[derive(Debug)]
pub enum ExplorerToPlanet {
    /// This variant is used to ask the Planet for the available [BasicResourceType]
    SupportedResourceRequest { explorer_id: u32 },
    /// This variant is used to ask the Planet for the available [ComplexResourceType]
    SupportedCombinationRequest { explorer_id: u32 },
    /// This variant is used to ask the Planet to generate a [BasicResource]
    GenerateResourceRequest {
        explorer_id: u32,
        resource: BasicResourceType,
    },
    /// This variant is used to ask the Planet to generate a [ComplexResource] using the [ComplexResourceRequest]
    CombineResourceRequest {
        explorer_id: u32,
        msg: ComplexResourceRequest,
    },
    /// This variant is used to ask the Planet for the available energy_cells number
    AvailableEnergyCellRequest { explorer_id: u32 },
}

impl ExplorerToPlanet {
    /// Helper method to extract the `explorer_id` field from any message variant
    /// without needing to match a specific one.
    pub fn explorer_id(&self) -> u32 {
        match self {
            ExplorerToPlanet::SupportedResourceRequest { explorer_id, .. } => *explorer_id,
            ExplorerToPlanet::SupportedCombinationRequest { explorer_id, .. } => *explorer_id,
            ExplorerToPlanet::GenerateResourceRequest { explorer_id, .. } => *explorer_id,
            ExplorerToPlanet::CombineResourceRequest { explorer_id, .. } => *explorer_id,
            ExplorerToPlanet::AvailableEnergyCellRequest { explorer_id, .. } => *explorer_id,
        }
    }
}

/// Messages sent by a `Planet` to an `Explorer`.
#[derive(Debug)]
pub enum PlanetToExplorer {
    /// This variant is used to send the available [BasicResourceType] list to the Explorer
    SupportedResourceResponse {
        resource_list: HashSet<BasicResourceType>,
    },
    /// This variant is used to send the available [ComplexResourceType] list to the Explorer
    SupportedCombinationResponse {
        combination_list: HashSet<ComplexResourceType>,
    },
    /// This variant is used to send the Optional [BasicResource] generated or [None] in case of errors
    GenerateResourceResponse { resource: Option<BasicResource> },
    /// This variant is used to send the [ComplexResource] generated
    /// It contains a [Result] giving back the [ComplexResource] in case of success
    /// and a triplet containing an error string and the two [GenericResource] provided by the Explorer
    CombineResourceResponse {
        complex_response: Result<ComplexResource, (String, GenericResource, GenericResource)>,
    },
    /// This variant is used to send the number of available energy cells to the Explorer
    AvailableEnergyCellResponse { available_cells: u32 },
    /// This variant is used by planets that are currently in a *stopped* state
    /// to acknowledge any message coming from an explorer
    Stopped,
}
