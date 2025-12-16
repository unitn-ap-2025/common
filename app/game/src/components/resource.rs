//! # Resource Module
//!
//! This module defines the resources that can be generated and combined in the game.
//! It provides a framework for creating basic and complex resources, and for defining
//! the recipes that govern their creation.
//!
//! ## Resources
//!
//! Resources are defined by the [`Resource`] trait, which provides a common interface for all
//! resources. There are two types of resources:
//!
//! - **Basic Resources**: These are the simplest resources, and can be generated with
//!   just an [`EnergyCell`]. Examples include `Oxygen` and `Hydrogen`.
//! - **Complex Resources**: These are created by combining other resources and an energy cell.
//!   Examples include `Water` and `Diamond`.
//!
//!
//! ## Generator and Combinator
//!
//! The [`Generator`] and [`Combinator`] structs are used to manage the recipes for
//! creating resources. The `Generator` is responsible for creating basic resources,
//! while the `Combinator` is responsible for creating complex resources.
//!
//! Each planet has its own `Generator` and `Combinator`, which are initialized with
//! the recipes that are available to that planet.
use crate::components::energy_cell::EnergyCell;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

/// A trait that provides a common interface for all resources.
pub trait Resource: Display {
    /// Returns a static string representation of the resource.
    fn to_static_str(&self) -> &'static str;
}

/// An enum that identifies a resource, which can be either a [`BasicResourceType`] or a
/// [`ComplexResourceType`], without actually containing the underlying resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    /// A basic resource type.
    Basic(BasicResourceType),
    /// A complex resource type.
    Complex(ComplexResourceType),
}

/// An enum that contains a resource, which can be either a [`BasicResource`] or a
/// [`ComplexResource`].
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GenericResource {
    /// A basic resource.
    BasicResources(BasicResource),
    /// A complex resource.
    ComplexResources(ComplexResource),
}

impl GenericResource {
    /// Returns the [`ResourceType`] of the `GenericResource`.
    #[must_use] 
    pub fn get_type(&self) -> ResourceType {
        match self {
            GenericResource::BasicResources(basic) => ResourceType::Basic(basic.get_type()),
            GenericResource::ComplexResources(complex) => ResourceType::Complex(complex.get_type()),
        }
    }
}

impl Hash for ComplexResourceType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl Hash for BasicResourceType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

/// Manages the recipes and production of complex resources for a planet.
///
/// The `Combinator` is responsible for storing the allowed recipes for [`ComplexResource`]s
/// and validating creation requests.
///
/// It works in conjunction with an [`EnergyCell`]. To create a complex resource,
/// the combinator:
/// 1. Checks if the requested resource type is in its set of allowed recipes.
/// 2. Consumes the required input resources.
/// 3. Discharges the provided `EnergyCell` to power the combination process.
///
/// Each planet instance has its own `Combinator` initialized with a specific set of rules.
#[derive(Debug)]
pub struct Combinator {
    set: HashSet<ComplexResourceType>,
}

impl Default for Combinator {
    fn default() -> Self {
        Self::new()
    }
}

impl Combinator {
    /// Creates a new `Combinator` with no recipes.
    #[must_use] 
    pub fn new() -> Combinator {
        Combinator {
            set: Default::default(),
        }
    }

    /// Returns `true` if the `Combinator` contains a recipe for the specified
    /// [`ComplexResourceType`].
    #[must_use] 
    pub fn contains(&self, complex: ComplexResourceType) -> bool {
        matches!(&self.set.get(&complex), Some(_f))
    }

    /// # Internal API - Do not use directly
    ///
    /// Adds a recipe for the specified [`ComplexResourceType`] to the `Combinator`.
    /// This method is intended for internal use only, to initialize a planet's `Combinator`.
    #[doc(hidden)]
    pub(crate) fn add(&mut self, complex: ComplexResourceType) -> Result<(), String> {
        if self.set.insert(complex) {
            Ok(())
        } else {
            Err(format!(
                "There was already a recipe for {complex:?}, the value should never be updated"
            ))
        }
    }

    /// Returns a `HashSet` of all the recipes available in the `Combinator`.
    #[must_use] 
    pub fn all_available_recipes(&self) -> HashSet<ComplexResourceType> {
        self.set.iter().copied().collect()
    }
}

