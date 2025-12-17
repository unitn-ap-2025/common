//! # Orchestrator and Explorer protocol messages
//!
//! Defines the types of messages exchanged of the full duplex communication channel
//! between the Orchestrator and the Explorers
//! For a more detailed view of the interactions between these two entities, visit the communications [diagrams](https://github.com/unitn-ap-2025/common/blob/main/MESSAGE_DIAGRAMS.md)
use crate::components::resource::{BasicResourceType, ComplexResourceType};
use crate::protocols::planet_explorer::ExplorerToPlanet;
use crate::utils::ID;
use crossbeam_channel::Sender;
use enum_as_inner::EnumAsInner;
use std::collections::HashSet;
use strum_macros::EnumDiscriminants;

#[cfg(doc)]
use crate::components::resource::{BasicResource, ComplexResource};

/// This enum describes all possible messages from the Orchestrator to an Explorer
#[derive(Debug, EnumAsInner, EnumDiscriminants)]
#[strum_discriminants(name(OrchestratorToExplorerKind))]
pub enum OrchestratorToExplorer {
    /// This variant is used to start an Explorer AI
    /// 
    /// **Expected Response**: [`ExplorerToOrchestrator::StartExplorerAIResult`]
    /// 
    /// **Use Case**: Starting the Explorer AI at game start
    StartExplorerAI,
    /// This variant is used to reset the Explorer AI and restart it if it is in manual mode
    /// 
    /// **Expected Response**: [`ExplorerToOrchestrator::ResetExplorerAIResult`]
    /// 
    /// **Use Case**: Reset the Explorer knowledge or restart the AI if it is in manual mode
    ResetExplorerAI,
    /// This variant is used to kill an Explorer
    /// 
    /// **Expected Response**: [`ExplorerToOrchestrator::KillExplorerResult`]
    /// 
    /// **Use Case**: Killing the explorer instantly
    KillExplorer,
    ///This variant is used to stop the Explorer AI from autonomous decision-making
    /// 
    /// **Expected Response**: [`ExplorerToOrchestrator::StopExplorerAIResult`]
    /// 
    /// **Use Case**: Stopping the autonomous decision-making and entering the manual mode
    StopExplorerAI,
    /// This variant is used to tell the Explorer to move to a different planet
    /// 
    /// **Expected Response**: [`ExplorerToOrchestrator::MovedToPlanetResult`]
    /// 
    /// **Use Case**
    /// 
    /// When in manual mode, the orchestrator moves the explorer to a new planet and gives the new [Sender]
    /// 
    /// When in normal mode, this is the response to [`ExplorerToOrchestrator::TravelToPlanetRequest`], in this case
    /// the orchestrator checks that the explorer can move to the planet specified in the request and sends the optional new sender
    MoveToPlanet {
        ///The optional [Sender] to the new planet, [None] if explorer cannot move to the specified planet
        sender_to_new_planet: Option<Sender<ExplorerToPlanet>>,
    },
    /// This variant is used to ask the ID of the Planet in which the Explorer is currently located
    /// 
    /// **Expected Response**: [`ExplorerToOrchestrator::CurrentPlanetResult`]
    CurrentPlanetRequest,
    /// This variant is used to enforce the Explorer to ask the supported Resources on the Planet
    /// 
    /// **Expected Response**: [`ExplorerToOrchestrator::SupportedResourceResult`]
    /// 
    /// **Use Case**: In manual mode, ask the explorer to send a [`ExplorerToPlanet::SupportedResourceRequest`] to know the available [`BasicResourceType`] on its current planet
    SupportedResourceRequest,
    /// This variant is used to enforce the Explorer to ask the supported Combinations on the Planet
    /// 
    /// **Expected Response**: [`ExplorerToOrchestrator::SupportedCombinationResult`]
    /// 
    /// **Use Case**: In manual mode, ask the explorer to send a [`ExplorerToPlanet::SupportedCombinationRequest`] to know the available [`ComplexResourceType`] on its current planet
    SupportedCombinationRequest,
    /// This variant is used to enforce the Explorer to ask the Planet to Generate a [`BasicResource`]
    /// 
    /// **Expected Response**: [`ExplorerToOrchestrator::GenerateResourceResponse`]
    /// 
    /// **Use Case**: In manual mode, ask the explorer to send a [`ExplorerToPlanet::GenerateResourceRequest`] craft a [`BasicResource`]
    GenerateResourceRequest {
        ///The type of basic resource to craft
        to_generate: BasicResourceType,
    },
    /// This variant is used to enforce the Explorer to ask the Planet to Generate a [`ComplexResource`] provided by [`ComplexResourceType`]
    /// 
    /// **Expected Response**: [`ExplorerToOrchestrator::CombineResourceResponse`]
    /// 
    /// **Use Case**: In manual mode, ask the explorer to send a [`ExplorerToPlanet::CombineResourceRequest`] to craft a [`ComplexResource`]
    CombineResourceRequest {
        ///The type of complex resource to generate
        to_generate: ComplexResourceType,
    },
    /// This variant is used to ask the content of the Explorer Bag
    /// 
    /// **Expected Response**: [`ExplorerToOrchestrator::BagContentResponse`]
    /// 
    /// **Use Case**: Message used by the GUI to get information on the Explorer bag content to be shown
    BagContentRequest,
    /// This variant is used to send to the Explorer the IDs of the planets to which it can be moved
    /// 
    /// **Response To**: [`ExplorerToOrchestrator::NeighborsRequest`]
    NeighborsResponse {
        ///The list of IDs of the planets to which it can be moved
        neighbors: Vec<ID>,
    },
}
/// This enum describes all possible messages from an Explorer to the Orchestrator
#[derive(Debug, EnumAsInner, EnumDiscriminants)]
#[strum_discriminants(name(ExplorerToOrchestratorKind))]
pub enum ExplorerToOrchestrator<T> {
    /// This variant is used to acknowledge the starting of the Explorer AI
    /// 
    /// **Response To**: [`OrchestratorToExplorer::StartExplorerAI`]
    StartExplorerAIResult {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
    },
    /// This variant is used to acknowledge the killing of an Explorer
    /// 
    /// **Response To**: [`OrchestratorToExplorer::KillExplorer`]
    KillExplorerResult {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
    },
    /// This variant is used to acknowledge the reset of the Explorer AI
    /// 
    /// **Response To**: [`OrchestratorToExplorer::ResetExplorerAI`]
    ResetExplorerAIResult {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
    },
    /// This variant is used to acknowledge the stopping of the Explorer AI
    /// 
    /// **Response To**: [`OrchestratorToExplorer::StopExplorerAI`]
    StopExplorerAIResult {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
    },
    /// This variant is used to acknowledge the transfer of an Explorer to a new Planet
    /// 
    /// **Response To**: [`OrchestratorToExplorer::MoveToPlanet`]
    MovedToPlanetResult {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
    },
    /// This variant is used to send the ID of the current planet on which the Explorer is located
    /// 
    /// **Response To**: [`OrchestratorToExplorer::CurrentPlanetRequest`]
    CurrentPlanetResult {
        ///The ID of the explorer sending the message
        explorer_id: ID,
        ///The ID of the planet it currently lives on
        planet_id: ID,
    },
    /// This variant is used to send the list of the available [`BasicResourceType`] in the Explorer's current planet
    /// 
    /// **Response To**: [`OrchestratorToExplorer::SupportedResourceRequest`]
    SupportedResourceResult {
        ///The ID of the explorer sending the message
        explorer_id: ID,
        ///The Set of [`BasicResourceType`] available in the Explorer's current planet
        supported_resources: HashSet<BasicResourceType>,
    },
    /// This variant is used to send the list of the available [`ComplexResourceType`] in the Explorer's current planet
    /// 
    /// **Response To**: [`OrchestratorToExplorer::SupportedCombinationRequest`]
    SupportedCombinationResult {
        ///The ID of the explorer sending the message
        explorer_id: ID,
        ///The Set of [`ComplexResourceType`] available in the Explorer's current planet
        combination_list: HashSet<ComplexResourceType>,
    },
    /// This variant is used to send the generated Basic Resource asked by the Orchestrator
    /// 
    /// **Response To**: [`OrchestratorToExplorer::GenerateResourceRequest`]
    GenerateResourceResponse {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
        ///A Result consisting of: [Ok] if the requested resource has been generated and added to the Explorer Bag
        /// 
        ///An [Err] String if the requested resource has not been generated
        generated: Result<(), String>,
    },
    /// This variant is used to send the generated [`ComplexResource`] asked by the Orchestrator
    /// 
    /// **Response To**: [`OrchestratorToExplorer::CombineResourceRequest`]
    CombineResourceResponse {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
        ///A Result consisting of: [Ok] if the requested resource has been generated and added to the Explorer Bag
        /// 
        ///An [Err] String if the requested resource has not been generated
        generated: Result<(), String>,
    },
    /// This message is for passing around the bag content and has been implemented with a generic type to let the group the freedom to implement the methods on it
    /// 
    /// **Response To**: [`OrchestratorToExplorer::BagContentRequest`]
    BagContentResponse {
        ///The ID of the explorer sending the message
        explorer_id: ID,
        ///The generic `bag_content` type
        bag_content: T,
    },
    /// This variant asks the Orchestrator for the list of neighbors Planets to travel to
    /// 
    /// **Expected Response**: [`OrchestratorToExplorer::NeighborsResponse`]
    /// 
    /// **Use Case**: Knowing reachable planets from current planet
    NeighborsRequest {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
        ///The ID of the current planet the Explorer lives on
        current_planet_id: ID,
    },
    /// This variant asks the Orchestrator to be sent to the specified Planet
    /// 
    /// **Expected Response**: [`OrchestratorToExplorer::MoveToPlanet`]
    /// 
    /// **Use Case**: Autonomously asking to travel to a planet
    TravelToPlanetRequest {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
        ///The ID of the current planet the Explorer lives on
        current_planet_id: ID,
        ///The ID of the planet the Explorer wants to travel to
        dst_planet_id: ID,
    },
}

impl<T> ExplorerToOrchestrator<T> {
    /// Helper method to extract the `explorer_id` field from any message variant
    /// without needing to match a specific one.
    pub fn explorer_id(&self) -> ID {
        match self {
            Self::StartExplorerAIResult { explorer_id, .. }
            | Self::KillExplorerResult { explorer_id, .. }
            | Self::ResetExplorerAIResult { explorer_id, .. }
            | Self::MovedToPlanetResult { explorer_id, .. }
            | Self::CurrentPlanetResult { explorer_id, .. }
            | Self::SupportedResourceResult { explorer_id, .. }
            | Self::SupportedCombinationResult { explorer_id, .. }
            | Self::GenerateResourceResponse { explorer_id, .. }
            | Self::CombineResourceResponse { explorer_id, .. }
            | Self::BagContentResponse { explorer_id, .. }
            | Self::NeighborsRequest { explorer_id, .. }
            | Self::TravelToPlanetRequest { explorer_id, .. }
            | Self::StopExplorerAIResult { explorer_id, .. } => *explorer_id,
        }
    }
}
