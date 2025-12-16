//! # Planet module
//! This module provides common definitions for planets and their associated types
//! that need be used by a group to construct its own planet.
//! The [Planet] struct is the **main component**: an instance of it represents the
//! actual planet and contains all the logic and state (see [`PlanetState`]) needed to work as one, in fact
//! this is what the orchestrator will interact with.
//!
//! You can instantiate a new planet by calling the [`Planet::new`] constructor method and passing
//! valid construction parameters to it (look into its documentation to learn more).
//!
//! One of the construction parameters is a group-defined struct that implements the [`PlanetAI`] trait,
//! which defines several methods for handling messages coming from the orchestrator and the explorers. This is
//! the core of each group's planet implementation, as it defines the planet *behavior*, that is
//! how a planet "reacts" to the possible events or requests.
//!
//! ## Examples
//! Intended usage (for planet definition, by groups):
//!
//! ```
//! use crossbeam_channel::{Sender, Receiver};
//! use common_game::components::planet::{Planet, PlanetAI, PlanetState, PlanetType, DummyPlanetState};
//! use common_game::components::resource::{Combinator, Generator};
//! use common_game::components::rocket::Rocket;
//! use common_game::components::sunray::Sunray;
//! use common_game::protocols::messages;
//!
//! // Group-defined AI struct
//! struct AI { /* your AI state here */ };
//!
//! impl PlanetAI for AI {
//!     fn handle_sunray(
//!         &mut self,
//!         state: &mut PlanetState,
//!         generator: &Generator,
//!         combinator: &Combinator,
//!         sunray: Sunray
//!     ) {
//!         // your handler code here...
//!     }
//!
//!     fn handle_internal_state_req(
//!         &mut self,
//!         state: &mut PlanetState,
//!         generator: &Generator,
//!         combinator: &Combinator
//!     ) -> DummyPlanetState {
//!         // your handler code here...
//!         state.to_dummy()
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
//! }
//!
//! // This is the group's "export" function. It will be called by
//! // the orchestrator to spawn your planet.
//! pub fn create_planet(
//!     rx_orchestrator: Receiver<messages::OrchestratorToPlanet>,
//!     tx_orchestrator: Sender<messages::PlanetToOrchestrator>,
//!     rx_explorer: Receiver<messages::ExplorerToPlanet>,
//! ) -> Planet {
//!     let id = 1;
//!     let ai = AI {};
//!     let gen_rules = vec![/* your recipes */];
//!     let comb_rules = vec![/* your recipes */];
//!
//!     // Construct the planet and return it
//!     Planet::new(
//!         id,
//!         PlanetType::A,
//!         Box::new(ai),
//!         gen_rules,
//!         comb_rules,
//!         (rx_orchestrator, tx_orchestrator),
//!         rx_explorer,
//!     ).unwrap() // Don't call .unwrap()! You should do error checking instead.
//! }
//! ```

use crate::components::energy_cell::EnergyCell;
use crate::components::resource::{BasicResourceType, Combinator, ComplexResourceType, Generator};
use crate::components::rocket::Rocket;
use crate::components::sunray::Sunray;
use crate::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
};
use crossbeam_channel::{Receiver, Sender, select_biased};
use std::collections::HashMap;
use std::slice::{Iter, IterMut};

/// The trait that defines the behavior of a planet.
///
/// Structs implementing this trait are intended to be passed to the
/// [Planet] constructor, so that the handlers can be invoked by the planet
/// internal logic when certain messages are received on any of the planet channels.
///
/// The handlers can alter the planet state by accessing the
/// `state` parameter, which is passed to the methods as a mutable borrow.
pub trait PlanetAI: Send {
    /// This handler will be invoked when a [`OrchestratorToPlanet::Sunray`]
    /// message is received. The `sunray` parameter is the actual [Sunray] struct
    /// used to charged energy cells.
    fn handle_sunray(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        sunray: Sunray,
    );

    /// This handler will be invoked when a [`OrchestratorToPlanet::Asteroid`]
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

    /// This handler will be invoked when a [`OrchestratorToPlanet::InternalStateRequest`]
    /// message is received.
    ///
    /// # Returns
    /// Should return a [`DummyPlanetState`] instance representing the current state
    /// of the planet.
    fn handle_internal_state_req(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
    ) -> DummyPlanetState;

