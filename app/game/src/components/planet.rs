//! # Planet module
//! This module provides common definitions for planets and their associated types
//! that need be used by a group to construct its own planet.
//! The [Planet] struct is the **main component**: an instance of it represents the
//! actual planet and contains all the logic and state (see [PlanetState]) needed to work as one, in fact
//! this is what the orchestrator will interact with.
//!
//! You can instantiate a new planet by calling the [Planet::new] constructor method and passing
//! valid construction parameters to it (look into its documentation to learn more).
//!
//! One of the construction parameters is a group-defined struct that implements the [PlanetAI] trait,
//! which defines several methods for handling messages coming from the orchestrator and the explorers. This is
//! the core of each group's planet implementation, as it defines the planet *behaviour*, that is
//! how a planet "reacts" to the possible events or requests.
//!
//! ## Examples
//! Intended usage (for planet definition, by groups):
//!
//! ```
//! use std::sync::mpsc;
//! use common_game::components::planet::{Planet, PlanetAI, PlanetState, PlanetType};
//! use common_game::components::resource::{Combinator, Generator};
//! use common_game::components::rocket::Rocket;
//! use common_game::protocols::messages;
//!
//! // Group-defined AI struct
//! struct AI { /* your AI state here */ };
//!
//! impl PlanetAI for AI {
//!     fn handle_orchestrator_msg(
//!         &mut self,
//!         state: &mut PlanetState,
//!         generator: &Generator,
//!         combinator: &Combinator,
//!         msg: messages::OrchestratorToPlanet
//!     ) -> Option<messages::PlanetToOrchestrator> {
//!         // your handler code here...
//!         None
//!     }
//!
//!     fn handle_explorer_msg(
//!         &mut self,
//!         state: &mut PlanetState,
//!         generator: &Generator,
//!         combinator: &Combinator,
//!         msg: messages::ExplorerToPlanet
//!     ) -> Option<messages::PlanetToExplorer> {
//!         // your handler code here...
//!         None
//!     }
//!
//!     fn handle_asteroid(
//!         &mut self,
//!         state: &mut PlanetState,
//!         generator: &Generator,
//!         combinator: &Combinator,
//!     ) -> Option<Rocket> {
//!         // your handler code here...
//!         None
//!     }
//!
//!     fn start(&mut self, state: &PlanetState) { /* startup code */ }
//!     fn stop(&mut self, state: &PlanetState) { /* stop code */ }
//! }
//!
//! // This is the group's "export" function. It will be called by
//! // the orchestrator to spawn your planet.
//! pub fn create_planet(
//!     rx_orchestrator: mpsc::Receiver<messages::OrchestratorToPlanet>,
//!     tx_orchestrator: mpsc::Sender<messages::PlanetToOrchestrator>,
//!     rx_explorer: mpsc::Receiver<messages::ExplorerToPlanet>,
//!     tx_explorer: mpsc::Sender<messages::PlanetToExplorer>
//! ) -> Planet<AI> {
//!     let id = 1;
//!     let ai = AI {};
//!     let gen_rules = vec![/* your recipes */];
//!     let comb_rules = vec![/* your recipes */];
//!
//!     // Construct the planet and return it
//!     Planet::new(
//!         id,
//!         PlanetType::A,
//!         ai,
//!         gen_rules,
//!         comb_rules,
//!         (rx_orchestrator, tx_orchestrator),
//!         (rx_explorer, tx_explorer)
//!     ).unwrap() // Don't call .unwrap()! You should do error checking instead.
//! }
//! ```

use std::slice::{Iter, IterMut};
use std::sync::mpsc;

use crate::components::energy_cell::EnergyCell;
use crate::components::resource::{BasicResourceType, Combinator, ComplexResourceType, Generator};
use crate::components::rocket::Rocket;
use crate::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
};

