use crate::components::energy_cell::EnergyCell;
/// Represents the rocket in the game, used by the planet.
#[allow(dead_code)]
pub struct Rocket {
    _private: (),
}

#[allow(dead_code)]
impl Rocket {
    /// Creates a new instance of [Rocket].
    ///
    /// This method serves as the primary constructor and requires an energy cell
    /// to initialize the rocket.
    ///
    /// # Arguments
    ///
    /// * `energy_cell` - An energy cell ([EnergyCell]) used to build the rocket.
    ///
    /// # Returns
    ///
    /// Returns a new instance of [Rocket].
    ///
    /// # Errors
    ///
    /// Returns an error if `energy_cell` is not charged.
    pub(crate) fn new(energy_cell: &mut EnergyCell) -> Result<Rocket, String> {
        energy_cell.discharge().map(|_| Rocket { _private: () })
    }
}