    /// Handler for **all** messages received by an explorer (receiving
    /// end of the [`ExplorerToPlanet`] channel).
    ///
    /// # Returns
    /// This method can return an optional response to the message, which will
    /// be delivered to the explorer that sent the message.
    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer>;

    /// This method will be invoked when an explorer (identified by the `explorer_id`
    /// parameter) lands on the planet.
    #[allow(unused)]
    fn on_explorer_arrival(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        explorer_id: u32,
    ) {
    }

    /// This method will be invoked when an explorer (identified by the `explorer_id`
    /// parameter) leaves the planet.
    #[allow(unused)]
    fn on_explorer_departure(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        explorer_id: u32,
    ) {
    }

    /// This method will be invoked when a [`OrchestratorToPlanet::StartPlanetAI`]
    /// is received, but **only if** the planet is currently in a *stopped* state.
    ///
    /// Start messages received when planet is already running are **ignored**.
    #[allow(unused)]
    fn on_start(&mut self, state: &PlanetState, generator: &Generator, combinator: &Combinator) {}

    /// This method will be invoked when a [`OrchestratorToPlanet::StopPlanetAI`]
    /// is received, but **only if** the planet is currently in a *running* state.
    ///
    /// Stop messages received when planet is already stopped are **ignored**.
    #[allow(unused)]
    fn on_stop(&mut self, state: &PlanetState, generator: &Generator, combinator: &Combinator) {}
}

/// Contains planet rules constraints (see [`PlanetType`]).
pub struct PlanetConstraints {
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

    /// Returns a tuple with the constraints associated to the planet type,
    /// as described in the project specifications.
    #[must_use] 
    pub fn constraints(&self) -> PlanetConstraints {
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
/// - A vec of [`EnergyCell`].
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
    #[must_use] 
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Indexed getter accessor for the [`EnergyCell`] vec.
    ///
    /// # Returns
    /// An immutable borrow of the *i-th* energy cell.
    ///
    /// # Panics
    /// This method will panic if the index `i` is out of bounds.
    /// Always check the number of energy cells available with [`PlanetState::cells_count`].
    #[must_use] 
    pub fn cell(&self, i: usize) -> &EnergyCell {
        &self.energy_cells[i]
    }

    /// Indexed *mutable* getter accessor for the [`EnergyCell`] vec.
    ///
    /// # Returns
    /// An mutable borrow of the *i-th* energy cell.
    ///
    /// # Panics
    /// This method will panic if the index `i` is out of bounds.
    /// Always check the number of energy cells available with [`PlanetState::cells_count`].
    pub fn cell_mut(&mut self, i: usize) -> &mut EnergyCell {
        &mut self.energy_cells[i]
    }

    /// Returns the number of energy cells owned by
    /// the planet. This is the actual size of the internal
    /// vec containing the cells.
    #[must_use] 
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

    /// Charges the first empty (discharged) cell.
    /// Returns an optional [Sunray] if there's no cell to charge.
    pub fn charge_cell(&mut self, sunray: Sunray) -> Option<Sunray> {
        match self.empty_cell() {
            None => Some(sunray),
            Some((cell, _)) => {
                cell.charge(sunray);
                None
            }
        }
    }

    /// Returns a tuple containing a *mutable* borrow of the first empty (discharged) cell
    /// and its index, or `None` if there isn't any.
    pub fn empty_cell(&mut self) -> Option<(&mut EnergyCell, usize)> {
        let idx = self.energy_cells.iter().position(|cell| !cell.is_charged());
        idx.map(|i| (&mut self.energy_cells[i], i))
    }

    /// Returns a tuple containing a *mutable* borrow of the first full (charged) cell
    /// and its index, or `None` if there isn't any.
    pub fn full_cell(&mut self) -> Option<(&mut EnergyCell, usize)> {
        let idx = self.energy_cells.iter().position(super::energy_cell::EnergyCell::is_charged);
        idx.map(|i| (&mut self.energy_cells[i], i))
    }

    /// Returns `true` if the planet can have a rocket.
    #[must_use] 
    pub fn can_have_rocket(&self) -> bool {
        self.can_have_rocket
    }

    /// Returns `true` if the planet has a rocket built and ready to launch.
    #[must_use] 
    pub fn has_rocket(&self) -> bool {
        self.rocket.is_some()
    }

    /// Takes the rocket out of the planet state (if there is one), leaving
    /// `None` in its place.
    pub fn take_rocket(&mut self) -> Option<Rocket> {
        self.rocket.take()
    }

    /// Constructs a rocket using the *i-th* [`EnergyCell`] of the planet and stores it
    /// inside the planet, taking ownership of it.
    ///
    /// # Panics
    /// This method will panic if the index `i` is out of bounds.
    /// Always check the number of energy cells available with [`PlanetState::cells_count`].
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

    /// Returns a *dummy* clone of this state.
    #[must_use] 
    pub fn to_dummy(&self) -> DummyPlanetState {
        DummyPlanetState {
            energy_cells: self
                .energy_cells
                .iter()
                .map(super::energy_cell::EnergyCell::is_charged)
                .collect(),
            charged_cells_count: self
                .energy_cells
                .iter()
                .filter(|cell| cell.is_charged())
                .count(),
            has_rocket: self.has_rocket(),
        }
    }
}

/// This is a dummy struct containing an overview of the internal state of a planet.
/// Use [`PlanetState::to_dummy`] to construct one.
///
/// Used in [`PlanetToOrchestrator::InternalStateResponse`].
#[derive(Debug, Clone)]
pub struct DummyPlanetState {
    pub energy_cells: Vec<bool>,
    pub charged_cells_count: usize,
    pub has_rocket: bool,
}

/// Main, top-level planet definition. This type is built on top of
/// [`PlanetState`], [`PlanetType`] and [`PlanetAI`], through composition.
///
/// It needs to be constructed by each group as it represents the actual planet
/// and contains the base logic that runs the AI. Also, this is what should be
/// returned to the orchestrator.
///
/// See module-level docs for more general info.
pub struct Planet {
    state: PlanetState,
    planet_type: PlanetType,
    pub ai: Box<dyn PlanetAI>,
    generator: Generator,
    combinator: Combinator,

