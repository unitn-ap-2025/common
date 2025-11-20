#[allow(dead_code)]
pub mod resources {
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

    ///
    /// Gives the necessary methods to print out the resource
    ///
    pub trait Resource: Display {
        fn to_static_str(&self) -> &'static str;
    }

    ///
    /// Identifies a resource which could be both [`BasicResourceType`] and [`ComplexResourceType`]
    /// without actually containing the underlying resource,
    ///
    pub enum ResourceType {
        Basic(BasicResourceType),
        Complex(ComplexResourceType),
    }
    ///
    /// Contains a resource which could be both [`BasicResource`] and [`ComplexResource`]
    ///
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
            if self.set.insert(complex.clone()) {
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
            if self.set.insert(basic.clone()) {
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
            #[derive(Debug,Clone, Eq)]
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
            pub enum BasicResource {
                $($basic($basic),)*
            }

             ///
             /// Gives the choice between every possible complex resource
             ///
            pub enum ComplexResource {
                $($complex($complex),)*
            }

              ///
              /// Identifies a [`BasicResource`]
              /// without actually containing the underlying resource,
              ///
            #[derive(Debug,Clone,Eq)]
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
                        fn [<  $result:lower _fn >] ( _r1: $lhs  , _r2: $rhs ) -> $result   {
                            $result { _private: () }
                       }
                }
            )*

               paste::paste! {
                     ///
                     /// Gives a structured way to pass around the request to produce a complex resource
                     ///
                    pub enum ComplexResourceRequest{
                         $([<$result >]( $lhs, $rhs ), )*
                    }
            }

            impl Combinator {
                paste::paste! {
                    $(
                         pub fn [<make_ $result:lower>]  (&self, r1 :  $lhs  ,r2 : $rhs  ) -> Result<$result, (String, $lhs , $rhs )  > {
                             let c = ComplexResourceType::$result;
                            if let Some(_f_enum)  =  &self.set.get( &c ) {
                                Ok( [<$result:lower _fn >](r1,r2))
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
}


