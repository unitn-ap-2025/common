//! # Common Game Crate
//!
//! This crate includes the shared architecture, components, and communication
//! protocols used by all implementations of the UniTN Advanced Programming course
//! project of 2025.
//!
//! It exists to allow anyone to develop planets, explorers, orchestrators, and other
//! components that are guaranteed to be compatible with each other.
//!
//! ## Architectural overview
//! The system is composed of three main actor types:
//! - **Orchestrator**: coordinates the simulation and message routing
//! - **Planets**: stateful entities that manage energy and resources
//! - **Explorers**: mobile agents that travel between planets and interact with them
//!
//! All actors communicate exclusively through the protocols defined in the protocols module.
//! Direct shared-memory interaction between actors is forbidden.

pub mod components;
pub mod logging;
pub mod protocols;
