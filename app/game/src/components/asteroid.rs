/// Represents an asteroid object, instanciable by the orchestrator.
///
#[derive(Debug)]
pub struct Asteroid {
    _private: (),
}

#[allow(dead_code)]
impl Default for Asteroid {
    fn default() -> Self {
        Self::new()
    }
}

impl Asteroid {
    /// Creates a new, default instance of an [Asteroid].
    ///
    /// This method is the basic constructor and does not require any
    /// specific initial parameters.
    ///
    /// # Returns
    ///
    /// Returns a new instance of [Asteroid].
    pub(crate) fn new() -> Asteroid {
        Asteroid { _private: () }
    }
}