/// The trait that defines the behaviour of a planet.
///
/// Structs implementing this trait are intended to be passed to the
/// [Planet] constructor, so that the handlers can be invoked by the planet
/// internal logic when certain messages are received on any of the planet channels.
///
/// The handlers can alter the planet state by accessing the
/// `state` parameter, which is passed to the methods as a mutable borrow.
/// A response can be sent by returning an optional message of the correct type,
/// that will be forwarded to the associated channel passed on planet construction.
pub trait PlanetAI {
    /// Handler for messages received by the orchestrator (receiving
    /// end of the [OrchestratorToPlanet] channel).
    /// The following messages will **not** invoke this handler:
    /// - [OrchestratorToPlanet::StartPlanetAI] (see [PlanetAI::start])
    /// - [OrchestratorToPlanet::StopPlanetAI] (see [PlanetAI::stop])
    /// - [OrchestratorToPlanet::Asteroid] (see [PlanetAI::handle_asteroid])
    ///
    /// Check [PlanetAI] docs for general meaning of the parameters and return type.
    fn handle_orchestrator_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: OrchestratorToPlanet,
    ) -> Option<PlanetToOrchestrator>;

    /// Handler for **all** messages received by an explorer (receiving
    /// end of the [ExplorerToPlanet] channel).
    ///
    /// Check [PlanetAI] docs for general meaning of the parameters and return type.
    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer>;

    /// This handler will be invoked when a [OrchestratorToPlanet::Asteroid]
    /// message is received. It's important to handle *Asteroid* messages
    /// correctly, as this will the determine the planet survival.
    ///
    /// # Returns
    /// In order to survive, an owned [Rocket] **must** be returned from this method;
    /// if `None` is returned instead, the planet will (or *should*) be **destroyed** by the orchestrator
    fn handle_asteroid(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
    ) -> Option<Rocket>;

    /// This method will be invoked when a [OrchestratorToPlanet::StartPlanetAI]
    /// is received, but **only if** the planet is currently in a *stopped* state.
    ///
    /// Start messages received when planet is already running are **ignored**.
    fn start(&mut self, state: &PlanetState);

    /// This method will be invoked when a [OrchestratorToPlanet::StopPlanetAI]
    /// is received, but **only if** the planet is currently in a *running* state.
    ///
    /// Stop messages received when planet is already stopped are **ignored**.
    fn stop(&mut self, state: &PlanetState);
}

// Defines the planet rules constraints
struct PlanetConstraints {
    n_energy_cells: usize,
    unbounded_gen_rules: bool,
    can_have_rocket: bool,
    n_comb_rules: usize,
}

/// Planet types definitions, intended to be passed
/// to the planet constructor. Identifies the planet rules constraints,
/// with each type having its own.
#[derive(Debug, Clone, Copy)]
pub enum PlanetType {
    A,
    B,
    C,
    D,
}

impl PlanetType {
    const N_ENERGY_CELLS: usize = 5;
    const N_RESOURCE_COMB_RULES: usize = 6;

    // Returns a tuple with the constraints associated with the planet type,
    // as described in the project specifications.
    fn constraints(&self) -> PlanetConstraints {
        match self {
            PlanetType::A => PlanetConstraints {
                n_energy_cells: Self::N_ENERGY_CELLS,
                unbounded_gen_rules: false,
                can_have_rocket: true,
                n_comb_rules: 0,
            },
            PlanetType::B => PlanetConstraints {
                n_energy_cells: 1,
                unbounded_gen_rules: true,
                can_have_rocket: false,
                n_comb_rules: 1,
            },
            PlanetType::C => PlanetConstraints {
                n_energy_cells: 1,
                unbounded_gen_rules: false,
                can_have_rocket: true,
                n_comb_rules: Self::N_RESOURCE_COMB_RULES,
            },
            PlanetType::D => PlanetConstraints {
                n_energy_cells: Self::N_ENERGY_CELLS,
                unbounded_gen_rules: true,
                can_have_rocket: false,
                n_comb_rules: 0,
            },
        }
    }
}

/// This struct is a representation of the internal state
/// of the planet. Through its public methods, it gives access to the all resources
/// of the planet:
/// - A vec of [EnergyCell].
/// - An optional [Rocket], that can be built accordingly to the planet type.
/// - [Generator] for generating basic resources.
/// - [Combinator] for combining basic resources into complex ones.
pub struct PlanetState {
    id: u32,
    energy_cells: Vec<EnergyCell>,
    rocket: Option<Rocket>,
    can_have_rocket: bool,
}

