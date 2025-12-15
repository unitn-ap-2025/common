//! # Components
//!
//! This module defines all **core game entities** shared across implementations
//! of the Advanced Programming project.
//!
//! Components represent concrete objects that exist in the simulation, such as
//! planets, explorers, energy-related entities, and resources.
//!
//! ## Responsibilities of components
//! - Encode domain concepts as strongly typed Rust structures
//! - Enforce system-wide invariants at construction time
//! - Prevent illegal states through restricted constructors
//! - Serve as the only valid data exchanged through protocols
//!
//! ## Creation and ownership rules
//! Some components are **user-inconstructible** and can only be created by
//! privileged system actors:
//! - **Forge** is a singleton and is the sole creator of `Sunray` and `Asteroid`
//! - `Sunray` and `Asteroid` instances cannot be constructed directly by users
//! - Rockets can only be built through controlled energy consumption
//!
//! These restrictions are enforced to prevent inconsistent or invalid system
//! states.
//!
//! ## Energy and resources
//! Energy is represented explicitly via `EnergyCell`s, which have a **binary
//! charged / uncharged state**. Energy consumption is required for:
//! - Basic resource generation
//! - Complex resource combination
//! - Rocket construction
//!
//! Resources are divided into **basic** and **complex** types. Complex resources
//! can only be obtained by combining specific basic or complex inputs according
//! to predefined recipes.
//!
//! ## Planet constraints
//! Planet behavior is restricted by its declared type. Depending on the type,
//! a planet may:
//! - Have a limited or unlimited number of energy cells
//! - Support a bounded or unbounded number of resource recipes
//! - Be allowed or forbidden from owning rockets
//!
//! These constraints are validated during planet construction and must not be
//! bypassed by implementations.
//!
//! ## Communication
//! Components do not communicate directly with each other. All interactions
//! occur through the message protocols defined in the `protocols` module.

pub mod asteroid;
pub mod energy_cell;
pub mod planet;
pub mod resource;

pub mod forge;
pub mod rocket;
pub mod sunray;
