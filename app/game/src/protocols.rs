//! # Communication Protocols
//!
//! This module defines all **message-based communication protocols** used by
//! the actors in the system.
//!
//! Protocols specify *what* messages can be exchanged, *who* can send them,
//! and *which responses are required*. They form the backbone of all
//! inter-component interaction.
//!
//! ## Actor pairs
//! The system defines explicit protocols for the following actor pairs:
//! - Orchestrator ↔ Planet
//! - Planet ↔ Explorer
//! - Orchestrator ↔ Explorer
//!
//! No other communication paths are allowed.
//!
//! ## Message guarantees
//! Protocols enforce several global guarantees:
//! - Certain messages require **mandatory acknowledgments**
//! - Actors in a stopped state must respond deterministically
//! - Termination messages (`Kill*`) are always honored
//! - Planet destruction may trigger cascading termination of explorers
//!
//! These rules ensure that the system remains observable, debuggable, and
//! free of deadlocks or undefined behavior.
//!
//! ## Failure handling
//! Errors are represented explicitly in message payloads rather than through
//! panics. When an operation fails, messages include sufficient information
//! to allow the sender to recover or retry.
//!
//! ## Determinism and safety
//! All protocols are designed to be:
//! - Deterministic
//! - Fully type-checked at compile time
//! - Independent of concrete actor implementations
//!
//! This allows heterogeneous implementations to interoperate safely without
//! requiring shared internal logic.

pub mod messages;
