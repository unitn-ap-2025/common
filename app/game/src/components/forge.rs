//! Forge module
//!
//! This module defines the [Forge] type, a singleton-like component responsible
//! for generating [`Asteroid`] and [`Sunray`] instances.
//!
//! Only one Forge may exist at a time. Attempting to construct more than one
//! instance results in an error. The component is designed to centralize object
//! creation in a controlled manner.

use crate::components::asteroid::Asteroid;
use crate::components::sunray::Sunray;

/// Internal module containing global state used by the [Forge].
///
/// # Internal API - Do not use directly
///
/// This module must be public only because Rust requires `pub` visibility for
/// cross-module access within the crate.
/// It **is not** considered stable API and must not be used by external code.
pub(crate) mod internal {
    use std::sync::{LazyLock, Mutex};

    /// Tracks whether a [Forge] instance has already been created.
    ///
    /// # Internal API - Do not use directly
    pub(crate) static ALREADY_CREATED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
}

/// The `Forge` is a singleton-like generator used to create [`Asteroid`] and
/// [`Sunray`] instances.
///
/// Only one Forge may ever be created at a time. Attempting to create a second
/// instance will return an error.
///
/// This ensures that all energy-related components are produced in a controlled,
/// centralized manner.
pub struct Forge {
    /// Hidden field to prevent external construction.
    _private: (),
}

impl Forge {
    /// Attempts to create a new `Forge`.
    ///
    /// # Errors
    ///
    /// - Returns `"Another generator has already been created"` if a Forge
    ///   instance already exists.
    /// - Returns `"Internal error: forge state mutex poisoned"` if the internal
    ///   state cannot be accessed.
    pub fn new() -> Result<Self, String> {
        let mut check = internal::ALREADY_CREATED
            .lock()
            .map_err(|_| "Internal error: forge state mutex poisoned".to_string())?;

        if *check {
            Err("Another generator has already been created".into())
        } else {
            *check = true;
            Ok(Forge { _private: () })
        }
    }

    /// Creates a new [`Asteroid`].
    ///
    /// # Returns
    /// A freshly constructed `Asteroid` instance.
    #[must_use]
    pub fn generate_asteroid(&self) -> Asteroid {
        Asteroid::new()
    }

    /// Creates a new [`Sunray`].
    ///
    /// # Returns
    /// A freshly constructed `Sunray` instance.
    #[must_use]
    pub fn generate_sunray(&self) -> Sunray {
        Sunray::new()
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the [Forge].
    //!
    //! These tests validate singleton behavior and basic construction rules.

    use super::internal::ALREADY_CREATED;
    use super::*;

    /// Resets the global singleton state.
    ///
    /// Used only in tests.
    fn reset_flag() {
        let mut created = ALREADY_CREATED
            .lock()
            .expect("Test setup failed: mutex poisoned");
        *created = false;
    }

    /// Verifies that the first Forge creation succeeds.
    #[test]
    fn first_creation_succeeds() {
        reset_flag();
        assert!(Forge::new().is_ok());
    }

    /// Ensures that constructing a second Forge returns an error.
    #[test]
    fn second_creation_fails() {
        reset_flag();

        let g0 = Forge::new();
        assert!(g0.is_ok());

        let g1 = Forge::new();
        assert!(g1.is_err());
    }
}
