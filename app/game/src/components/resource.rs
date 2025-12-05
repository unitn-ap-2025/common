//! # Resource Module
//! Defines all basic and complex resources.
//!
//! Every resource has a struct and implements the resource trait.
//!
//! The primary public items for other modules are the [`Resource`] trait,
//! the enums that aggregate the generated types, such as [`BasicResource`], [`ComplexResource`] and [`GenericResource`] for passing the actual [`Resource`] around and
//! [`BasicResourceType`],[`ComplexResourceType`] and [`ResourceType`] for passing the names and acts like phantoms of a resource.
//! There exist two structs [`Generator`] and [`Combinator`] which are given to the planet to generate and combine resources,
//! these structs contain the signatures of the available recipes, these recipes are only updated in the constructor of the planet
//! To generate of combinator resources use the available methods in either structs, these return a [Result]
//! There exist an enum to pass around the request to the planet to generate a complex resource  [`ComplexResourceRequest`]
use crate::components::energy_cell::EnergyCell;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

/// Gives the necessary methods to print out the resource
pub trait Resource: Display {
    fn to_static_str(&self) -> &'static str;
}

///
/// Identifies a resource which could be both [`BasicResourceType`] and [`ComplexResourceType`]
/// without actually containing the underlying resource,
///
#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Basic(BasicResourceType),
    Complex(ComplexResourceType),
}
///
/// Contains a resource which could be both [`BasicResource`] and [`ComplexResource`]
///
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GenericResource {
    BasicResources(BasicResource),
    ComplexResources(ComplexResource),
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

///contains all the recipes available to a planet and enables the creation of complex resources
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
    pub fn new() -> Combinator {
        Combinator {
            set: Default::default(),
        }
    }
    ///to find out if a specific recipe is contained
    pub fn contains(&self, complex: ComplexResourceType) -> bool {
        matches!(&self.set.get(&complex), Some(_f))
    }
    ///to add a specific recipe
    ///this returns ar [Err] if there already was a recipe for a [Resource], since there can't be two different recipes for a single [Resource]
    ///
    #[allow(unused)]
    pub(crate) fn add(&mut self, complex: ComplexResourceType) -> Result<(), String> {
        if self.set.insert(complex) {
            Ok(())
        } else {
            Err(format!(
                "There was already a recipe for {:?}, the value should never be updated",
                complex
            ))
        }
    }
    ///to retrieve all available recipes
    pub fn all_available_recipes(&self) -> HashSet<ComplexResourceType> {
        self.set.iter().cloned().collect()
    }
}

///contains all the recipes available to a planet and enables the creation of basic resources
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
    pub fn new() -> Generator {
        Generator {
            set: Default::default(),
        }
    }
    ///to find out if a specific recipe is contained
    pub fn contains(&self, basic: BasicResourceType) -> bool {
        matches!(&self.set.get(&basic), Some(_f))
    }
    ///to add a specific recipe
    ///this returns ar [Err] if there already was a recipe for a [Resource], since there can't be two different recipes for a single [Resource]
    ///
    #[allow(unused)]
    pub(crate) fn add(&mut self, basic: BasicResourceType) -> Result<(), String> {
        if self.set.insert(basic) {
            Ok(())
        } else {
            Err(format!(
                "There was already a recipe for {:?}, the value should never be updated",
                basic
            ))
        }
    }
    ///to retrieve all available recipes
    pub fn all_available_recipes(&self) -> HashSet<BasicResourceType> {
        self.set.iter().cloned().collect()
    }
}
macro_rules! define_resources {
        (Basic: [$($basic:ident),* $(,)?], Complex: [$($complex:ident),* $(,)?]) => {

            $(
                #[derive(Debug, PartialEq,Eq,Hash)]
                pub struct $basic { _private: () }

                impl Display for $basic {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "Basic Resource {}", stringify!($basic))
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

            )*

             ///
             /// Identifies a [`ComplexResource`]
             /// without actually containing the underlying resource,
             ///
            #[derive(Debug,Clone,Copy, Eq)]
            pub enum ComplexResourceType {
                $($complex,)*
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

             ///
             /// Gives the choice between every possible basic resource
             ///
             #[derive(Debug, PartialEq,Eq,Hash)]
            pub enum BasicResource {
                $($basic($basic),)*
            }

             ///
             /// Gives the choice between every possible complex resource
             ///
             #[derive(Debug ,PartialEq,Eq,Hash)]
            pub enum ComplexResource {
                $($complex($complex),)*
            }

              ///
              /// Identifies a [`BasicResource`]
              /// without actually containing the underlying resource,
              ///
            #[derive(Debug,Clone,Copy,Eq)]
            pub enum BasicResourceType {
                $($basic,)*
            }


             impl Generator {
                paste::paste! {
                    $(
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
            }
        };
    }

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
                 ///
                 /// Gives a structured way to pass around the request to produce a complex resource
                 ///
                 #[derive(Debug, PartialEq,Eq,Hash )]
                pub enum ComplexResourceRequest{
                     $([<$result >]( $lhs, $rhs ), )*
                }
            }

            impl Combinator {
                paste::paste! {
                    $(
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
// ... (End of your define_combination_rules! macro call)

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
}