    from_orchestrator: Receiver<OrchestratorToPlanet>,
    to_orchestrator: Sender<PlanetToOrchestrator>,
    from_explorers: Receiver<ExplorerToPlanet>,
    to_explorers: HashMap<u32, Sender<PlanetToExplorer>>,
}

impl Planet {
    const ORCH_DISCONNECT_ERR: &'static str = "Orchestrator disconnected.";

    /// Constructor for the [Planet] type.
    ///
    /// # Errors
    /// Returns an error if the construction parameters are *invalid* (they violate the `planet_type` constraints).
    ///
    /// # Arguments
    /// - `id` - The identifier to assign to the planet.
    /// - `planet_type` - Type of the planet. Constraints the rules of the planet.
    /// - `ai` - A group-defined struct implementing the [`PlanetAI`] trait.
    /// - `gen_rules` - A vec of [`BasicResourceType`] containing the basic resources the planet will be able to generate.
    /// - `comb_rules` - A vec of [`ComplexResourceType`] containing the complex resources the planet will be able to make.
    /// - `orchestrator_channels` - A pair containing the receiver and sender half
    ///   of the channels [`OrchestratorToPlanet`] and [`PlanetToOrchestrator`].
    /// - `explorers_receiver` - The receiver half of the [`ExplorerToPlanet`] channel
    ///   where all explorers send messages to this planet (when they're visiting it).
    pub fn new(
        id: u32,
        planet_type: PlanetType,
        ai: Box<dyn PlanetAI>,
        gen_rules: Vec<BasicResourceType>,
        comb_rules: Vec<ComplexResourceType>,
        orchestrator_channels: (Receiver<OrchestratorToPlanet>, Sender<PlanetToOrchestrator>),
        explorers_receiver: Receiver<ExplorerToPlanet>,
    ) -> Result<Planet, String> {
        let PlanetConstraints {
            n_energy_cells,
            unbounded_gen_rules,
            can_have_rocket,
            n_comb_rules,
        } = planet_type.constraints();
        let (from_orchestrator, to_orchestrator) = orchestrator_channels;

        if gen_rules.is_empty() {
            Err("gen_rules is empty".to_string())
        } else if !unbounded_gen_rules && gen_rules.len() > 1 {
            Err(format!(
                "Too many generation rules (Planet type {planet_type:?} is limited to 1)"
            ))
        } else if comb_rules.len() > n_comb_rules {
            Err(format!(
                "Too many combination rules (Planet type {planet_type:?} is limited to {n_comb_rules})"
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
                from_explorers: explorers_receiver,
                to_explorers: HashMap::new(),
            })
        }
    }