impl PlanetState {
    /// Returns the planet id.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Indexed getter accessor for the [EnergyCell] vec.
    ///
    /// # Returns
    /// An immutable borrow of the *i-th* energy cell.
    ///
    /// # Panics
    /// This method will panic if the index `i` is out of bounds.
    /// Always check the number of energy cells available with [cells_count].
    pub fn cell(&self, i: usize) -> &EnergyCell {
        &self.energy_cells[i]
    }

    /// Indexed *mutable* getter accessor for the [EnergyCell] vec.
    ///
    /// # Returns
    /// An mutable borrow of the *i-th* energy cell.
    ///
    /// # Panics
    /// This method will panic if the index `i` is out of bounds.
    /// Always check the number of energy cells available with [cells_count].
    pub fn cell_mut(&mut self, i: usize) -> &mut EnergyCell {
        &mut self.energy_cells[i]
    }

    /// Returns the number of energy cells owned by
    /// the planet. This is the actual size of the internal
    /// vec containing the cells.
    pub fn cells_count(&self) -> usize {
        self.energy_cells.len()
    }

    /// Returns an *immutable* iterator over the energy cells owned by the planet.
    pub fn cells_iter(&self) -> Iter<'_, EnergyCell> {
        self.energy_cells.iter()
    }

    /// Returns a *mutable* iterator over the energy cells owned by the planet.
    pub fn cells_iter_mut(&mut self) -> IterMut<'_, EnergyCell> {
        self.energy_cells.iter_mut()
    }

    /// Returns `true` if the planet can have a rocket.
    pub fn can_have_rocket(&self) -> bool {
        self.can_have_rocket
    }

    /// Returns `true` if the planet has a rocket built and ready to launch.
    pub fn has_rocket(&self) -> bool {
        self.rocket.is_some()
    }

    /// Takes the rocket out of the planet state (if there is one), leaving
    /// `None` in its place.
    pub fn take_rocket(&mut self) -> Option<Rocket> {
        self.rocket.take()
    }

    /// Constructs a rocket using the *i-th* [EnergyCell] of the planet and stores it
    /// inside the planet, taking ownership of it.
    ///
    /// # Panics
    /// This method will panic if the index `i` is out of bounds.
    /// Always check the number of energy cells available with [cells_count].
    ///
    /// # Errors
    /// Returns an error if:
    /// - The planet type prohibits the storing of rockets.
    /// - The planet already has a rocket built.
    /// - The energy cell is not charged
    pub fn build_rocket(&mut self, i: usize) -> Result<(), String> {
        if !self.can_have_rocket {
            Err("This planet type can't have rockets.".to_string())
        } else if self.has_rocket() {
            Err("This planet already has a rocket.".to_string())
        } else {
            let energy_cell = self.cell_mut(i);
            Rocket::new(energy_cell).map(|rocket| {
                self.rocket = Some(rocket);
            })
        }
    }
}

/// Main, top-level planet definition. This type is built on top of
/// [PlanetState], [PlanetType] and [PlanetAI], through composition.
///
/// It needs to be constructed by each group as it represents the actual planet
/// and contains the base logic that runs the AI. Also, this is what should be
/// returned to the orchestrator.
///
/// See module-level docs for more general info.
pub struct Planet<T: PlanetAI> {
    state: PlanetState,
    planet_type: PlanetType,
    pub ai: T,
    generator: Generator,
    combinator: Combinator,

    from_orchestrator: mpsc::Receiver<OrchestratorToPlanet>,
    to_orchestrator: mpsc::Sender<PlanetToOrchestrator>,
    from_explorer: mpsc::Receiver<ExplorerToPlanet>,
    to_explorer: mpsc::Sender<PlanetToExplorer>,
}

