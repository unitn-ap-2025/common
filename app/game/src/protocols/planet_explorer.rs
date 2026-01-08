//! # Planet and Explorer protocol messages
//!
//! Defines the types of messages exchanged of the full duplex communication channel
//! between the Planets and the Explorers
//! For a more detailed view of the interactions between these two entities, visit the communications [diagrams](https://github.com/unitn-ap-2025/common/blob/main/MESSAGE_DIAGRAMS.md)

use crate::components::resource::{
    BasicResource, BasicResourceType, ComplexResource, ComplexResourceRequest, ComplexResourceType,
    GenericResource,
};
use crate::utils::ID;
use enum_as_inner::EnumAsInner;
use std::collections::HashSet;
use strum_macros::EnumDiscriminants;

#[cfg(doc)]
use crate::components::energy_cell::EnergyCell;

/// This enum describes all possible messages from an Explorer to a Planet.
#[derive(Debug, EnumAsInner, EnumDiscriminants)]
#[strum_discriminants(name(ExplorerToPlanetKind))]
#[strum_discriminants(derive(Hash))]
pub enum ExplorerToPlanet {
    /// This variant is used to ask the Planet for the available [`BasicResourceType`]
    ///
    /// **Expected Response**: [`PlanetToExplorer::SupportedResourceResponse`]
    ///
    /// **Use Case**: Asking Available Basic Resources
    SupportedResourceRequest {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
    },
    /// This variant is used to ask the Planet for the available [`ComplexResourceType`]
    ///
    /// **Expected Response**: [`PlanetToExplorer::SupportedCombinationResponse`]
    ///
    /// **Use Case**: Asking Available Complex Resources
    SupportedCombinationRequest {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
    },
    /// This variant is used to ask the Planet to generate a [`BasicResource`]
    ///
    /// **Expected Response**: [`PlanetToExplorer::GenerateResourceResponse`]
    ///
    /// **Use Case**: Asking to craft a Basic Resource
    GenerateResourceRequest {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
        ///The basic resource to be generated
        resource: BasicResourceType,
    },
    /// This variant is used to ask the Planet to generate a [`ComplexResource`] using the [`ComplexResourceRequest`]
    ///
    /// **Expected Response**: [`PlanetToExplorer::CombineResourceResponse`]
    ///
    /// **Use Case**: Asking to craft a Complex Resource
    CombineResourceRequest {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
        ///The struct containing the complex resource to generate and the resources to be combined for the crafting to take place
        msg: ComplexResourceRequest,
    },
    /// This variant is used to ask the Planet for the available charged [`EnergyCell`] number
    ///
    /// **Expected Response**: [`PlanetToExplorer::AvailableEnergyCellResponse`]
    ///
    /// **Use Case**: Asking the number of charged cells available
    AvailableEnergyCellRequest {
        ///The ID of the Explorer sending the message
        explorer_id: ID,
    },
}

impl ExplorerToPlanet {
    /// Helper method to extract the `explorer_id` field from any message variant
    /// without needing to match a specific one.
    #[must_use]
    pub fn explorer_id(&self) -> ID {
        match self {
            ExplorerToPlanet::SupportedResourceRequest { explorer_id, .. }
            | ExplorerToPlanet::SupportedCombinationRequest { explorer_id, .. }
            | ExplorerToPlanet::GenerateResourceRequest { explorer_id, .. }
            | ExplorerToPlanet::CombineResourceRequest { explorer_id, .. }
            | ExplorerToPlanet::AvailableEnergyCellRequest { explorer_id, .. } => *explorer_id,
        }
    }
}

/// This enum describes all possible messages from a Planet to an Explorer.
#[derive(Debug, EnumAsInner, EnumDiscriminants)]
#[strum_discriminants(name(PlanetToExplorerKind))]
#[strum_discriminants(derive(Hash))]
pub enum PlanetToExplorer {
    /// This variant is used to send the available [`BasicResourceType`] list to the Explorer
    ///
    /// **Response To**: [`ExplorerToPlanet::SupportedResourceRequest`]
    SupportedResourceResponse {
        ///The list of available [`BasicResourceType`]
        resource_list: HashSet<BasicResourceType>,
    },
    /// This variant is used to send the available [`ComplexResourceType`] list to the Explorer
    ///
    /// **Response To**: [`ExplorerToPlanet::SupportedCombinationRequest`]
    SupportedCombinationResponse {
        ///The list of available [`ComplexResourceType`]
        combination_list: HashSet<ComplexResourceType>,
    },
    /// This variant is used to send the generated Basic Resource
    ///
    /// **Response To**: [`ExplorerToPlanet::GenerateResourceRequest`]
    GenerateResourceResponse {
        ///The optional Basic Resource generated:
        ///
        /// [Some(BasicResource)] if resource has been crafted correctly
        ///
        /// [None] if some error occurred
        resource: Option<BasicResource>,
    },
    /// This variant is used to send the [`ComplexResource`] generated
    ///
    /// **Response To**: [`ExplorerToPlanet::CombineResourceRequest`]
    CombineResourceResponse {
        ///The complex basic resource generated:
        ///
        ///[Ok(ComplexResource)] if complex resource has been crafted correctly
        ///
        ///An [Err] triplet containing an error String and the two resources that were intended to be combined that are given
        ///back to the Explorer
        complex_response: Result<ComplexResource, (String, GenericResource, GenericResource)>,
    },
    /// This variant is used to send the number of available energy cells to the Explorer
    ///
    /// **Response To**: [`ExplorerToPlanet::AvailableEnergyCellRequest`]
    AvailableEnergyCellResponse {
        ///The number of charged cells available
        available_cells: ID,
    },
    /// This variant is used by planets that are currently in a *stopped* state
    /// to acknowledge any message coming from an explorer
    Stopped,
}
