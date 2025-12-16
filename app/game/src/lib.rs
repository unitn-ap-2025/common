//! # Common Game Crate
//!
//! This crate includes the shared architecture, components, and communication
//! protocols used by all implementations of the UniTN Advanced Programming course
//! project of 2025.
//!
//! The project consists in bulding a space simulation game where multiple explorers
//! travel through planets to collect resources and combine them.
//!
//! This crate does not aim provide a full implementation of the project, instead it
//! exists to make multiple implementations of shared components (Planets and Explorers)
//! intercompatible.
//!
//! ## Architectural overview
//! ### Components
//! The system has three main actors:
//! - **Orchestrator**: coordinates the simulation and message routing
//! - **Planets**: stateful entities that manage energy and resources
//! - **Explorers**: mobile agents that travel between planets and interact with them
//!
//! ### Protocol
//! The protocol through which actors should communicate is defined in the `protocols` module.
//! It includes a definition for all messages and documentation on how to use them.
//!
//! ### Logging
//! Logging infrastructure and specification can be found in the `logging` module.

pub mod components;
pub mod logging;
pub mod protocols;
pub mod utils;