impl<T: PlanetAI> Planet<T> {
    /// Constructor for the [Planet] type.
    ///
    /// # Errors
    /// Returns an error if the construction parameters are *invalid* (they violate the `planet_type` constraints).
    ///
    /// # Arguments
    /// - `id` - The identifier to assign to the planet.
    /// - `planet_type` - Type of the planet. Constraints the rules of the planet.
    /// - `ai` - A group-defined struct implementing the [PlanetAI] trait.
    /// - `gen_rules` - A vec of [BasicResourceType] containing the basic resources the planet will be able to generate.
    /// - `comb_rules` - A vec of [ComplexResourceType] containing the complex resources the planet will be able to make.
    /// - `orchestrator_channels` - A pair containing the [mpsc::Receiver] and [mpsc::Sender] half
    ///   of the channels [OrchestratorToPlanet] and [PlanetToOrchestrator].
    /// - `explorer_channels` - A pair containing the [mpsc::Receiver] and [mpsc::Sender] half
    ///   of the channels [ExplorerToPlanet] and [PlanetToExplorer].
    pub fn new(
        id: u32,
        planet_type: PlanetType,
        ai: T,
        gen_rules: Vec<BasicResourceType>,
        comb_rules: Vec<ComplexResourceType>,
        orchestrator_channels: (
            mpsc::Receiver<OrchestratorToPlanet>,
            mpsc::Sender<PlanetToOrchestrator>,
        ),
        explorer_channels: (
            mpsc::Receiver<ExplorerToPlanet>,
            mpsc::Sender<PlanetToExplorer>,
        ),
    ) -> Result<Planet<T>, String> {
        let PlanetConstraints {
            n_energy_cells,
            unbounded_gen_rules,
            can_have_rocket,
            n_comb_rules,
        } = planet_type.constraints();
        let (from_orchestrator, to_orchestrator) = orchestrator_channels;
        let (from_explorer, to_explorer) = explorer_channels;

        if gen_rules.is_empty() {
            Err("gen_rules is empty".to_string())
        } else if !unbounded_gen_rules && gen_rules.len() > 1 {
            Err(format!(
                "Too many generation rules (Planet type {:?} is limited to 1)",
                planet_type
            ))
        } else if comb_rules.len() > n_comb_rules {
            Err(format!(
                "Too many combination rules (Planet type {:?} is limited to {})",
                planet_type, n_comb_rules
            ))
        } else {
            let mut generator = Generator::new();
            let mut combinator = Combinator::new();

            // add gen and comb rules to the planet generator and combinator
            for r in gen_rules {
                let _ = generator.add(r);
            }
            for r in comb_rules {
                let _ = combinator.add(r);
            }

            Ok(Planet {
                state: PlanetState {
                    id,
                    energy_cells: (0..n_energy_cells).map(|_| EnergyCell::new()).collect(),
                    can_have_rocket,
                    rocket: None,
                },
                planet_type,
                ai,
                generator,
                combinator,
                from_orchestrator,
                to_orchestrator,
                from_explorer,
                to_explorer,
            })
        }
    }

