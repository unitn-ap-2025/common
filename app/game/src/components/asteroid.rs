/// Represents an asteroid object, instanciable by the orchestrator.
///
pub struct Asteroid;
#[allow(dead_code)]
impl Asteroid {
    /// Creates a new, default instance of an [Asteroid].
    ///
    /// This method is the basic constructor and does not require any
    /// specific initial parameters.
    ///
    /// # Returns
    ///
    /// Returns a new instance of [Asteroid].
   pub fn new() -> Asteroid {
       Asteroid
   }
}