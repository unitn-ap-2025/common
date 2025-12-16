//! # Components
//!
//! This module defines all **core game entities** shared across implementations
//! of the Advanced Programming project.
//!
//! Components represent concrete objects that exist in the simulation, such as
//! planets, explorers, energy-related entities, and resources.
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