    /// Starts the planet in a *stopped* state, waiting for a [OrchestratorToPlanet::StartPlanetAI],
    /// then invokes [PlanetAI::start] and runs the main message polling loop.
    /// See [PlanetAI] docs to know more about when handlers are invoked and how the planet reacts
    /// to the different messages.
    ///
    /// This method is *blocking* and should be called by the orchestrator in a separate thread.
    ///
    /// # Errors
    /// This method should not return, but if it does, it returns an error message
    /// describing the cause. For now, the only possible cause of error is orchestrator disconnection
    /// from one of the channels.
    pub fn run(&mut self) -> Result<(), String> {
        // run the planet stopped by default
        // and wait for a StartPlanetAI message
        self.wait_for_start()?;

        self.ai.start(&self.state);

        // maybe spawn a thread for async event handling ?
        loop {
            // TODO: disconnection error handling

            // orchestrator incoming message polling
            match self.from_orchestrator.try_recv() {
                // TODO: do something with the StartPlanetAI message content
                Ok(OrchestratorToPlanet::StartPlanetAI(_)) => {}
                // TODO: do something with the StopPlanetAI message content
                Ok(OrchestratorToPlanet::StopPlanetAI(_)) => {
                    self.ai.stop(&self.state);
                    self.wait_for_start()?; // blocking wait

                    // restart AI
                    self.ai.start(&self.state)
                }
                Ok(OrchestratorToPlanet::Asteroid(_)) => {
                    let rocket =
                        self.ai
                            .handle_asteroid(&mut self.state, &self.generator, &self.combinator);
                    self.to_orchestrator
                        .send(PlanetToOrchestrator::AsteroidAck {
                            planet_id: self.id(),
                            rocket,
                        })
                        .map_err(|_| "Orchestrator disconnected".to_string())?;
                }
                Ok(msg) => {
                    self.ai
                        .handle_orchestrator_msg(
                            &mut self.state,
                            &self.generator,
                            &self.combinator,
                            msg,
                        )
                        .map(|response| self.to_orchestrator.send(response))
                        .transpose()
                        .map_err(|_| "Orchestrator disconnected".to_string())?;
                }

                Err(mpsc::TryRecvError::Disconnected) => {
                    return Err("Orchestrator disconnected".to_string());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }

            // explorer incoming message polling
            match self.from_explorer.try_recv() {
                Ok(msg) => {
                    if let Some(response) = self.ai.handle_explorer_msg(
                        &mut self.state,
                        &self.generator,
                        &self.combinator,
                        msg,
                    ) {
                        self.to_explorer
                            .send(response)
                            .unwrap_or_else(|_| println!("No explorer connected!"))
                    }
                }

                Err(mpsc::TryRecvError::Disconnected) => {}
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
    }

    // private helper function that blocks until
    // a StartPlanetAI message is received
    fn wait_for_start(&self) -> Result<(), String> {
        loop {
            // TODO: error handling
            let recv_re = self.from_orchestrator.recv();
            match recv_re {
                // TODO: do something with the StartPlanetAI message content
                Ok(OrchestratorToPlanet::StartPlanetAI(_)) => return Ok(()),
                Err(_) => return Err("Orchestrator disconnected!".to_string()),
                _ => {}
            }
        }
    }

    /// Returns the planet id.
    pub fn id(&self) -> u32 {
        self.state.id
    }

    /// Returns the planet type.
    pub fn planet_type(&self) -> PlanetType {
        self.planet_type
    }

    /// Returns an immutable borrow the planet internal state.
    pub fn state(&self) -> &PlanetState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Duration, SystemTime};

    use crate::components::asteroid::Asteroid;
    use crate::components::energy_cell::EnergyCell;
    use crate::components::resource::{BasicResourceType, Combinator, Generator};
    use crate::components::rocket::Rocket;
    use crate::components::sunray::Sunray;
    use crate::protocols::messages::{
        ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
        StartPlanetAiMsg, StopPlanetAiMsg,
    };

    // --- Mock AI ---
    struct MockAI {
        start_called: bool,
        stop_called: bool,
        sunray_count: u32,
    }

    impl MockAI {
        fn new() -> Self {
            Self {
                start_called: false,
                stop_called: false,
                sunray_count: 0,
            }
        }
    }

    impl PlanetAI for MockAI {
        fn handle_orchestrator_msg(
            &mut self,
            state: &mut PlanetState,
            _generator: &Generator,
            _combinator: &Combinator,
            msg: OrchestratorToPlanet,
        ) -> Option<PlanetToOrchestrator> {
            match msg {
                OrchestratorToPlanet::Sunray(s) => {
                    self.sunray_count += 1;

                    if let Some(cell) = state.cells_iter_mut().next() {
                        cell.charge(s);
                    }

                    Some(PlanetToOrchestrator::SunrayAck {
                        planet_id: state.id(),
                        timestamp: SystemTime::now(),
                    })
                }
                _ => None,
            }
        }

        fn handle_explorer_msg(
            &mut self,
            _state: &mut PlanetState,
            _generator: &Generator,
            _combinator: &Combinator,
            _msg: ExplorerToPlanet,
        ) -> Option<PlanetToExplorer> {
            None
        }

        fn handle_asteroid(
            &mut self,
            state: &mut PlanetState,
            _generator: &Generator,
            _combinator: &Combinator,
        ) -> Option<Rocket> {
            if let Some(idx) = state.cells_iter().position(|c| c.is_charged()) {
                if state.build_rocket(idx).is_ok() {
                    return state.take_rocket();
                }
            }
            None
        }

        fn start(&mut self, _state: &PlanetState) {
            self.start_called = true;
        }

        fn stop(&mut self, _state: &PlanetState) {
            self.stop_called = true;
        }
    }

    // --- Helper for creating dummy channels ---
    // Returns the halves required by Planet::new
    type OrchChannels = (
        mpsc::Receiver<OrchestratorToPlanet>,
        mpsc::Sender<PlanetToOrchestrator>,
    );
    type ExplChannels = (
        mpsc::Receiver<ExplorerToPlanet>,
        mpsc::Sender<PlanetToExplorer>,
    );

    fn get_test_channels() -> (OrchChannels, ExplChannels) {
        // Channel 1: Orchestrator -> Planet
        let (_tx_orch_in, rx_orch_in) = mpsc::channel::<OrchestratorToPlanet>();
        // Channel 2: Planet -> Orchestrator
        let (tx_orch_out, _rx_orch_out) = mpsc::channel::<PlanetToOrchestrator>();

        // Channel 3: Explorer -> Planet
        let (_tx_expl_in, rx_expl_in) = mpsc::channel::<ExplorerToPlanet>();
        // Channel 4: Planet -> Explorer
        let (tx_expl_out, _rx_expl_out) = mpsc::channel::<PlanetToExplorer>();

        ((rx_orch_in, tx_orch_out), (rx_expl_in, tx_expl_out))
    }

    // --- Unit Tests: Planet State Logic ---

    #[test]
    fn test_planet_state_rocket_construction() {
        let mut state = PlanetState {
            id: 0,
            energy_cells: vec![EnergyCell::new()],
            rocket: None,
            can_have_rocket: true,
        };

        let cell = state.cell_mut(0);
        let sunray = Sunray::new();
        cell.charge(sunray);

        // Build Rocket
        let res = state.build_rocket(0);
        assert!(res.is_ok());
        assert!(state.has_rocket());
        assert!(!state.cell(0).is_charged());

        // Take Rocket
        let rocket = state.take_rocket();
        assert!(rocket.is_some());
        assert!(!state.has_rocket());
    }

    #[test]
    fn test_planet_state_type_b_no_rocket() {
        let mut state = PlanetState {
            id: 0,
            energy_cells: vec![EnergyCell::new()],
            rocket: None,
            can_have_rocket: false, // Type B
        };

        let cell = state.cell_mut(0);
        cell.charge(Sunray::new());

        let res = state.build_rocket(0);
        assert!(res.is_err(), "Type B should not be able to build rockets");
    }

    // --- Integration Tests: Constructor ---

    #[test]
    fn test_planet_construction_constraints() {
        // 1. Valid Construction
        let (orch_ch, expl_ch) = get_test_channels();
        let valid_gen = vec![BasicResourceType::Oxygen];

        let valid_planet = Planet::new(
            1,
            PlanetType::A,
            MockAI::new(),
            valid_gen,
            vec![],
            orch_ch,
            expl_ch,
        );
        assert!(valid_planet.is_ok());

        // 2. Invalid: Empty Gen Rules
        let (orch_ch, expl_ch) = get_test_channels();
        let invalid_empty = Planet::new(
            1,
            PlanetType::A,
            MockAI::new(),
            vec![], // Error
            vec![],
            orch_ch,
            expl_ch,
        );
        assert!(invalid_empty.is_err());

        // 3. Invalid: Too Many Gen Rules for Type A
        let (orch_ch, expl_ch) = get_test_channels();
        let invalid_gen = Planet::new(
            1,
            PlanetType::A,
            MockAI::new(),
            vec![BasicResourceType::Oxygen, BasicResourceType::Hydrogen], // Error for Type A
            vec![],
            orch_ch,
            expl_ch,
        );
        assert!(invalid_gen.is_err());
    }

    // --- Integration Tests: Loop ---

    #[test]
    fn test_planet_run_loop_survival() {
        // 1. Orch -> Planet
        let (tx_to_planet_orch, rx_from_orch) = mpsc::channel::<OrchestratorToPlanet>();
        // 2. Planet -> Orch
        let (tx_from_planet_orch, rx_to_orch) = mpsc::channel::<PlanetToOrchestrator>();

        // 3. Expl -> Planet
        let (_tx_to_planet_expl, rx_from_expl) = mpsc::channel::<ExplorerToPlanet>();
        // 4. Planet -> Expl
        let (tx_from_planet_expl, _rx_from_planet) = mpsc::channel::<PlanetToExplorer>();

        // Build Planet
        let mut planet = Planet::new(
            100,
            PlanetType::A,
            MockAI::new(),
            vec![BasicResourceType::Oxygen],
            vec![],
            (rx_from_orch, tx_from_planet_orch),
            (rx_from_expl, tx_from_planet_expl),
        )
        .expect("Failed to create planet");

        // Spawn thread
        let handle = thread::spawn(move || {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _ = planet.run();
            }));
        });

