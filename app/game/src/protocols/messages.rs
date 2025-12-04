//! # Communication protocol messages
//!
//! Defines the types of messages exchanged between the different
//! components using [mpsc] channels.

use crate::components::asteroid::Asteroid;
use crate::components::planet::PlanetState;
use crate::components::resource::{
    BasicResource, BasicResourceType, ComplexResource, ComplexResourceRequest, ComplexResourceType,
};
use crate::components::rocket::Rocket;
use crate::components::sunray::Sunray;
use std::collections::HashSet;
use std::sync::mpsc;

/// Messages sent by the `Orchestrator` to a `Planet`.
pub enum OrchestratorToPlanet {
    Sunray(Sunray),
    Asteroid(Asteroid),
    StartPlanetAI,
    StopPlanetAI,
    InternalStateRequest,
    IncomingExplorerRequest {
        explorer_id: u32,
        new_mpsc_sender: mpsc::Sender<PlanetToExplorer>,
    },
    OutgoingExplorerRequest {
        explorer_id: u32,
    },
}

/// Messages sent by a `Planet` to the `Orchestrator`.
pub enum PlanetToOrchestrator {
    SunrayAck {
        planet_id: u32,
    },
    AsteroidAck {
        planet_id: u32,
        rocket: Option<Rocket>,
    },
    StartPlanetAIResult {
        planet_id: u32,
    },
    StopPlanetAIResult {
        planet_id: u32,
    },
    InternalStateResponse {
        planet_id: u32,
        planet_state: PlanetState,
    },
    IncomingExplorerResponse {
        planet_id: u32,
        res: Result<(), String>,
    },
    OutgoingExplorerResponse {
        planet_id: u32,
        res: Result<(), String>,
    },
}

/// Messages sent by the `Orchestrator` to an `Explorer`.
pub enum OrchestratorToExplorer {
    StartExplorerAI,
    ResetExplorerAI,
    KillExplorerAI,
    MoveToPlanet {
        sender_to_new_planet: Option<mpsc::Sender<ExplorerToPlanet>>,
    }, //none if explorer asks to move to a non-adjacent planet,
    CurrentPlanetRequest,
    SupportedResourceRequest,
    SupportedCombinationRequest,
    GenerateResourceRequest {
        to_generate: BasicResourceType,
    },
    CombineResourceRequest(ComplexResourceRequest),
    BagContentRequest,
    NeighborsResponse {
        neighbors: Vec<u32>,
    }, //do we want to send ids of the planets?
}

/// Messages sent by an `Explorer` to the `Orchestrator`.
pub enum ExplorerToOrchestrator<T> {
    StartExplorerAIResult {
        explorer_id: u32,
    },
    KillExplorerAIResult {
        explorer_id: u32,
    },
    ResetExplorerAIResult {
        explorer_id: u32,
    },
    MovedToPlanetResult {
        explorer_id: u32,
    },
    CurrentPlanetResult {
        explorer_id: u32,
        planet_id: u32,
    },
    SupportedResourceResult {
        explorer_id: u32,
        supported_resources: HashSet<BasicResourceType>,
    },
    SupportedCombinationResult {
        explorer_id: u32,
        combination_list: HashSet<ComplexResourceType>,
    },
    GenerateResourceResponse {
        explorer_id: u32,
        generated: Result<(), ()>,
    }, //tells to the orchestrator if the asked resource has been generated
    CombineResourceResponse {
        explorer_id: u32,
        generated: Result<(), ()>,
    },
    /// This message is for passing around the bag content and has been implemented with a generic type to let the group the freedom to implement the methods on it
    ///
    /// ## Example
    /// ```
    /// use common_game::protocols::messages::ExplorerToOrchestrator;
    /// use common_game::components::resource::{ComplexResourceType, BasicResourceType};
    /// use std::collections::HashMap;
    ///
    /// let _ = ExplorerToOrchestrator::BagContentResponse {
    ///     explorer_id: 1,
    ///     bag_content: DummyBag { complex: HashMap::new(), basic: HashMap::new() }
    /// };
    ///
    /// pub struct DummyBag {
    ///     pub complex : HashMap<ComplexResourceType, u32>,
    ///     pub basic : HashMap<BasicResourceType, u32>,
    /// }
    ///  ```
    ///
    BagContentResponse {
        explorer_id: u32,
        bag_content: T,
    },
    NeighborsRequest {
        explorer_id: u32,
        current_planet_id: u32,
    },
    TravelToPlanetRequest {
        explorer_id: u32,
        current_planet_id: u32,
        dst_planet_id: u32,
    },
}

impl<T> ExplorerToOrchestrator<T> {
    /// Helper method to extract the `explorer_id` from any message variant
    /// without needing to match the specific enum variant.
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
pub enum ExplorerToPlanet {
    SupportedResourceRequest {
        explorer_id: u32,
    },
    SupportedCombinationRequest {
        explorer_id: u32,
    },
    GenerateResourceRequest {
        explorer_id: u32,
        resource: BasicResourceType,
    },
    CombineResourceRequest {
        explorer_id: u32,
        msg: ComplexResourceRequest,
    },
    AvailableEnergyCellRequest {
        explorer_id: u32,
    },
}

impl ExplorerToPlanet {
    /// Helper method to extract the `explorer_id` from any message variant
    /// without needing to match the specific enum variant.
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
pub enum PlanetToExplorer {
    SupportedResourceResponse {
        resource_list: HashSet<BasicResourceType>,
    },
    SupportedCombinationResponse {
        combination_list: HashSet<ComplexResourceType>,
    },
    GenerateResourceResponse {
        resource: Option<BasicResource>,
    },
    CombineResourceResponse {
        complex_response: Option<ComplexResource>,
    },
    AvailableEnergyCellResponse {
        available_cells: u32,
    },
}
