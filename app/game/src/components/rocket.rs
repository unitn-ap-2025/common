use crate::components::energy_cell::EnergyCell;
/// Represents the rocket in the game, used by the planet. 
///

#[allow(dead_code)]
struct Rocket;
#[allow(dead_code)]
impl Rocket {
    /// Creates a new instance of [Rocket].
    ///
    /// This method serves as the primary constructor and requires an energy cell
    /// to initialize the rocket.
    ///
    /// # Arguments
    ///
    /// * `_energy_cell` - An energy cell ([EnergyCell]) used to build the rocket.
    ///
    /// # Returns
    ///
    /// Returns a new instance of [Rocket].
    pub(crate) fn new(_energy_cell: EnergyCell) -> Self {
        Rocket
    }
}