/// Manages the recipes and production of basic resources for a planet.
///
/// The `Generator` is responsible for storing the allowed recipes for [`BasicResource`]s
/// and validating creation requests.
///
/// Unlike the [`Combinator`], the `Generator` creates resources "from scratch" (using only energy).
/// To create a basic resource, the generator:
/// 1. Checks if the requested resource type is in its set of allowed recipes.
/// 2. Discharges the provided [`EnergyCell`] to power the generation process.
///
/// Each planet instance has its own `Generator` initialized with a specific set of rules.
#[derive(Debug)]
pub struct Generator {
    set: HashSet<BasicResourceType>,
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator {
    /// Creates a new `Generator` with no recipes.
    #[must_use] 
    pub fn new() -> Generator {
        Generator {
            set: Default::default(),
        }
    }

    /// Returns `true` if the `Generator` contains a recipe for the specified
    /// [`BasicResourceType`].
    #[must_use] 
    pub fn contains(&self, basic: BasicResourceType) -> bool {
        matches!(&self.set.get(&basic), Some(_f))
    }

    /// # Internal API - Do not use directly
    ///
    /// Adds a recipe for the specified [`BasicResourceType`] to the `Generator`.
    /// This method is intended for internal use only, to initialize a planet's `Generator`.
    #[doc(hidden)]
    pub(crate) fn add(&mut self, basic: BasicResourceType) -> Result<(), String> {
        if self.set.insert(basic) {
            Ok(())
        } else {
            Err(format!(
                "There was already a recipe for {basic:?}, the value should never be updated"
            ))
        }
    }

    /// Returns a `HashSet` of all the recipes available in the `Generator`.
    #[must_use] 
    pub fn all_available_recipes(&self) -> HashSet<BasicResourceType> {
        self.set.iter().copied().collect()
    }
}

/// A macro for defining the basic and complex resources.
///
/// This macro defines the structs and enums for the resources, and implements the
/// [`Resource`] trait for them. It also defines the methods for converting between
/// the different resource types.
///
/// ## Arguments
///
/// * `Basic`: A list of the basic resources to define.
/// * `Complex`: A list of the complex resources to define.
///
/// ## Generated Code
///
/// This macro generates the following code:
///
/// * A struct for each basic and complex resource.
/// * An implementation of the [`Resource`] trait for each resource.
/// * An implementation of the `Display` trait for each resource.
/// * Methods for converting between the different resource types.
/// * An enum for each of the following:
///     * `BasicResourceType`: An enum that identifies a basic resource.
///     * `ComplexResourceType`: An enum that identifies a complex resource.
///     * `BasicResource`: An enum that contains a basic resource.
///     * `ComplexResource`: An enum that contains a complex resource.
/// * Methods for the `Generator` and `Combinator` structs that allow to create
///   the resources.
///
/// ## Example
///
/// ```ignore
/// define_resources!(
///     Basic: [Oxygen, Hydrogen],
///     Complex: [Water]
/// );
///
/// define_combination_rules!(
///    Water from Hydrogen + Oxygen,
/// );
/// ```
macro_rules! define_resources {
        (Basic: [$($basic:ident),* $(,)?], Complex: [$($complex:ident),* $(,)?]) => {

            $(
                /// A basic resource.
                ///
                /// This struct represents the basic resource `$basic`.
                #[derive(Debug, PartialEq,Eq,Hash)]
                pub struct $basic { _private: () }

                impl Display for $basic {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "Basic Resource {}", stringify!($basic))
                    }
                }

                impl $basic {
                    /// Converts this resource to a [`ResourceType`].
                    pub fn to_type(&self) -> ResourceType {
                        match self {
                            $basic { .. } =>  ResourceType::Basic(BasicResourceType::$basic),
                        }
                    }

                    /// Converts this resource to a [`GenericResource`].
                    pub fn to_generic(self) -> GenericResource {
                        GenericResource::BasicResources( BasicResource::$basic(self) )
                    }

                    /// Converts this resource to a [`BasicResource`].
                    pub fn to_basic(self) -> BasicResource {
                        BasicResource::$basic( self )
                    }

                    /// Returns the [`BasicResourceType`] of this resource.
                    pub fn to_basic_type(&self) -> BasicResourceType {
                        match self {
                            $basic { .. } =>  BasicResourceType::$basic,
                        }
                    }
                }



                impl Resource for $basic {
                    fn to_static_str(&self) -> &'static str {
                        stringify!($basic)
                    }
                }

