/// Represents a sunray object, instanciable by the orchestrator.
pub struct Sunray {
    _private: (),
}
#[allow(dead_code)]
impl Default for Sunray {
    fn default() -> Self {
        Self::new()
    }
}

impl Sunray {
    /// Creates a new, default instance of a [Sunray].
    ///
    /// This method is the basic constructor and does not require any
    /// specific initial parameters.
    ///
    /// # Returns
    ///
    /// Returns a new instance of [Sunray].
    pub fn new() -> Sunray {
        Sunray { _private: () }
    }
}