    /// Starts the planet in a *stopped* state, waiting for a [`OrchestratorToPlanet::StartPlanetAI`] message,
    /// then invokes [`PlanetAI::start`] and runs the main message polling loop.
    /// See [`PlanetAI`] docs to know more about when message handlers are invoked and how the planet reacts
    /// to the different messages.
    ///
    /// This method is *blocking* and should be called by the orchestrator in a separate thread.
    /// It returns with an [Ok] when the planet has been **destroyed** (killed).
    ///
    /// # Errors
    /// If the orchestrator disconnects from the channel, this will return an [Err].
    pub fn run(&mut self) -> Result<(), String> {
        // run the planet stopped by default
        // and wait for a StartPlanetAI message
        let kill = self.wait_for_start()?;
        if kill {
            return Ok(());
        }

        self.ai
            .on_start(&self.state, &self.generator, &self.combinator);

        loop {
            select_biased! {
                // wait for orchestrator message (prioritized operation)
                recv(self.from_orchestrator) -> msg => match msg {
                    Ok(OrchestratorToPlanet::StartPlanetAI) => {}

                    Ok(OrchestratorToPlanet::StopPlanetAI) => {
                        self.to_orchestrator
                            .send(PlanetToOrchestrator::StopPlanetAIResult {
                                planet_id: self.id(),
                            })
                            .map_err(|_| Self::ORCH_DISCONNECT_ERR.to_string())?;

                        self.ai.on_stop(&self.state, &self.generator, &self.combinator);

                        let kill = self.wait_for_start()?; // blocking wait
                        if kill { return Ok(()) }

                        // restart AI
                        self.ai.on_start(&self.state, &self.generator, &self.combinator);
                    }

                    Ok(OrchestratorToPlanet::KillPlanet) => {
                        self.to_orchestrator
                            .send(PlanetToOrchestrator::KillPlanetResult { planet_id: self.id() })
                            .map_err(|_| Self::ORCH_DISCONNECT_ERR.to_string())?;

                        return Ok(())
                    }

                    Ok(OrchestratorToPlanet::Sunray(sunray)) => {
                        self.ai.handle_sunray(
                            &mut self.state,
                            &self.generator,
                            &self.combinator,
                            sunray
                        );

                        self.to_orchestrator
                            .send(PlanetToOrchestrator::SunrayAck { planet_id: self.id() })
                            .map_err(|_| Self::ORCH_DISCONNECT_ERR.to_string())?;
                    }

                    Ok(OrchestratorToPlanet::Asteroid(_)) => {
                        let rocket =
                            self.ai
                                .handle_asteroid(&mut self.state, &self.generator, &self.combinator);

                        self.to_orchestrator
                            .send(PlanetToOrchestrator::AsteroidAck {
                                planet_id: self.id(),
                                rocket
                            })
                            .map_err(|_| Self::ORCH_DISCONNECT_ERR.to_string())?;
                    }

                    Ok(OrchestratorToPlanet::IncomingExplorerRequest {
                        explorer_id,
                        new_mpsc_sender,
                    }) => {
                        self.to_explorers.insert(explorer_id, new_mpsc_sender); // add new explorer channel
                        self.ai.on_explorer_arrival(&mut self.state, &self.generator, &self.combinator, explorer_id);

                        // send ack back to orchestrator
                        self.to_orchestrator
                            .send(PlanetToOrchestrator::IncomingExplorerResponse {
                                planet_id: self.id(),
                                explorer_id,
                                res: Ok(()),
                            })
                            .map_err(|_| Self::ORCH_DISCONNECT_ERR.to_string())?;
                    }

                    Ok(OrchestratorToPlanet::OutgoingExplorerRequest { explorer_id }) => {
                        self.to_explorers.remove(&explorer_id); // remove outgoing explorer channel
                        self.ai.on_explorer_departure(&mut self.state, &self.generator, &self.combinator, explorer_id);

                        // send ack back to orchestrator
                        self.to_orchestrator
                            .send(PlanetToOrchestrator::OutgoingExplorerResponse {
                                planet_id: self.id(),
                                explorer_id,
                                res: Ok(()),
                            })
                            .map_err(|_| Self::ORCH_DISCONNECT_ERR.to_string())?;
                    }

                    // default case: relay to generic handler
                    Ok(OrchestratorToPlanet::InternalStateRequest) => {
                        let dummy_state = self.ai.handle_internal_state_req(
                            &mut self.state,
                            &self.generator,
                            &self.combinator,
                        );

                        self.to_orchestrator.send(PlanetToOrchestrator::InternalStateResponse {
                            planet_id: self.id(),
                            planet_state: dummy_state,
                        })
                        .map_err(|_| Self::ORCH_DISCONNECT_ERR.to_string())?;
                    }

                    Err(_) => {
                        return Err(Self::ORCH_DISCONNECT_ERR.to_string())
                    }
                },

                // wait for explorer message (ignore disconnections)
                recv(self.from_explorers) -> msg => if let Ok(msg) = msg {
                    let explorer_id = msg.explorer_id();

                    // if requesting explorer is currently
                    // on the planet respond to it
                    if let Some(to_explorer) = self.to_explorers.get(&explorer_id)
                        && let Some(response) = self.ai.handle_explorer_msg(
                            &mut self.state,
                            &self.generator,
                            &self.combinator,
                            msg,
                        )
                    {
                        to_explorer
                            .send(response)
                            .map_err(|_| format!("Explorer {explorer_id} disconnected."))?;
                    }
                }
            }
        }
    }