                 paste::paste!{
                    fn [<generate_ $basic:lower>] (energy_cell: &mut EnergyCell) -> Result<$basic , String> {
                            energy_cell.discharge().and_then(|()| Ok($basic { _private: () }))
                    }
                 }
            )*

            $(
                /// A complex resource.
                ///
                /// This struct represents the complex resource `$complex`.
                #[derive(Debug, PartialEq,Eq,Hash)]
                pub struct $complex {
                    _private: (),
                }
                impl Display for $complex {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "Complex Resource {}", stringify!($complex))
                    }
                }

                impl Resource for $complex {
                    fn to_static_str(&self) -> &'static str {
                        stringify!($complex)
                    }
                }

                 impl $complex {
                        /// Converts this resource to a [`ResourceType`].
                        pub fn to_type(&self) -> ResourceType {
                            match self {
                                $complex { .. } =>  ResourceType::Complex(ComplexResourceType::$complex),
                            }
                        }

                        /// Converts this resource to a [`GenericResource`].
                        pub fn to_generic(self) -> GenericResource {
                            GenericResource::ComplexResources( ComplexResource::$complex(self) )
                        }

                        /// Converts this resource to a [`ComplexResource`].
                        pub fn to_complex(self) -> ComplexResource {
                            ComplexResource::$complex( self )
                        }

                        /// Returns the [`ComplexResourceType`] of this resource.
                        pub fn to_complex_type(&self) -> ComplexResourceType {
                            match self {
                                $complex { .. } =>  ComplexResourceType::$complex,
                            }
                        }
                 }




            )*


            impl ResourceType{
                    paste::paste! {
                        $(
                            /// Creates a new [`ResourceType::Complex`] variant for `$complex`.
                            pub fn [< make_ $complex:lower >] () -> Self {
                                ResourceType::Complex(ComplexResourceType::$complex)
                            }
                        )*
                    }

                    paste::paste! {
                        $(
                            /// Returns `true` if the resource type is [`ComplexResourceType::$complex`].
                            pub fn [< is_ $complex:lower >] (&self) -> bool {
                                if let ResourceType::Complex(ComplexResourceType::$complex) = self {
                                    true
                                } else {
                                    false
                                }
                            }
                        )*
                    }

                     paste::paste! {
                        $(
                            /// Creates a new [`ResourceType::Basic`] variant for `$basic`.
                            pub fn [< make_ $basic:lower >] () -> Self {
                                ResourceType::Basic(BasicResourceType::$basic)
                            }
                        )*
                    }

                    paste::paste! {
                        $(
                            /// Returns `true` if the resource type is [`BasicResourceType::$basic`].
                            pub fn [< is_ $basic:lower >] (&self) -> bool {
                                if let ResourceType::Basic(BasicResourceType::$basic) = self {
                                    true
                                } else {
                                    false
                                }
                            }
                        )*
                    }

            }

            impl BasicResourceType{

                    paste::paste! {
                        $(
                            /// Returns `true` if the resource type is `$basic`.
                            pub fn [< is_ $basic:lower >] (&self) -> bool {
                                if let BasicResourceType::$basic = self {
                                    true
                                } else {
                                    false
                                }
                            }
                        )*
                    }

            }


               impl ComplexResourceType{

                    paste::paste! {
                        $(
                            /// Returns `true` if the resource type is `$complex`.
                            pub fn [< is_ $complex:lower >] (&self) -> bool {
                                if let ComplexResourceType::$complex = self {
                                    true
                                } else {
                                    false
                                }
                            }
                        )*
                    }

            }

            /// An enum that identifies a [`ComplexResource`] type without actually containing the
            /// underlying resource.
            ///
            #[derive(Debug,Clone,Copy, Eq)]
            pub enum ComplexResourceType {
                $(
                    $complex,
                )*
            }

            impl BasicResource {
                /// Returns the [`BasicResourceType`] of this resource.
                pub fn get_type(&self) -> BasicResourceType {
                    match self {
                        $( BasicResource:: $basic (_) => BasicResourceType::$basic, )*
                    }
                }
                paste::paste!{
                           $(
                            /// Attempts to convert the `BasicResource` into a `$basic`.
                            ///
                            /// # Returns
                            /// * `Ok($basic)` if the resource is `$basic`.
                            /// * `Err(String)` if the resource is of a different type.
                            pub fn [< to_ $basic:lower >] (self) -> Result< $basic , String> {
                                match self {
                                    BasicResource:: $basic (h) => Ok(h) ,
                                    _ => Err( "Different type found".into() )
                                }
                            }
                        )*
                }
            }
            impl GenericResource {
                paste::paste! {
                   $(
                        /// Attempts to convert the `GenericResource` into a `$complex`.
                        ///
                        /// # Returns
                        /// * `Ok($complex)` if the resource is `$complex`.
                        /// * `Err(String)` if the resource is of a different type.
                        pub fn [< to_ $complex:lower >] (self) -> Result< $complex,String> {
                            match self {
                                GenericResource::ComplexResources(ComplexResource:: $complex(h))  => Ok(h),
                                _ => Err("Different type found".into())
                            }
                        }
                    )*
                }

                paste::paste! {
                   $(
                        /// Attempts to convert the `GenericResource` into a `$basic`.
                        ///
                        /// # Returns
                        /// * `Ok($basic)` if the resource is `$basic`.
                        /// * `Err(String)` if the resource is of a different type.
                        pub fn [< to_ $basic:lower >] (self) -> Result< $basic , String> {
                            match self {
                                GenericResource::BasicResources(BasicResource:: $basic(h))  => Ok(h),
                                _ => Err("Different type found".into())
                            }
                        }
                    )*
                }
            }

            impl ComplexResource {
                /// Returns the [`ComplexResourceType`] of this resource.
                pub fn get_type(&self) -> ComplexResourceType {
                    match self {
                         $( ComplexResource:: $complex (_) => ComplexResourceType::$complex, )*
                    }
                }

                paste::paste!{
                   $(
                    /// Attempts to convert the `ComplexResource` into a `$complex`.
                    ///
                    /// # Returns
                    /// * `Ok($complex)` if the resource is `$complex`.
                    /// * `Err(String)` if the resource is of a different type.
                    pub fn [< to_ $complex:lower >] (self) -> Result< $complex,String> {
                        match self {
                            ComplexResource:: $complex( h) => Ok(h) ,
                            _ => Err("Different type found".into())
                        }
                    }
                )*
                }
            }


            impl PartialEq<Self> for ComplexResourceType {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        $( ( ComplexResourceType::$complex ,  ComplexResourceType::$complex) => { true}, )*
                        (_, _) => { false}
                    }
                }
            }
             impl PartialEq<Self> for BasicResourceType {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        $( ( BasicResourceType::$basic ,  BasicResourceType::$basic) => { true}, )*
                        (_, _) => { false}
                    }
                }
            }

            /// An enum that provides a unified type for all possible basic resources.
            ///
            /// This enum wraps every generated basic resource struct (e.g., `Oxygen`, `Hydrogen`)
            /// into a single type. It is useful when you need to store or pass around any basic
            /// resource without knowing its specific concrete type at compile time.
            #[derive(Debug, PartialEq,Eq,Hash)]
            pub enum BasicResource {
                $(
                    $basic($basic),
                )*
            }

            /// An enum that provides a unified type for all possible complex resources.
            ///
            /// This enum wraps every generated complex resource struct (e.g., `Water`, `Diamond`)
            /// into a single type. It is useful when you need to store or pass around any complex
            /// resource without knowing its specific concrete type at compile time.
            #[derive(Debug ,PartialEq,Eq,Hash)]
            pub enum ComplexResource {
                $(
                    $complex($complex),
                )*
            }

            /// An enum that identifies a [`BasicResource`] type without actually containing the
            /// underlying resource.
            ///
            /// This enum is generated by the `define_resources!` macro and contains a variant for
            /// each basic resource defined in the macro invocation. It is primarily used for
            /// type identification and recipe definitions within the [`Generator`].
            #[derive(Debug,Clone,Copy,Eq)]
            pub enum BasicResourceType {
                $(
                    $basic,
                )*
            }


             impl Generator {
                paste::paste! {
                    $(
                         /// Creates a new `[<$basic>]` resource.
                         ///
                         /// This method attempts to create a new instance of the corresponding basic
                         /// resource by discharging an `EnergyCell`.
                         ///
                         /// # Arguments
                         ///
                         /// * `energy_cell` - A mutable reference to an `EnergyCell` which will be
                         ///   discharged to create the resource.
                         ///
                         /// # Returns
                         ///
                         /// A `Result` indicating success or failure:
                         /// * `Ok([<$basic>])`: The resource was successfully created.
                         /// * `Err(String)`: An error occurred, either because there is no recipe
                         ///   for this resource or the `energy_cell` was not charged.
                         pub fn [<make_ $basic:lower>]  (&self, energy_cell : &mut EnergyCell ) -> Result<$basic, String > {
                             let b = BasicResourceType::$basic;
                            if let Some(_f_enum)  =  &self.set.get(&b) {
                                Ok( [<generate_ $basic:lower>] (energy_cell )?)
                            } else {
                               Err(format!("there isn't a recipe for {:?}", b))
                            }
                        }
                    )*
                }

                  /// Attempts to create a basic resource of the specified type.
                  ///
                  /// This method provides a generic way to request the creation of any basic
                  /// resource that the generator has a recipe for.
                  ///
                  /// # Arguments
                  ///
                  /// * `req` - The `BasicResourceType` enum variant representing the desired resource.
                  /// * `energy_cell` - A mutable reference to an `EnergyCell` to be used for
                  ///   discharging during resource creation.
                  ///
                  /// # Returns
                  ///
                  /// A `Result` indicating success or failure:
                  /// * `Ok(BasicResource)`: The requested resource was successfully created and
                  ///   wrapped in the `BasicResource` enum.
                  /// * `Err(String)`: An error occurred, such as the `energy_cell` not being charged
                  ///   or a missing recipe for the requested resource.
                  pub fn try_make(&self , req :  BasicResourceType , energy_cell: &mut EnergyCell) -> Result<BasicResource, String> {
                    if !energy_cell.is_charged() {
                        return Err("The energy is not charged".to_string());
                    }
                    match req {
                        $(
                            BasicResourceType::$basic => {
                            if self.set.contains( &BasicResourceType::$basic ) {
                                energy_cell.discharge()?;
                                Ok($basic{ _private: () }.to_basic())
                            }
                            else {
                                Err(format!("Missing recipe for {:?}", stringify!($basic) ))
                            }
                        },
                        )*
                    }
                }

            }
        };
    }

