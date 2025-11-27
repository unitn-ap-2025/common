use crate::components::sunray::Sunray;
#[allow(dead_code)]
pub struct EnergyCell {
    charge: bool,
}

impl Default for EnergyCell {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl EnergyCell {
    pub fn new() -> Self {
        Self { charge: false }
    }

    pub fn charge(&mut self, _sunray: Sunray) {
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
