//! # Communication protocol messages
//!
//! Defines the types of messages exchanged between the different
//! components using [mpsc] channels.

use crate::components::asteroid::Asteroid;
use crate::components::planet::PlanetState;
use crate::components::resource::{
    Bag, BasicResource, BasicResourceType, ComplexResource, ComplexResourceRequest,
    ComplexResourceType,
};
use crate::components::rocket::Rocket;
use crate::components::sunray::Sunray;
use std::collections::HashSet;
use std::sync::mpsc;
use std::time::SystemTime;

//placeholder for the BagContentResponse
pub struct ExplorerBag;
// TODO: this is just a draft! needs to be completed

/// Messages sent by the `Orchestrator` to a `Planet`.
pub enum OrchestratorToPlanet {
    Sunray(Sunray),
    Asteroid(Asteroid),
    StartPlanetAI(StartPlanetAiMsg),
    StopPlanetAI(StopPlanetAiMsg),
    InternalStateRequest(InternalStateRequestMsg), //I think orchestrator should always have the internal state for the UI, but up to discussions
}
pub struct StartPlanetAiMsg;
pub struct StopPlanetAiMsg;
pub struct ManualStopPlanetAiMsg;
pub struct ManualStartPlanetAiMsg;
pub struct InternalStateRequestMsg;

/// Messages sent by a `Planet` to the `Orchestrator`.
pub enum PlanetToOrchestrator {
    SunrayAck {
        planet_id: u32,
        timestamp: SystemTime,
    },
    AsteroidAck {
        planet_id: u32,
        rocket: Option<Rocket>,
    }, //depends on how we want to manage the defense + TODO add timestamp but planet code complains
    StartPlanetAIResult {
        planet_id: u32,
        timestamp: SystemTime,
    },
    StopPlanetAIResult {
        planet_id: u32,
        timestamp: SystemTime,
    },
    InternalStateResponse {
        planet_id: u32,
        planet_state: PlanetState,
        timestamp: SystemTime,
    }, //do we want to clone the planetState?, orchestrator should always know the planetState
}

/// Messages sent by the `Orchestrator` to an `Explorer`.
pub enum OrchestratorToExplorer {
    StartExplorerAI,
    ResetExplorerAI(ResetExplorerAIMsg),
    MoveToPlanet {
        sender_to_new_planet: Option<mpsc::Sender<ExplorerToPlanet>>,
    }, //none if explorer asks to move to a non-adjacent planet,
    CurrentPlanetRequest(CurrentPlanetRequest),
    SupportedResourceRequest(SupportedResourceRequest),
    SupportedCombinationRequest(SupportedCombinationRequest),
    GenerateResourceRequest {
        to_generate: BasicResourceType,
    },
    CombineResourceRequest(CombineResourceRequest),
    BagContentRequest(BagContentRequestMsg),
    NeighborsResponse {
        neighbors: Vec<u32>,
    }, //do we want to send ids of the planets?
}

pub struct BagContentRequestMsg;
pub struct ResetExplorerAIMsg;
pub struct MoveToPlanet {} //TODO: DELETE THIS LINE, USED TO COMPLY WITH OLD ORCHESTRATOR CODE
pub struct CurrentPlanetRequest;
pub struct SupportedResourceRequest;
pub struct SupportedCombinationRequest;

/// Messages sent by an `Explorer` to the `Orchestrator`.
pub enum ExplorerToOrchestrator<T> {
    StartExplorerAIResult {
        explorer_id: u32,
        timestamp: SystemTime,
    },
    StopExplorerAIResult {
        explorer_id: u32,
        timestamp: SystemTime,
    },
    MovedToPlanetResult {
        explorer_id: u32,
        timestamp: SystemTime,
    },
    CurrentPlanetResult {
        explorer_id: u32,
        planet_id: u32,
        timestamp: SystemTime,
    },
    SupportedResourceResult {
        explorer_id: u32,
        supported_resources: Option<HashSet<BasicResourceType>>,
        timestamp: SystemTime,
    },
    SupportedCombinationResult {
        explorer_id: u32,
        combination_list: Option<HashSet<ComplexResourceType>>,
        timestamp: SystemTime,
    },
    GenerateResourceResponse {
        explorer_id: u32,
        generated: Result<(), ()>,
        timestamp: SystemTime,
    }, //tells to the orchestrator if the asked resource has been generated
    CombineResourceResponse {
        explorer_id: u32,
        generated: Result<(), ()>,
        timestamp: SystemTime,
    },
    BagContentResponse {
        explorer_id: u32,
        bag_content: Box<dyn Bag<T>>,
        timestamp: SystemTime,
    },
    NeighborsRequest {
        explorer_id: u32,
        current_planet_id: u32,
        timestamp: SystemTime,
    },
    TravelToPlanetRequest {
        explorer_id: u32,
        current_planet_id: u32,
        dst_planet_id: u32,
        timestamp: SystemTime,
    },
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
    InternalStateRequest {
        explorer_id: u32,
    },
}

pub struct GenerateResourceRequest {} //TODO DELETE THIS LINE, ONLY USE TO COMPLY WITH OLD CODE OF THE ORCHESTRATOR

pub struct CombineResourceRequest {} //TODO delete this line, only use to comply with old code of the orchestrator

/// Messages sent by a `Planet` to an `Explorer`.
pub enum PlanetToExplorer {
    SupportedResourceResponse {
        resource_list: Option<HashSet<BasicResourceType>>,
    },
    SupportedCombinationResponse {
        combination_list: Option<HashSet<ComplexResourceType>>,
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
    InternalStateResponse {
        planet_state: PlanetState,
    },
}