/// A macro for defining the combination rules for complex resources.
///
/// This macro defines the functions for creating complex resources from other
/// resources.
///
/// ## Arguments
///
/// * A list of rules, where each rule has the following format:
///   `result from lhs + rhs`
///
/// ## Generated Code
///
/// This macro generates the following code:
///
/// * A function for each combination rule that creates the complex resource.
/// * An enum that gives a structured way to pass around the request to produce a
///   complex resource.
/// * An implementation of the `try_make` method for the `Combinator` struct that
///   allows to create the complex resources.
///
/// ## Example
///
/// ```ignore
/// define_resources!(
///   Basic: [Hydrogen, Oxygen],
///  Complex: [Water]
/// );
///
/// define_combination_rules!(
///     Water from Hydrogen + Oxygen,
/// );
/// ```
macro_rules! define_combination_rules {
        ($($result:ident from  $lhs:ident + $rhs:ident ),* $(,)?) => {
            $(
                paste::paste! {
                    fn [<  $result:lower _fn >] ( r1: $lhs  , r2: $rhs , energy_cell: &mut EnergyCell) ->  Result<$result, (String ,$lhs , $rhs ) >    {
                        match energy_cell.discharge(){
                            Ok(_) => Ok($result { _private: () }),
                            Err(e) => Err( (e, r1, r2 )),
                        }
                   }
                }
            )*

            paste::paste! {
                /// An enum that represents a structured request to produce a specific complex resource.
                ///
                /// Each variant corresponds
                /// to a combination rule and holds the necessary input resources (`lhs` and `rhs`) required
                /// to produce the target complex resource.
                ///
                /// It allows passing all ingredients for a reaction as a single object to the [`Combinator`].
                #[derive(Debug, PartialEq,Eq,Hash )]
                pub enum ComplexResourceRequest{
                     $(
                        [<$result >]( $lhs, $rhs ),
                     )*
                }
            }

            impl Combinator {
                paste::paste! {
                    $(
                         /// Creates a new `[<$result>]` resource.
                         ///
                         /// This method attempts to create a new instance of the corresponding
                         /// complex resource by combining two input resources (`r1` and `r2`) and
                         /// discharging an `EnergyCell`.
                         ///
                         /// # Arguments
                         ///
                         /// * `r1` - The first input resource ([`$lhs`]).
                         /// * `r2` - The second input resource ([`$rhs`]).
                         /// * `energy_cell` - A mutable reference to an `EnergyCell` which will be
                         ///   discharged to create the resource.
                         ///
                         /// # Returns
                         ///
                         /// A `Result` indicating success or failure:
                         /// * `Ok([<$result>])`: The complex resource was successfully created.
                         /// * `Err((String, [$lhs], [$rhs]))`: An error occurred. The tuple contains:
                         ///     1. An error message string (e.g., missing recipe, uncharged cell).
                         ///     2. The original input resource `r1` (returned so it is not lost).
                         ///     3. The original input resource `r2` (returned so it is not lost).
                         pub fn [<make_ $result:lower>]  (&self, r1 :  $lhs  ,r2 : $rhs , energy_cell: &mut EnergyCell  ) -> Result<$result, (String, $lhs , $rhs )  > {
                             let c = ComplexResourceType::$result;
                            if let Some(_f_enum)  =  &self.set.get( &c ) {
                                  [<$result:lower _fn >](r1,r2 , energy_cell )
                            } else {
                               Err((format!("there isn't a recipe for {:?}", c), r1 ,r2 ) )
                            }
                        }
                    )*
                }

                 /// Attempts to create a complex resource based on a given request.
                 ///
                 /// This method provides a generic way to request the creation of any complex
                 /// resource that the combinator has a recipe for.
                 ///
                 /// # Arguments
                 ///
                 /// * `req` - The `ComplexResourceRequest` enum variant representing the desired
                 ///   complex resource and its required input resources.
                 /// * `energy_cell` - A mutable reference to an `EnergyCell` which will be
                 ///   discharged during resource creation.
                 ///
                 /// # Returns
                 ///
                 /// A `Result` indicating success or failure:
                 /// * `Ok(ComplexResource)`: The complex resource was successfully created.
                 /// * `Err((String, GenericResource, GenericResource))`: An error occurred. The tuple contains:
                 ///     1. An error message string (e.g., missing recipe, uncharged cell).
                 ///     2. The first input resource as a `GenericResource`.
                 ///     3. The second input resource as a `GenericResource`.
                 ///
                 /// The input resources are returned in the error case to prevent ownership loss
                 /// on failure.
                 pub fn try_make(&self , req :  ComplexResourceRequest , energy_cell: &mut EnergyCell) -> Result<ComplexResource, (String, GenericResource , GenericResource )> {
                    match req {
                        $(
                        ComplexResourceRequest::$result(r1, r2) => {
                            if self.set.contains( &ComplexResourceType::$result ) {
                                    paste::paste! {
                                     [<$result:lower _fn >](r1,r2 , energy_cell ).map(|r| {
                                            r.to_complex()
                                        }).map_err(|(s , r1 ,r2)| {
                                            (s , r1.to_generic() ,r2.to_generic())
                                        })
                                    }
                            }
                            else {
                               Err((format!("there isn't a recipe for {:?}", stringify!($result)), r1.to_generic() ,r2.to_generic() ) )
                            }
                        },
                        )*
                    }
                }

            }

        };
    }