        // 1. Start AI
        tx_to_planet_orch
            .send(OrchestratorToPlanet::StartPlanetAI(StartPlanetAiMsg))
            .unwrap();
        thread::sleep(Duration::from_millis(50));

        // 2. Send Sunray
        tx_to_planet_orch
            .send(OrchestratorToPlanet::Sunray(Sunray::new()))
            .unwrap();

        // Expect Ack
        if let Ok(PlanetToOrchestrator::SunrayAck { planet_id, .. }) =
            rx_to_orch.recv_timeout(Duration::from_millis(200))
        {
            assert_eq!(planet_id, 100);
        } else {
            panic!("Did not receive SunrayAck");
        }

        // 3. Send Asteroid (AI should build rocket using the charged cell)
        tx_to_planet_orch
            .send(OrchestratorToPlanet::Asteroid(Asteroid::new()))
            .unwrap();

        // 4. Expect Survival (Ack with Some(Rocket))
        match rx_to_orch.recv_timeout(Duration::from_millis(200)) {
            Ok(PlanetToOrchestrator::AsteroidAck {
                planet_id, rocket, ..
            }) => {
                assert_eq!(planet_id, 100);
                assert!(rocket.is_some(), "Planet failed to build rocket!");
            }
            Ok(_) => panic!("Wrong message type"),
            Err(_) => panic!("Timeout waiting for AsteroidAck"),
        }

