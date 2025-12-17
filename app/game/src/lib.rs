//! # Common Game Crate
//!
//! This crate includes the shared architecture, components, and communication
//! protocols used by all implementations of the `UniTN` Advanced Programming course
//! project of 2025.
//!
//! The project consists in bulding a space simulation game where multiple explorers
//! travel through planets to collect resources and combine them.
//!
//! This crate does not aim provide a full implementation of the project, instead it
//! exists to make multiple implementations of shared components (Planets and Explorers)
//! intercompatible.
//!
//! ## Actors
//! The system has three main actors.
//! - ### Planets
//!   Stateful entities that manage energy and resources.
//!   A partial implementation is provided in the [`planet`](crate::components::planet) module.
//!   It is meant to be extended by implementing your own [`PlanetAI`](crate::components::planet::PlanetAI).
//! - ### Orchestrator
//!   Coordinates the simulation and message routing.
//!   This actor is not implemented in this crate.
//! - ### Explorers
//!   Mobile agents that travel between planets and interact with them.
//!   This actor is not implemented in this crate.

pub mod components;
pub mod logging;
pub mod protocols;
pub mod utils;
