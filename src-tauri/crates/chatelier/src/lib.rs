//! # `chatelier`
//!
//! Types to simulate Le Chatelier's Principle

use chem_eq::{error::ConcentrationNameError, Equation};
use float_cmp::approx_eq;
use thiserror::Error;

/// Normalizing number to make number modifications notisable. Too big numbers
/// cause the simulation to take __way__ too long.
pub const NORMALIZE_FACTOR: isize = 1_000_000_000;

/// A simulation of Le Chatelier's Principle.
///
/// It will produce how the system should react to certain changes
#[derive(Debug, Default, Clone, PartialEq)]
pub struct System {
    eq: Equation,
    k_expr: f32,
}

impl System {
    /// Construct a [`System`], passing an equation at equilibrium
    pub fn new(eq: Equation) -> Result<Self, SystemError> {
        if !matches!(eq.direction(), chem_eq::Direction::Reversible) {
            return Err(SystemError::NotReversible);
        }
        let k_expr = eq
            .equilibrium_constant()
            .ok_or(SystemError::ConcentrationIsZero)?;
        Ok(Self { eq, k_expr })
    }

    /// Take a transformation to the reaction, return time to reach new values
    pub fn adjust(&mut self, adjust: Adjustment) -> Result<(), AdjustError> {
        match adjust {
            Adjustment::Temperature(_tmp) => {
                // change time-per-reaction
                // - increase = lower time-per-reaction
                // - decrease = higher time-per-reaction
                //
                // a change of around 10 degrees Celsius doubles reactionr rates
                todo!("adjust temperature")
            }
            Adjustment::Volume(_vol) => {
                // if mol ratio is 1:1 or equivalent do nothing otherwise have
                // to figure out how much to shift to either side
                todo!("adjust volume")
            }
            Adjustment::Concentration(cmp, conc) => {
                // update the one concentration
                self.eq.set_concentration_by_name(cmp, conc)?;

                // shift until the k-expr matches again
                self.react_to_match_k();

                Ok(())
            }
        }
    }

    /// Continue to react until the Kc matches Qc
    fn react_to_match_k(&mut self) {
        // number to modify concentrations by
        let mut addend = 1.0;
        let mut prev = self.direction_to_favour();

        loop {
            let dir = match self.direction_to_favour() {
                Direction::None => break,
                d => d,
            };

            // we're switching direction and moving by smaller increments
            if dir != prev {
                prev = dir;
                addend /= 2.0;
            }

            self.react_addend(addend, dir);
        }
    }

    fn react_addend(&mut self, addend: f32, direction: Direction) {
        match direction {
            Direction::Forward => {
                for cmp in self.eq.left_mut() {
                    if cmp.concentration < addend {
                        cmp.concentration = 0.0;
                    } else {
                        cmp.concentration -= addend * cmp.coefficient as f32;
                    }
                }
                for cmp in self.eq.right_mut() {
                    cmp.concentration += addend * cmp.coefficient as f32;
                }
            }
            Direction::Reverse => {
                for cmp in self.eq.left_mut() {
                    cmp.concentration += addend * cmp.coefficient as f32;
                }
                for cmp in self.eq.right_mut() {
                    if cmp.concentration < addend {
                        cmp.concentration = 0.0;
                    } else {
                        cmp.concentration -= addend * cmp.coefficient as f32;
                    }
                }
            }
            Direction::None => {}
        }
    }

    /// Which direction the equation should go, based on k_expr and the system's goal k_expr
    fn direction_to_favour(&self) -> Direction {
        let q_c = self.eq.reaction_quotient();
        if q_c.is_nan() {
            panic!("both sides of equation have concentration of 0")
        } else if approx_eq!(f32, self.k_expr, q_c, ulps = 5) {
            Direction::None
        } else if q_c.is_infinite() || self.k_expr > q_c {
            Direction::Forward
        } else {
            Direction::Reverse
        }
    }

    /// Get the internal equation
    pub fn equation(&self) -> &Equation {
        &self.eq
    }
}

/// An change to a [`System`]
#[derive(Debug, Clone, Copy)]
pub enum Adjustment<'a> {
    /// Change in the temperature, passing the new value
    Temperature(f32),
    /// Change in volume, passing the new value
    Volume(f32),
    /// Change in concentration, passing the name and new value
    Concentration(&'a str, f32),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Forward,
    Reverse,
    #[default]
    None,
}

/// An error on using [`System`]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemError {
    /// The reaction was not reversible
    #[error("equation doesn't have a reversible reaction")]
    NotReversible,
    #[error("a concentration is zero, the system isn't at equilibrium")]
    ConcentrationIsZero,
}

/// An error on using [`System`]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjustError {
    /// The reaction was not reversible
    #[error("concentration not adjusted: {0:?}")]
    CompoundNotFound(#[from] ConcentrationNameError),
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! system_eq {
        ($eq:literal) => {
            System::new(Equation::new($eq).unwrap())
        };
    }

    #[test]
    fn init_sim() {
        assert_eq!(
            system_eq!("H2 + O2 -> H2O"),
            Err(SystemError::NotReversible)
        );
    }

    #[test]
    fn adjust_concentration() {
        let mut eq = Equation::new("SO2 + NO2 <-> NO + SO3").unwrap();
        eq.set_concentrations(&[2.0, 1.0, 2.0, 2.0]).unwrap();

        let mut system = System::new(eq).unwrap();
        assert_eq!(system.equation().equilibrium_constant(), Some(2.0));

        system
            .adjust(Adjustment::Concentration("SO3", 3.0))
            .unwrap();

        assert_eq!(system.equation().equilibrium_constant(), Some(2.0));

        assert_eq!(
            system
                .equation()
                .concentrations()
                .copied()
                .collect::<Vec<_>>(),
            vec![2.1789083, 1.1789083, 1.8210917, 2.8210917]
        );
    }
}
