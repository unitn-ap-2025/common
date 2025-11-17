use crate::energy_cell::EnergyCell;
use crate::resources::*;

pub mod sunray {
    pub struct Sunray;
    impl Sunray {
        pub(crate) fn new() -> Sunray {
            Sunray
        }
    }
}
pub mod energy_cell {
    use crate::sunray::Sunray;
    pub struct EnergyCell {
        _private: (),
        charge: bool,
    }

    impl EnergyCell {
        pub fn new() -> EnergyCell {
            EnergyCell {
                _private: (),
                charge: false,
            }
        }

        pub fn charge(&mut self, sunray: Sunray) {
            if !self.charge {
                self.charge = true;
            }
            // if the cell is already charged nothing happens
        }

        pub fn discharge(&mut self) -> Result<(), String> {
            if self.charge {
                self.charge = false;
                Ok(())
            } else {
                Err("EnergyCell not charged!".to_string())
            }
        }

        pub fn is_charged(&self) -> bool {
            self.charge
        }
    }
}

pub mod resources {
    //! Defines all basic and complex resources.
    //!
    //! Every resource has a struct and implements the resource trait.
    //!
    //! The primary public items for other modules are the [`Resource`] trait
    //! and the enums that aggregate the generated types, such as [`BasicResource`], [`ComplexResource`] and [`GenericResource`] for passing the actual [`Resource`] around,
    //! [`BasicResourceType`],[`ComplexResourceType`] and [`ResourceType`] are for passing the names and acts like phantoms of a resource.
    //! There exist two functions which are pub(crate) and given to the planet constructor to generate and combine resources through the [`get_generation_fn`] and [`get_combination_fn`],
    //! every possible combination and generation rule then exist in two enums  [`CombinationFn`] and [`GenerationFn`].
    //! There exist an enum to pass around the request to the planet to generate a complex resource  [`ComplexResourceRequest`]
    use crate::energy_cell::EnergyCell;
    use paste::paste;
    use std::fmt::Display;

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
            pub enum ComplexResourceType {
                $($complex,)*
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
            pub enum BasicResourceType {
                $($basic,)*
            }


            paste::paste! {
                   ///
                   /// Gives the choice between every possible generation rule in the form of function pointers to a specific combination
                   ///
                    pub enum GenerationFn {
                         $([<$basic Generation>](fn (  &mut EnergyCell  ) -> Result<$basic, String> ), )*
                    }
            }

             ///
             /// Given a [`BasicResourceType`] returns the underlying function that generate the [`BasicResource`]
             ///
            pub(crate) fn get_generation_fn( result: BasicResourceType ) -> GenerationFn {
                match (result) {
                    $(
                         BasicResourceType::$basic => {
                            paste! {
                                GenerationFn::[<$basic Generation>]([< generate_ $basic:lower  >])
                            }
                        },
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
                     /// Gives the choice between every possible combination rule in the form of function pointers to a specific combination
                     ///
                    pub enum CombinationFn {
                         $([<$result Fn>](fn ( $lhs, $rhs  ) -> $result), )*
                    }
            }

               paste::paste! {
                     ///
                     /// Gives a structured way to pass around the request to produce a complex resource
                     ///
                    pub enum ComplexResourceRequest{
                         $([<$result >]( $lhs, $rhs ), )*
                    }
            }

            ///
            /// Given a [`ComplexResourceType`] returns the underlying function that generate the [`ComplexResource`]
            ///
            pub(crate) fn get_combination_fn( result: ComplexResourceType ) -> CombinationFn {
                match (result ) {
                    $(

                         ComplexResourceType::$result => {
                            paste! {
                                CombinationFn::[<$result Fn>]([<  $result:lower _fn >])
                            }
                        },
                    )*
                }
            }
        };
    }

    define_resources!(
        Basic: [Oxygen , Hydrogen, Carbon, Silicon],
        Complex: [Diamond, Water , Life , Robot , Dolphine , AIPartner]
    );

    define_combination_rules!(
        Water from Hydrogen + Oxygen,
        Diamond from Carbon + Carbon,
        Life from Water + Carbon ,
        Robot from Silicon + Life ,
        Dolphine from Water + Life ,
        AIPartner from Robot +  Diamond
    );
}

fn main() {}