    // private helper function that blocks until
    // a StartPlanetAI message is received
    fn wait_for_start(&self) -> Result<bool, String> {
        loop {
            select_biased! {
                // orch messages
                recv(self.from_orchestrator) -> msg => match msg {
                    // if `Start` is received, return false
                    Ok(OrchestratorToPlanet::StartPlanetAI) => {
                        self.to_orchestrator
                            .send(PlanetToOrchestrator::StartPlanetAIResult {
                                planet_id: self.id(),
                            })
                            .map_err(|_| Self::ORCH_DISCONNECT_ERR.to_string())?;

                        return Ok(false);
                    }
                    // if `Kill` is received, return true
                    Ok(OrchestratorToPlanet::KillPlanet) => {
                        self.to_orchestrator
                            .send(PlanetToOrchestrator::KillPlanetResult { planet_id: self.id() })
                            .map_err(|_| Self::ORCH_DISCONNECT_ERR.to_string())?;

                        return Ok(true)
                    }
                    // every other message we respond with `Stopped`
                    Ok(_) => {
                        self.to_orchestrator
                            .send(PlanetToOrchestrator::Stopped {
                                planet_id: self.id(),
                            })
                            .map_err(|_| Self::ORCH_DISCONNECT_ERR.to_string())?;
                    }

                    Err(_) => return Err(Self::ORCH_DISCONNECT_ERR.to_string()),
                },

                // explorers messages
                recv(self.from_explorers) -> msg => if let Ok(msg) = msg &&
                    let Some(to_explorer) = self.to_explorers.get(&msg.explorer_id())
                {
                    let _ = to_explorer.send(PlanetToExplorer::Stopped);
                }
            }
        }
    }

    /// Returns the planet id.
    #[must_use] 
    pub fn id(&self) -> u32 {
        self.state.id
    }

    /// Returns the planet type.
    #[must_use] 
    pub fn planet_type(&self) -> PlanetType {
        self.planet_type
    }

    /// Returns an immutable borrow of planet's internal state.
    #[must_use] 
    pub fn state(&self) -> &PlanetState {
        &self.state
    }

    /// Returns an immutable borrow of the planet generator.
    #[must_use] 
    pub fn generator(&self) -> &Generator {
        &self.generator
    }

