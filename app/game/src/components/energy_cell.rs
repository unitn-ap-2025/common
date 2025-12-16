//! `EnergyCell` module
//!
//! This module defines the [`EnergyCell`] type, a simple component that can store
//! energy after being exposed to a [Sunray]. It supports charging, discharging,
//! and checking whether the cell currently holds energy.

use crate::components::sunray::Sunray;
use std::fmt::{Debug, Formatter};

/// Represents an energy storage cell that can be charged by receiving a [Sunray].
#[allow(dead_code)]
pub struct EnergyCell {
    /// Indicates whether the cell currently holds energy.
    charge: bool,
}

impl Default for EnergyCell {
    /// Creates a new uncharged `EnergyCell`.
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for EnergyCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Energy cell charge: {}", self.charge)
    }
}

#[allow(dead_code)]
impl EnergyCell {
    /// Constructs a new `EnergyCell` that starts uncharged.
    #[must_use]
    pub fn new() -> Self {
        Self { charge: false }
    }

    /// Charges the cell using a [Sunray].
    ///
    /// If the cell is already charged, the sunray has no additional effect.
    ///
    /// # Parameters
    ///
    /// - `_sunray`: The sunray that charges the cell.
    pub fn charge(&mut self, _sunray: Sunray) {
        if !self.charge {
            self.charge = true;
        }
        // If already charged, nothing happens and the Sunray is wasted.
    }

    /// Attempts to discharge the cell.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the cell was charged and is now discharged.
    /// - `Err(String)` if the cell was already empty.
    pub fn discharge(&mut self) -> Result<(), String> {
        if self.charge {
            self.charge = false;
            Ok(())
        } else {
            Err("EnergyCell not charged!".to_string())
        }
    }

    /// Returns `true` if the cell currently holds a charge, false otherwise
    #[must_use]
    pub fn is_charged(&self) -> bool {
        self.charge
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the [EnergyCell] type.
    //!
    //! These tests validate the expected behavior of construction, charging,
    //! discharging, and error handling.

    use super::*;
    use crate::components::sunray::Sunray;

    /// Verifies that a newly constructed cell begins uncharged.
    #[test]
    fn constructor_creates_uncharged_cell() {
        let cell = EnergyCell::new();
        assert!(!cell.is_charged(), "New cells should start uncharged");
    }

    /// Ensures that calling `charge()` sets the cell to a charged state.
    #[test]
    fn charging_sets_state_to_charged() {
        let mut cell = EnergyCell::new();
        cell.charge(Sunray::new());

        assert!(
            cell.is_charged(),
            "Cell should become charged after calling charge()"
        );
    }

    /// Confirms that discharging a charged cell succeeds and clears the charge state.
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

    /// Ensures discharging an empty cell returns an error.
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
