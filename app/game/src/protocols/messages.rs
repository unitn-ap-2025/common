//! # Communication protocol messages
//!
//! Defines the types of messages exchanged between the different
//! components using [mpsc] channels.

use std::sync::mpsc;
use crate::components::asteroid::Asteroid;
use crate::components::rocket::Rocket;
use crate::components::sunray::Sunray;

// TODO: this is just a draft! needs to be completed

/// Messages sent by the `Orchestrator` to a `Planet`.
pub enum OrchestratorToPlanet {
    Sunray(Sunray),
    Asteroid(Asteroid),
    StartPlanetAI(StartPlanetAiMsg),
    StopPlanetAI(StopPlanetAiMsg),
}

pub struct StartPlanetAiMsg;
pub struct StopPlanetAiMsg;

/// Messages sent by a `Planet` to the `Orchestrator`.
pub enum PlanetToOrchestrator {
    SunrayAck { planet_id: u32 },
    AsteroidAck { planet_id: u32, rocket: Option<Rocket> },
    StartPlanetAIResult { planet_id: u32, timestamp: u32 },
    StopPlanetAIResult { planet_id: u32, timestamp: u32 },
}

/// Messages sent by the `Orchestrator` to an `Explorer`.
pub enum OrchestratorToExplorer {
    ResetExplorerAI(ResetExplorerAIMsg),
    MoveToPlanet(MoveToPlanet),
    CurrentPlanetRequest(CurrentPlanetRequest),
    SupportedResourceRequest(SupportedResourceRequest),
    SupportedCombinationRequest(SupportedCombinationRequest),
    GenerateResourceRequest,
    CombineResourceRequest { first: &'static str, second: &'static str },
}

pub struct ResetExplorerAIMsg;
pub struct MoveToPlanet {
    #[allow(unused)]
    sender_to_new_planet: mpsc::Sender<ExplorerToPlanet>,
}
pub struct CurrentPlanetRequest;
pub struct SupportedResourceRequest;
pub struct SupportedCombinationRequest;

/// Messages sent by an `Explorer` to the `Orchestrator`.
pub enum ExplorerToOrchestrator {
    StartExplorerAIResult,
    StopExplorerAIResult,
    MovedToPlanetResult,
    SupportedResourceResult,
    SupportedCombinationResult,
    GenerateResourceResponse { resources: Vec<String> }
}

/// Messages sent by an `Explorer` to a `Planet`.
pub enum ExplorerToPlanet {
    SupportedResourceRequest,
    SupportedCombinationRequest,
    GenerateResourceRequest(GenerateResourceRequest),
    CombineResourceRequest(CombineResourceRequest),
    EnergyCellRequest,
}

pub struct GenerateResourceRequest;
pub struct CombineResourceRequest {
    #[allow(unused)]
    first: &'static str,
    #[allow(unused)]
    second: &'static str,
}

/// Messages sent by a `Planet` to an `Explorer`.
pub enum PlanetToExplorer {
    SupportedResourceResponse,
    SupportedCombinationResponse,
    GenerateResourceResponse,
    CombineResourceResponse,
    EnergyCellResponse { number: std::num::NonZeroUsize },
}