    /// Returns an immutable borrow of the planet combinator.
    #[must_use] 
    pub fn combinator(&self) -> &Combinator {
        &self.combinator
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam_channel::{Receiver, Sender, unbounded};
    use std::thread;
    use std::time::Duration;

    use crate::components::asteroid::Asteroid;
    use crate::components::energy_cell::EnergyCell;
    use crate::components::resource::{BasicResourceType, Combinator, Generator};
    use crate::components::rocket::Rocket;
    use crate::components::sunray::Sunray;
    use crate::protocols::messages::{
        ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
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
        fn handle_sunray(
            &mut self,
            state: &mut PlanetState,
            _generator: &Generator,
            _combinator: &Combinator,
            sunray: Sunray,
        ) {
            self.sunray_count += 1;

            if let Some(cell) = state.cells_iter_mut().next() {
                cell.charge(sunray);
            }
        }

        fn handle_asteroid(
            &mut self,
            state: &mut PlanetState,
            _generator: &Generator,
            _combinator: &Combinator,
        ) -> Option<Rocket> {
            match state.full_cell() {
                None => None,
                Some((_cell, i)) => {
                    // assert!(cell.is_charged());
                    let _ = state.build_rocket(i);
                    state.take_rocket()
                }
            }
        }

        fn handle_internal_state_req(
            &mut self,
            state: &mut PlanetState,
            _generator: &Generator,
            _combinator: &Combinator,
        ) -> DummyPlanetState {
            state.to_dummy()
        }

        fn handle_explorer_msg(
            &mut self,
            _state: &mut PlanetState,
            _generator: &Generator,
            _combinator: &Combinator,
            msg: ExplorerToPlanet,
        ) -> Option<PlanetToExplorer> {
            match msg {
                ExplorerToPlanet::AvailableEnergyCellRequest { .. } => {
                    Some(PlanetToExplorer::AvailableEnergyCellResponse { available_cells: 5 })
                }
                _ => None,
            }
        }

        fn on_start(
            &mut self,
            _state: &PlanetState,
            _generator: &Generator,
            _combinator: &Combinator,
        ) {
            self.start_called = true;
        }

        fn on_stop(
            &mut self,
            _state: &PlanetState,
            _generator: &Generator,
            _combinator: &Combinator,
        ) {
            self.stop_called = true;
        }
    }

    // --- Helper for creating dummy channels ---
    // Returns the halves required by Planet::new
    type PlanetOrchHalfChannels = (Receiver<OrchestratorToPlanet>, Sender<PlanetToOrchestrator>);

    type PlanetExplHalfChannels = (Receiver<ExplorerToPlanet>, Sender<PlanetToExplorer>);

    type OrchPlanetHalfChannels = (Sender<OrchestratorToPlanet>, Receiver<PlanetToOrchestrator>);

    type ExplPlanetHalfChannels = (Sender<ExplorerToPlanet>, Receiver<PlanetToExplorer>);

    fn get_test_channels() -> (
        PlanetOrchHalfChannels,
        PlanetExplHalfChannels,
        OrchPlanetHalfChannels,
        ExplPlanetHalfChannels,
    ) {
        // Channel 1: Orchestrator -> Planet
        let (tx_orch_in, rx_orch_in) = unbounded::<OrchestratorToPlanet>();
        // Channel 2: Planet -> Orchestrator
        let (tx_orch_out, rx_orch_out) = unbounded::<PlanetToOrchestrator>();

        // Channel 3: Explorer -> Planet
        let (tx_expl_in, rx_expl_in) = unbounded::<ExplorerToPlanet>();
        // Channel 4: Planet -> Explorer
        let (tx_expl_out, rx_expl_out) = unbounded::<PlanetToExplorer>();

        (
            (rx_orch_in, tx_orch_out),
            (rx_expl_in, tx_expl_out),
            (tx_orch_in, rx_orch_out),
            (tx_expl_in, rx_expl_out),
        )
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
        let (orch_ch, expl_ch, _, _) = get_test_channels();
        let valid_gen = vec![BasicResourceType::Oxygen];

        let valid_planet = Planet::new(
            1,
            PlanetType::A,
            Box::new(MockAI::new()),
            valid_gen,
            vec![],
            orch_ch,
            expl_ch.0,
        );
        assert!(valid_planet.is_ok());

        // 2. Invalid: Empty Gen Rules
        let (orch_ch, expl_ch, _, _) = get_test_channels();
        let invalid_empty = Planet::new(
            1,
            PlanetType::A,
            Box::new(MockAI::new()),
            vec![], // Error
            vec![],
            orch_ch,
            expl_ch.0,
        );
        assert!(invalid_empty.is_err());

        // 3. Invalid: Too Many Gen Rules for Type A
        let (orch_ch, expl_ch, _, _) = get_test_channels();
        let invalid_gen = Planet::new(
            1,
            PlanetType::A,
            Box::new(MockAI::new()),
            vec![BasicResourceType::Oxygen, BasicResourceType::Hydrogen], // Error for Type A
            vec![],
            orch_ch,
            expl_ch.0,
        );
        assert!(invalid_gen.is_err());
    }

    // --- Integration Tests: Loop ---

    #[test]
    fn test_planet_run_loop_survival() {
        let (planet_orch_ch, planet_expl_ch, orch_planet_ch, _) = get_test_channels();

        let (rx_from_orch, tx_from_planet_orch) = planet_orch_ch;
        let (rx_from_expl, _) = planet_expl_ch;
        let (tx_to_planet_orch, rx_to_orch) = orch_planet_ch;

        // Build Planet
        let mut planet = Planet::new(
            100,
            PlanetType::A,
            Box::new(MockAI::new()),
            vec![BasicResourceType::Oxygen],
            vec![],
            (rx_from_orch, tx_from_planet_orch),
            rx_from_expl,
        )
        .expect("Failed to create planet");

        // Spawn thread
        let handle = thread::spawn(move || {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let res = planet.run();
                match res {
                    Ok(_) => {}
                    Err(err) => {
                        dbg!(err);
                    }
                }
            }));
        });

