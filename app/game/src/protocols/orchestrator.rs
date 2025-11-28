use crate::components::asteroid::Asteroid;
use crate::components::planet::{Planet, PlanetAI};
use crate::components::sunray::Sunray;
use crate::protocols::messages::{
    CombineResourceRequest, CurrentPlanetRequest, GenerateResourceRequest, MoveToPlanet,
    ResetExplorerAIMsg, StartPlanetAiMsg, StopPlanetAiMsg, SupportedCombinationRequest,
    SupportedResourceRequest,
};
#[allow(unused)]
use std::sync::mpsc;

//Dummy definitions to avoid errors, waiting for other contractors to push their implementations
#[allow(unused)]
pub struct Explorer;

// Marker trait for the galaxy abstraction.
// This trait constraints the orchestrator to have a data structure that contains the galaxy information.
#[allow(unused)]
pub trait GalaxyTrait {}

// Messages that the Orchestrator can send to a Planet.
// Start/Stop AI have been wrapped in dedicated structs (StartPlanetAiMsg / StopPlanetAiMsg)
// to constrain and clarify their signatures.

// Using a struct instead of a bare enum variant argument gives us type-safety and
// the possibility to extend this message later without changing the enum shape.
#[allow(unused)]
pub trait OrchestratorTrait {
    // • Initializes planets (planet definitions are loaded from the galaxy initialization file).
    // Returns a type implementing GalaxyTrait, representing the logical galaxy abstraction.
    fn initialize_galaxy(&mut self, path: &str) -> impl GalaxyTrait;

    // • Constructs planets and explorers.

    // For now, we use a string from the initialization file to initialize every planet.
    // This matches the PDF’s notion that planet configuration is file-driven.
    fn make_planet<T: PlanetAI>(&self, init_sting: String) -> Planet<T>;

    // Creates a new explorer.
    // In the PDF, explorers are also constructed and managed by the orchestrator.
    fn make_explorer(&self) -> Explorer;

    // • Distributes all channels and starts the game.
    // The orchestrator is responsible for wiring up all entities and communication links.
    fn start_game(path: &str) -> Self;

    // • Is the only Sunray constructor.
    fn create_sunray(&self) -> Sunray;

    // • Is the only Asteroid constructor.
    fn create_asteroid(&self) -> Asteroid;

    // Functions for Orchestrator → Planet.
    // These methods conceptually wrap sending an OrchestratorToPlanet message on the correct channel.

    fn send_sunray<T, E>(&self, s: Sunray, planet_id: u32) -> Result<T, E>;

    fn send_asteroid<T, E>(&self, a: Asteroid, planet_id: u32) -> Result<T, E>;

    fn start_planet_ai<T, E>(&self, msg: StartPlanetAiMsg, planet_id: u32) -> Result<T, E>;

    fn stop_planet_ai<T, E>(&self, msg: StopPlanetAiMsg, planet_id: u32) -> Result<T, E>;

    // Functions for Orchestrator → Explorer.
    // These methods conceptually wrap sending an OrchestratorToExplorer message
    // on the explorer’s command channel.

    fn reset_explorer_ai<T, E>(&self, msg: ResetExplorerAIMsg, explorer_id: u32) -> Result<T, E>;

    fn move_to_planet<T, E>(&self, msg: MoveToPlanet) -> Result<T, E>;

    fn current_planet<T, E>(&self, msg: CurrentPlanetRequest) -> Result<T, E>;

    fn supported_resource_request<T, E>(&self, msg: SupportedResourceRequest) -> Result<T, E>;

    fn supported_combination_request<T, E>(&self, msg: SupportedCombinationRequest)
    -> Result<T, E>;

    fn generate_resource_request<T, E>(&self, msg: GenerateResourceRequest) -> Result<T, E>;

    fn combine_resource_request<T, E>(&self, msg: CombineResourceRequest) -> Result<T, E>;
}
