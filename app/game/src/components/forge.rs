use crate::components::asteroid::Asteroid;
use crate::components::sunray::Sunray;

use lazy_static::lazy_static;
use std::sync::Mutex;

//Variable that indicate if the generator has already been created

lazy_static! {
    static ref ALREADY_CREATED: Mutex<bool> = Mutex::new(false);
}

#[allow(dead_code)]
pub struct Forge {
    //Private field to forbid the creation of a generator without using new()
    _private: (),
}

#[allow(dead_code)]
impl Forge {
    //New method uses the ALREADY_CREATED variable to check if the generator has already been created or not
    pub fn new() -> Result<Self, String> {
        let mut check = ALREADY_CREATED.lock().unwrap();
        if !*check {
            *check = true;
            Ok(Forge { _private: () })
        } else {
            Err("Another generator has already been created".into())
        }
    }

    //Generator is the only entity that can create asteroids and sunrays

    pub fn generate_asteroid(&self) -> Asteroid {
        Asteroid::new()
    }

    pub fn generate_sunray(&self) -> Sunray {
        Sunray::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_first_creation_succeeds() {
        // Flag reset
        {
            let mut created = ALREADY_CREATED.lock().unwrap();
            *created = false;
        }

        let g = Forge::new();
        assert!(g.is_ok(), "The first forge should be created correctly");
    }

    #[test]
    fn test_generator_second_creation_fails() {
        // Flag reset
        {
            let mut created = ALREADY_CREATED.lock().unwrap();
            *created = false;
        }

        let g0 = Forge::new();
        assert!(g0.is_ok());

        let g1 = Forge::new();
        assert!(g1.is_err(), "The second forge should return an error");
    }
}