        // 1. Start AI
        tx_to_planet_orch
            .send(OrchestratorToPlanet::StartPlanetAI)
            .unwrap();
        match rx_to_orch.recv_timeout(Duration::from_millis(50)) {
            Ok(PlanetToOrchestrator::StartPlanetAIResult { .. }) => {}
            _ => panic!("Planet sent incorrect response"),
        }
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
            .send(OrchestratorToPlanet::StopPlanetAI)
            .unwrap();
        match rx_to_orch.recv_timeout(Duration::from_millis(200)) {
            Ok(PlanetToOrchestrator::StopPlanetAIResult { .. }) => {}
            _ => panic!("Planet sent incorrect response"),
        }

        // 6. Try to send a request while stopped
        tx_to_planet_orch
            .send(OrchestratorToPlanet::InternalStateRequest)
            .unwrap();
        match rx_to_orch.recv_timeout(Duration::from_millis(200)) {
            Ok(PlanetToOrchestrator::Stopped { .. }) => {}
            _ => panic!("Planet sent incorrect response"),
        }

        // 7. Kill planet while stopped
        tx_to_planet_orch
            .send(OrchestratorToPlanet::KillPlanet)
            .unwrap();
        match rx_to_orch.recv_timeout(Duration::from_millis(200)) {
            Ok(PlanetToOrchestrator::KillPlanetResult { .. }) => {}
            _ => panic!("Planet sent incorrect response"),
        }

