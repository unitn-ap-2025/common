use crate::components::sunray::Sunray;
pub struct EnergyCell {
    charge: bool,
}

impl EnergyCell {
    pub fn new() -> Self {
        Self { charge: false }
    }

    pub fn charge(&mut self, sunray: Sunray) {
        if !self.charge {
            self.charge = true;
        }
        // if the cell is already charged nothing happens, the Sunray is wasted
    }

    pub fn discharge(&mut self) -> Result<(), String> {
        if self.charge {
            self.charge = false;
            Ok(())
        } else {
            Err("EnergyCell not charged!".to_string())
        }
    }

    pub fn is_charged(&self) -> bool {
        self.charge
    }
}
