//! # Communication protocol messages
//!
//! Defines the types of messages exchanged between the different
//! components using [mpsc] channels.

use std::sync::{mpsc};
use std::time::SystemTime;
use crate::components::asteroid::Asteroid;
use crate::components::rocket::Rocket;
use crate::components::sunray::Sunray;
use crate::components::resource::{Bag, BasicResource, BasicResourceType, ComplexResource, ComplexResourceRequest, ComplexResourceType};
use crate::components::planet::PlanetState;

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
    SunrayAck { planet_id: u32, timestamp: SystemTime },
    AsteroidAck { planet_id: u32, rocket: Option<Rocket>,}, ///depends on how we want to manage the defense + TODO add timestamp but planet code complains
    StartPlanetAIResult { planet_id: u32, timestamp: SystemTime },
    StopPlanetAIResult { planet_id: u32, timestamp: SystemTime },
    ManualStopPlanetAIResult { planet_id: u32, timestamp: SystemTime },
    ManualStartPlanetAIResult { planet_id: u32, timestamp: SystemTime },
    InternalStateResponse { planet_id: u32, planet_state: PlanetState,  timestamp: SystemTime }, //do we want to clone the planetState?, orchestrator should always know the planetState
}

/// Messages sent by the `Orchestrator` to an `Explorer`.
pub enum OrchestratorToExplorer {

    StartExplorerAI(ManualStartExplorerAIMsg),
    ResetExplorerAI(ResetExplorerAIMsg),
    MoveToPlanet(MoveToPlanet),
    CurrentPlanetRequest(CurrentPlanetRequest),
    SupportedResourceRequest(SupportedResourceRequest),
    SupportedCombinationRequest(SupportedCombinationRequest),
    GenerateResourceRequest(GenerateResourceRequest),
    CombineResourceRequest (CombineResourceRequest),
    BagContentRequest(BagContentRequestMsg),
    NeighborsResponse{ neighbors: Vec<u32> }, //do we want to send ids of the planets?

}

pub struct BagContentRequestMsg;
pub struct ManualStartExplorerAIMsg;
pub struct ResetExplorerAIMsg;
pub struct MoveToPlanet {
    #[allow(unused)]
    sender_to_new_planet: Option <mpsc::Sender<ExplorerToPlanet>>, //none if explorer asks to move to a non-adjacent planet
}
pub struct CurrentPlanetRequest;
pub struct SupportedResourceRequest;
pub struct SupportedCombinationRequest;

/// Messages sent by an `Explorer` to the `Orchestrator`.
pub enum ExplorerToOrchestrator <T> {
    StartExplorerAIResult { explorer_id: u32, timestamp: SystemTime },
    StopExplorerAIResult  { explorer_id: u32, timestamp: SystemTime },
    MovedToPlanetResult  { explorer_id: u32, timestamp: SystemTime },
    CurrentPlanetResult  { explorer_id: u32, planet_id: u32, timestamp: SystemTime },
    SupportedResourceResult { explorer_id: u32, supported_resources: Option<Vec<BasicResourceType>> ,timestamp: SystemTime, },
    SupportedCombinationResult { explorer_id: u32, combination_list: Option<Vec<ComplexResourceType>>, timestamp: SystemTime },
    GenerateResourceResponse { explorer_id: u32, generated: Result<(), ()> , timestamp: SystemTime }, //tells to the explorer if the asked resource has been generated
    CombineResourceResponse { explorer_id: u32, generated: Result<(), ()> , timestamp: SystemTime },
    BagContentResponse { explorer_id: u32, bag_content: Box<dyn Bag<T>> , timestamp: SystemTime },
    NeighborsRequest { explorer_id: u32, current_planet_id: u32, timestamp: SystemTime },
}



/// Messages sent by an `Explorer` to a `Planet`.
pub enum ExplorerToPlanet {
    SupportedResourceRequest{ explorer_id: u32 },
    SupportedCombinationRequest { explorer_id: u32 },
    GenerateResourceRequest { explorer_id: u32, msg: GenerateResourceRequest },
    CombineResourceRequest { explorer_id: u32, msg: ComplexResourceRequest },
    AvailableEnergyCellRequest { explorer_id: u32 },
    InternalStateRequest { explorer_id: u32 },
}

pub struct GenerateResourceRequest {
    #[allow(unused)]
    resource: BasicResourceType,
}
pub struct CombineResourceRequest {} ///TODO delete this line, only use to comply with old code of the orchestrator

/// Messages sent by a `Planet` to an `Explorer`.
pub enum PlanetToExplorer {
    SupportedResourceResponse { resource_list: Option<Vec<BasicResourceType>> },
    SupportedCombinationResponse { combination_list: Option<Vec<ComplexResourceType>> },
    GenerateResourceResponse { resource: Option<BasicResource> },
    CombineResourceResponse { complex_response: Option<ComplexResource> },
    AvailableEnergyCellResponse { available_cells: u32 },
    InternalStateResponse { planet_state: PlanetState },
}