define_resources!(
    Basic: [Oxygen , Hydrogen, Carbon, Silicon],
    Complex: [Diamond, Water , Life , Robot , Dolphin , AIPartner]
);

define_combination_rules!(
    Water from Hydrogen + Oxygen,
    Diamond from Carbon + Carbon,
    Life from Water + Carbon ,
    Robot from Silicon + Life ,
    Dolphin from Water + Life ,
    AIPartner from Robot +  Diamond
);

#[cfg(test)]
mod tests {
    use super::*;
    // Adjust these imports based on where your files are located in the crate.
    // Based on previous context, I assume:
    use crate::components::energy_cell::EnergyCell;
    use crate::components::sunray::Sunray;

    // --- Helper to get a charged cell ---
    fn get_charged_cell() -> EnergyCell {
        let mut cell = EnergyCell::new();
        // We use the real Sunray constructor now
        cell.charge(Sunray::new());
        cell
    }

    #[test]
    fn test_generator_success() {
        let mut generator = Generator::new();
        let mut cell = get_charged_cell();

        // 1. Add recipe
        assert!(generator.add(BasicResourceType::Oxygen).is_ok());

        // 2. Generate resource
        let result = generator.make_oxygen(&mut cell);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_static_str(), "Oxygen");

        // 3. Ensure cell is discharged
        assert!(!cell.is_charged());
    }

    #[test]
    fn test_generator_fail_no_charge() {
        let mut generator = Generator::new();
        let mut cell = EnergyCell::new(); // Not charged

        generator.add(BasicResourceType::Oxygen).unwrap();

        let result = generator.make_oxygen(&mut cell);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "EnergyCell not charged!");
    }

    #[test]
    fn test_generator_fail_no_recipe() {
        let generator = Generator::new(); // Empty, no recipes added
        let mut cell = get_charged_cell();

        // Try to make Oxygen without adding the recipe first
        let result = generator.make_oxygen(&mut cell);

        assert!(result.is_err());
        assert!(result.err().unwrap().contains("there isn't a recipe for"));
    }

    #[test]
    fn test_combinator_success() {
        let mut generator = Generator::new();
        let mut comb = Combinator::new();
        let mut cell = get_charged_cell();

        // Setup
        generator.add(BasicResourceType::Oxygen).unwrap();
        generator.add(BasicResourceType::Hydrogen).unwrap();
        comb.add(ComplexResourceType::Water).unwrap();

        let oxygen = generator.make_oxygen(&mut cell).unwrap();

        // Recharge cell using real Sunray
        cell.charge(Sunray::new());
        let hydrogen = generator.make_hydrogen(&mut cell).unwrap();

        // Test Combination: Water = Hydrogen + Oxygen
        cell.charge(Sunray::new());
        let result = comb.make_water(hydrogen, oxygen, &mut cell);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_static_str(), "Water");
    }

    #[test]
    fn test_combinator_fail_no_recipe_returns_resources() {
        let mut generator = Generator::new();
        let mut comb = Combinator::new(); // No recipes added
        let mut cell = get_charged_cell();

        generator.add(BasicResourceType::Oxygen).unwrap();
        generator.add(BasicResourceType::Hydrogen).unwrap();

        let oxygen = generator.make_oxygen(&mut cell).unwrap();

        cell.charge(Sunray::new());
        let hydrogen = generator.make_hydrogen(&mut cell).unwrap();

        // Attempt make_water without recipe
        let result = comb.make_water(hydrogen, oxygen, &mut cell);

        assert!(result.is_err());
        let (_s, r1, r2) = result.err().unwrap();
        comb.add(ComplexResourceType::Water).unwrap();
        let result = comb.make_water(r1, r2, &mut cell);
        assert!(result.is_err());

        // Critical: Ensure we got our resources back in the error tuple
        let (_err_msg, returned_h, returned_o) = result.err().unwrap();

        assert_eq!(returned_h.to_static_str(), "Hydrogen");
        assert_eq!(returned_o.to_static_str(), "Oxygen");
    }

    #[test]
    fn test_recipe_management() {
        let mut generator = Generator::new();

        assert!(generator.add(BasicResourceType::Carbon).is_ok());
        assert!(generator.contains(BasicResourceType::Carbon));
        assert!(!generator.contains(BasicResourceType::Silicon));

        // Test duplicate addition error
        assert!(generator.add(BasicResourceType::Carbon).is_err());
    }

    #[test]
    fn test_enum_equality_and_hashing() {
        let t1 = BasicResourceType::Oxygen;
        let t2 = BasicResourceType::Oxygen;
        let t3 = BasicResourceType::Carbon;

        assert_eq!(t1, t2);
        assert_ne!(t1, t3);

        // Test Hashing implicitly via HashSet
        let mut set = HashSet::new();
        set.insert(BasicResourceType::Oxygen);
        set.insert(BasicResourceType::Oxygen);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_complex_chain() {
        // Tests a multi-step chain: Carbon + Carbon -> Diamond; Robot + Diamond -> AIPartner
        let mut generator = Generator::new();
        let mut comb = Combinator::new();
        let mut cell = get_charged_cell();

        // Add Recipes
        generator.add(BasicResourceType::Carbon).unwrap();
        generator.add(BasicResourceType::Silicon).unwrap();
        generator.add(BasicResourceType::Oxygen).unwrap();
        generator.add(BasicResourceType::Hydrogen).unwrap();

        comb.add(ComplexResourceType::Diamond).unwrap();
        comb.add(ComplexResourceType::Water).unwrap();
        comb.add(ComplexResourceType::Life).unwrap();
        comb.add(ComplexResourceType::Robot).unwrap();
        comb.add(ComplexResourceType::AIPartner).unwrap();

        // 1. Make Diamond (Carbon + Carbon)
        let c1 = generator.make_carbon(&mut cell).unwrap();
        cell.charge(Sunray::new());
        let c2 = generator.make_carbon(&mut cell).unwrap();
        cell.charge(Sunray::new());
        let diamond = comb.make_diamond(c1, c2, &mut cell).unwrap();

        // 2. Make Robot (Silicon + Life) -> Needs Life (Water + Carbon) -> Needs Water (H + O)

        // Make Water
        cell.charge(Sunray::new());
        let h = generator.make_hydrogen(&mut cell).unwrap();
        cell.charge(Sunray::new());
        let o = generator.make_oxygen(&mut cell).unwrap();
        cell.charge(Sunray::new());
        let water = comb.make_water(h, o, &mut cell).unwrap();

        // Make Life
        cell.charge(Sunray::new());
        let c3 = generator.make_carbon(&mut cell).unwrap();
        cell.charge(Sunray::new());
        let life = comb.make_life(water, c3, &mut cell).unwrap();

        // Make Robot
        cell.charge(Sunray::new());
        let silicon = generator.make_silicon(&mut cell).unwrap();
        cell.charge(Sunray::new());
        let robot = comb.make_robot(silicon, life, &mut cell).unwrap();

        // 3. Make AIPartner (Robot + Diamond)
        cell.charge(Sunray::new());
        let ai = comb.make_aipartner(robot, diamond, &mut cell);

        assert!(ai.is_ok());
        assert_eq!(ai.unwrap().to_static_str(), "AIPartner");
    }

    #[test]
    fn test_generator_try_make() {
        let mut generator = Generator::new();
        let mut cell = get_charged_cell();
        generator.add(BasicResourceType::Oxygen).unwrap();

        // Test success
        let result = generator.try_make(BasicResourceType::Oxygen, &mut cell);
        assert!(result.is_ok());
        let resource = result.unwrap();
        assert_eq!(resource.get_type(), BasicResourceType::Oxygen);
        assert!(!cell.is_charged());

        // Test fail no charge
        let result = generator.try_make(BasicResourceType::Oxygen, &mut cell);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "The energy is not charged");

        // Test fail no recipe
        let mut cell = get_charged_cell();
        let result = generator.try_make(BasicResourceType::Hydrogen, &mut cell);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Missing recipe for"));
    }

    #[test]
    fn test_combinator_try_make() {
        let mut generator = Generator::new();
        let mut combinator = Combinator::new();
        combinator.add(ComplexResourceType::Water).unwrap();
        generator.add(BasicResourceType::Hydrogen).unwrap();
        generator.add(BasicResourceType::Oxygen).unwrap();

        let mut cell = get_charged_cell();
        let hydrogen = generator.make_hydrogen(&mut cell).unwrap();
        cell.charge(Sunray::new());
        let oxygen = generator.make_oxygen(&mut cell).unwrap();

        // Test success
        cell.charge(Sunray::new());
        let request = ComplexResourceRequest::Water(hydrogen, oxygen);
        let result = combinator.try_make(request, &mut cell);
        assert!(result.is_ok());
        let resource = result.unwrap();
        assert_eq!(resource.get_type(), ComplexResourceType::Water);
        assert!(!cell.is_charged());

        let hydrogen = generator.make_hydrogen(&mut get_charged_cell()).unwrap();
        let oxygen = generator.make_oxygen(&mut get_charged_cell()).unwrap();

        // Test fail no charge
        let request = ComplexResourceRequest::Water(hydrogen, oxygen);
        let result = combinator.try_make(request, &mut cell);
        assert!(result.is_err());
        let (err, _, _) = result.err().unwrap();
        assert_eq!(err, "EnergyCell not charged!");

        // Test fail no recipe
        let mut cell = get_charged_cell();
        let combinator = Combinator::new(); // No recipes
        let hydrogen = generator.make_hydrogen(&mut get_charged_cell()).unwrap();
        let oxygen = generator.make_oxygen(&mut get_charged_cell()).unwrap();
        let request = ComplexResourceRequest::Water(hydrogen, oxygen);
        let result = combinator.try_make(request, &mut cell);
        assert!(result.is_err());
        let (err, _, _) = result.err().unwrap();
        assert!(err.contains("there isn't a recipe for"));
    }

    #[test]
    fn test_generic_resource_conversions() {
        let oxygen = Oxygen { _private: () };
        let generic_basic = oxygen.to_generic();
        assert_eq!(
            generic_basic.get_type(),
            ResourceType::Basic(BasicResourceType::Oxygen)
        );
        assert!(generic_basic.to_oxygen().is_ok());

        let water = Water { _private: () };
        let generic_complex = water.to_generic();
        assert_eq!(
            generic_complex.get_type(),
            ResourceType::Complex(ComplexResourceType::Water)
        );
        assert!(generic_complex.to_water().is_ok());
    }
}