        // 5. Stop
        tx_to_planet_orch
            .send(OrchestratorToPlanet::StopPlanetAI(StopPlanetAiMsg))
            .unwrap();

        drop(tx_to_planet_orch);
        let _ = handle.join();
    }

    #[test]
    fn test_resource_creation() {
        let (orch_ch, expl_ch) = get_test_channels();
        let gen_rules = vec![BasicResourceType::Oxygen, BasicResourceType::Hydrogen];
        let comb_rules = vec![ComplexResourceType::Water];
        let mut planet = Planet::new(
            0,
            PlanetType::B,
            MockAI::new(),
            gen_rules,
            comb_rules,
            orch_ch,
            expl_ch,
        )
        .unwrap();

        // aliases for planet internals
        let state = &mut planet.state;
        let generator = &planet.generator;
        let combinator = &planet.combinator;

        // gen oxygen
        let cell = state.cell_mut(0);
        cell.charge(Sunray::new());

        let oxygen = generator.make_oxygen(cell);
        assert!(oxygen.is_ok());
        let oxygen = oxygen.unwrap();

        // gen hydrogen
        let cell = state.cell_mut(0);
        cell.charge(Sunray::new());

        let hydrogen = generator.make_hydrogen(cell);
        assert!(hydrogen.is_ok());
        let hydrogen = hydrogen.unwrap();

        // combine the two elements into water
        let cell = state.cell_mut(0);
        cell.charge(Sunray::new());

        let diamond = combinator.make_water(hydrogen, oxygen, cell);
        assert!(diamond.is_ok());
    }
}
