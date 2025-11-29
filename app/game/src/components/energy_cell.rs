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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::sunray::Sunray;

    #[test]
    fn constructor_creates_uncharged_cell() {
        let cell = EnergyCell::new();
        assert!(!cell.is_charged(), "New cells should start uncharged");
    }

    #[test]
    fn charging_sets_state_to_charged() {
        let mut cell = EnergyCell::new();
        cell.charge(Sunray::new());

        assert!(
            cell.is_charged(),
            "Cell should become charged after calling charge()"
        );
    }

    #[test]
    fn discharge_works_when_charged() {
        let mut cell = EnergyCell::new();
        cell.charge(Sunray::new());

        let result = cell.discharge();
        assert!(
            result.is_ok(),
            "Discharging a charged cell should return Ok"
        );
        assert!(
            !cell.is_charged(),
            "Cell should no longer be charged after discharge()"
        );
    }

    #[test]
    fn discharge_fails_when_empty() {
        let mut cell = EnergyCell::new();
        let result = cell.discharge();

        assert!(
            result.is_err(),
            "Discharging an empty cell should return Err"
        );
        assert_eq!(result.unwrap_err(), "EnergyCell not charged!");
    }
}