        // should return immediately
        assert!(handle.join().is_ok(), "Planet thread exited with an error");
    }

    #[test]
    fn test_resource_creation() {
        let (orch_ch, expl_ch, _, _) = get_test_channels();
        let gen_rules = vec![BasicResourceType::Oxygen, BasicResourceType::Hydrogen];
        let comb_rules = vec![ComplexResourceType::Water];
        let mut planet = Planet::new(
            0,
            PlanetType::B,
            Box::new(MockAI::new()),
            gen_rules,
            comb_rules,
            orch_ch,
            expl_ch.0,
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

        // try to gen resource not contained in the planet recipes
        let carbon = generator.make_carbon(cell);
        assert!(carbon.is_err());
    }

    #[test]
    fn test_explorer_comms() {
        // 1. Setup Channels using the new helper
        let (
            planet_orch_channels,
            planet_expl_channels,
            (orch_tx, orch_rx),
            (expl_tx_global, _expl_rx_global),
        ) = get_test_channels();

        // 2. Setup Planet
        // Note: Planet::new only takes the Receiver half for explorers,
        // so we extract it from the tuple. The Sender half in the tuple is unused
        // by the planet itself (since it uses dynamic senders), but kept for type consistency.
        let (planet_expl_rx, _) = planet_expl_channels;

        let mut planet = Planet::new(
            1,
            PlanetType::A,
            Box::new(MockAI::new()),
            vec![BasicResourceType::Oxygen],
            vec![],
            planet_orch_channels,
            planet_expl_rx,
        )
        .expect("Failed to create planet");

        // Spawn planet thread
        let handle = thread::spawn(move || {
            let res = planet.run();
            match res {
                Ok(_) => {}
                Err(err) => {
                    dbg!(err);
                }
            }
        });

        // 3. Start Planet
        orch_tx.send(OrchestratorToPlanet::StartPlanetAI).unwrap();
        match orch_rx.recv_timeout(Duration::from_millis(50)) {
            Ok(PlanetToOrchestrator::StartPlanetAIResult { .. }) => {}
            _ => panic!("Planet sent incorrect response"),
        }
        thread::sleep(Duration::from_millis(50));

        // 4. Setup Local Explorer Channels (Simulating Explorer 101)
        // We create a dedicated channel for this specific explorer interaction
        let explorer_id = 101;
        let (expl_tx_local, expl_rx_local) = unbounded::<PlanetToExplorer>();

        // 5. Send IncomingExplorerRequest (Orchestrator -> Planet)
        orch_tx
            .send(OrchestratorToPlanet::IncomingExplorerRequest {
                explorer_id,
                new_mpsc_sender: expl_tx_local,
            })
            .unwrap();

        // 6. Verify Ack from Planet
        match orch_rx.recv_timeout(Duration::from_millis(200)) {
            Ok(PlanetToOrchestrator::IncomingExplorerResponse { planet_id, res, .. }) => {
                assert_eq!(planet_id, 1);
                assert!(res.is_ok());
            }
            _ => panic!("Expected IncomingExplorerResponse"),
        }

        // 7. Test Interaction (Explorer -> Planet -> Explorer)
        // Explorer sends a request using the GLOBAL channel, but includes its ID
        expl_tx_global
            .send(ExplorerToPlanet::AvailableEnergyCellRequest { explorer_id })
            .unwrap();

        // Verify Explorer receives response on the LOCAL channel
        match expl_rx_local.recv_timeout(Duration::from_millis(200)) {
            Ok(PlanetToExplorer::AvailableEnergyCellResponse { available_cells }) => {
                assert_eq!(available_cells, 5);
            }
            _ => panic!("Expected AvailableEnergyCellResponse"),
        }

        // Stop Planet AI
        orch_tx.send(OrchestratorToPlanet::StopPlanetAI).unwrap();
        match orch_rx.recv_timeout(Duration::from_millis(200)) {
            Ok(PlanetToOrchestrator::StopPlanetAIResult { .. }) => {}
            _ => panic!("Planet sent incorrect response"),
        }

        // Try to send request from explorer to stopped planet
        expl_tx_global
            .send(ExplorerToPlanet::AvailableEnergyCellRequest { explorer_id })
            .unwrap();
        match expl_rx_local.recv_timeout(Duration::from_millis(200)) {
            Ok(PlanetToExplorer::Stopped) => {}
            _ => panic!("Planet sent incorrect response"),
        }

        // Restart planet AI
        orch_tx.send(OrchestratorToPlanet::StartPlanetAI).unwrap();
        match orch_rx.recv_timeout(Duration::from_millis(200)) {
            Ok(PlanetToOrchestrator::StartPlanetAIResult { .. }) => {}
            _ => panic!("Planet sent incorrect response"),
        }

        // 8. Send OutgoingExplorerRequest (Orchestrator -> Planet)
        orch_tx
            .send(OrchestratorToPlanet::OutgoingExplorerRequest { explorer_id })
            .unwrap();

        // 9. Verify Ack from Planet
        match orch_rx.recv_timeout(Duration::from_millis(200)) {
            Ok(PlanetToOrchestrator::OutgoingExplorerResponse { planet_id, res, .. }) => {
                assert_eq!(planet_id, 1);
                assert!(res.is_ok());
            }
            _ => panic!("Expected OutgoingExplorerResponse"),
        }

        // 10. Verify Isolation
        // Explorer sends another request
        expl_tx_global
            .send(ExplorerToPlanet::AvailableEnergyCellRequest { explorer_id })
            .unwrap();

        // We expect NO response on expl_rx_local
        let result = expl_rx_local.recv_timeout(Duration::from_millis(200));
        assert!(
            result.is_err(),
            "Planet responded to explorer after it left!"
        );

        // 11. Cleanup
        drop(orch_tx);
        let _ = handle.join();
    }
}
