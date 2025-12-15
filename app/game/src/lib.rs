//! # Common Game Crate
//!
//! This crate defines the shared architecture, components, and communication
//! protocols used by all implementations of the **Advanced Programming** course
//! project.
//!
//! It exists to allow **independent teams ("contractors")** to develop planets,
//! explorers, orchestrators, and other components that are guaranteed to be
//! compatible with each other at integration time.
//!
//! ## What this crate provides
//! - Public, stable interfaces for all core game entities
//! - Message-based protocols for inter-component communication
//! - Strong invariants enforced through the Rust type system
//! - Runtime validation of architectural constraints
//!
//! This crate intentionally focuses on **interfaces and behavior**, not on
//! internal implementation details.
//!
//! ## Design principles
//! - **Protocol-first design**: all interactions occur via typed messages
//! - **User-inconstructible critical types**: some entities can only be created
//! by privileged components
//! - **Explicit failure modes**: no undocumented panics
//! - **No `unsafe` code**: memory safety is fully guaranteed
//! - **Clippy-pedantic compliance**: high code quality is expected
//!
//! ## Architectural overview
//! The system is composed of three main actor types:
//! - **Orchestrator**: coordinates the simulation and message routing
//! - **Planets**: stateful entities that manage energy and resources
//! - **Explorers**: mobile agents that travel between planets
//!
//! All actors communicate exclusively through the protocols defined in this
//! crate. Direct shared-memory interaction between actors is forbidden.
//!
//! ## Source of truth
//! This documentation, generated via `cargo doc`, is the **authoritative
//! reference** for the project. External documents (e.g. PDFs) are considered
//! obsolete and non-normative.

pub mod components;
pub mod logging;
pub mod protocols;
