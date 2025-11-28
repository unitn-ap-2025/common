use std::sync::mpsc;

use crate::components::energy_cell::EnergyCell;
use crate::components::resource::{BasicResourceType, Combinator, ComplexResourceType, Generator};
use crate::components::rocket::Rocket;
use crate::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
};

pub trait PlanetAI {
    fn handle_orchestrator_msg(
        &mut self,
        state: &mut PlanetState,
        msg: OrchestratorToPlanet,
    ) -> Option<PlanetToOrchestrator>;
    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer>;
    fn handle_asteroid(&mut self, state: &mut PlanetState) -> Option<Rocket>;
    fn start(&mut self, state: &PlanetState);
    fn stop(&mut self);
}

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

    pub fn values(&self) -> (usize, bool, bool, usize) {
        match self {
            PlanetType::A => (Self::N_ENERGY_CELLS, false, true, 0),
            PlanetType::B => (1, true, false, 1),
            PlanetType::C => (1, false, true, Self::N_RESOURCE_COMB_RULES),
            PlanetType::D => (Self::N_ENERGY_CELLS, true, false, 0),
        }
    }
}

pub struct PlanetState {
    id: u32,
    energy_cells: Vec<EnergyCell>,
    rocket: Option<Rocket>,
    pub generator: Generator,
    pub combinator: Combinator,
    has_rocket: bool,
}

impl PlanetState {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn cell(&self, i: usize) -> &EnergyCell {
        &self.energy_cells[i]
    }

    pub fn cell_mut(&mut self, i: usize) -> &mut EnergyCell {
        &mut self.energy_cells[i]
    }

    pub fn has_rocket(&self) -> bool {
        self.rocket.is_some()
    }

    pub fn take_rocket(&mut self) -> Option<Rocket> {
        self.rocket.take()
    }

    pub fn build_rocket(&mut self, energy_cell: &mut EnergyCell) -> Result<(), String> {
        if self.has_rocket {
            Rocket::new(energy_cell).map(|rocket| {
                self.rocket = Some(rocket);
            })
        } else {
            Err("This planet type can't build rockets.".to_string())
        }
    }
}

pub struct Planet<T: PlanetAI> {
    state: PlanetState,
    planet_type: PlanetType,
    pub ai: T,

    from_orchestrator: mpsc::Receiver<OrchestratorToPlanet>,
    to_orchestrator: mpsc::Sender<PlanetToOrchestrator>,
    pub from_explorer: mpsc::Receiver<ExplorerToPlanet>,
    pub to_explorer: mpsc::Sender<PlanetToExplorer>,
}

impl<T: PlanetAI> Planet<T> {
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
        let (n_energy_cells, gen_rules_unbounded, has_rocket, n_comb_rules) = planet_type.values();
        let (from_orchestrator, to_orchestrator) = orchestrator_channels;
        let (from_explorer, to_explorer) = explorer_channels;

        if gen_rules.is_empty() {
            Err("gen_rules is empty".to_string())
        } else if !gen_rules_unbounded && gen_rules.len() > 1 {
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
                    has_rocket,
                    rocket: None,
                    generator,
                    combinator,
                },
                planet_type,
                ai,
                from_orchestrator,
                to_orchestrator,
                from_explorer,
                to_explorer,
            })
        }
    }

    pub fn start(&mut self) {
        self.ai.start(&self.state);

        // maybe spawn a thread for async event handling ?
        loop {
            // TODO: disconnection error handling

            // orchestrator incoming message polling
            match self.from_orchestrator.try_recv() {
                // TODO: do something with the StopPlanetAI message content
                Ok(OrchestratorToPlanet::StopPlanetAI(_)) => {
                    self.ai.stop();
                    self.wait_for_start(); // blocking wait

                    // restart AI
                    self.ai.start(&self.state)
                }
                Ok(OrchestratorToPlanet::Asteroid(_)) => {
                    let rocket = self.ai.handle_asteroid(&mut self.state);
                    self.to_orchestrator
                        .send(PlanetToOrchestrator::AsteroidAck {
                            planet_id: self.state.id(),
                            rocket,
                        })
                        .unwrap()
                }
                Ok(msg) => {
                    if let Some(response) = self.ai.handle_orchestrator_msg(&mut self.state, msg) {
                        self.to_orchestrator.send(response).unwrap()
                    }
                }

                Err(mpsc::TryRecvError::Disconnected) => {
                    panic!("Orchestrator disconnected")
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }

            // explorer incoming message polling
            match self.from_explorer.try_recv() {
                Ok(msg) => {
                    if let Some(response) = self.ai.handle_explorer_msg(&mut self.state, msg) {
                        self.to_explorer.send(response).unwrap()
                    }
                }

                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Explorer disconnected")
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
    }

    pub fn planet_type(&self) -> PlanetType {
        self.planet_type
    }

    pub fn state(&self) -> &PlanetState {
        &self.state
    }

    fn wait_for_start(&self) {
        loop {
            let msg = self.from_orchestrator.recv().unwrap();
            // TODO: do something with the StartPlanetAI message content
            if let OrchestratorToPlanet::StartPlanetAI(_) = msg {
                break;
            }
        }
    }
}
