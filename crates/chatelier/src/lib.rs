//! # `chatelier`
//!
//! Types to simulate Le Chatelier's Principle

use std::time::Duration;

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
#[cfg_attr(feature = "bevy", derive(bevy_inspector_egui::Inspectable))]
pub struct System {
    eq: Equation,
    goal_k: f32,
    time_per_reaction: Duration,
}

impl System {
    /// Construct a [`System`]
    pub fn new(
        eq: Equation,
        goal_k: f32,
        time_per_reaction: Duration,
    ) -> Result<Self, SystemError> {
        if !matches!(eq.direction(), chem_eq::Direction::Reversible) {
            return Err(SystemError::NotReversible);
        }
        Ok(Self {
            eq,
            goal_k,
            time_per_reaction,
        })
    }

    /// Take initial [`Equation`] and bring it to equilibrium, return time to reach initial
    /// reaction
    pub fn init(&mut self) -> Duration {
        // react until k-expr matches the goal
        self.time_till_k_matches()
    }

    /// React until the goal k and equation k match:
    ///
    /// Find final k by reacting using direction and change in concentration per 
    /// reaction and comparing current k and final k to tell if we are done.
    fn time_till_k_matches(&mut self) -> Duration {
        let mut time_taken = Duration::default();
        loop {
            let dir = match self.direction_to_favour() {
                Direction::None => break,
                d => d,
            };

            self.react(dir);
            time_taken += self.time_per_reaction * NORMALIZE_FACTOR as _;
        }

        time_taken
    }

    /// React in one direction or another
    fn react(&mut self, direction: Direction) {
        let volume = self.eq.volume().unwrap_or(1.0);
        match direction {
            Direction::Forward => {
                for cmp in self.eq.left_mut() {
                    cmp.add_unit(volume, -(NORMALIZE_FACTOR * cmp.coefficient as isize));
                }
                for cmp in self.eq.right_mut() {
                    cmp.add_unit(volume, NORMALIZE_FACTOR * cmp.coefficient as isize);
                }
            }
            Direction::Reverse => {
                for cmp in self.eq.left_mut() {
                    cmp.add_unit(volume, NORMALIZE_FACTOR * cmp.coefficient as isize);
                }
                for cmp in self.eq.right_mut() {
                    cmp.add_unit(volume, -(NORMALIZE_FACTOR * cmp.coefficient as isize));
                }
            }
            Direction::None => {}
        }
    }

    /// Take a transformation to the reaction, return time to reach new values
    pub fn adjust(&mut self, adjust: Adjustment) -> Result<Duration, AdjustError> {
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
                Ok(self.time_till_k_matches())
            }
        }
    }

    /// Continue reacting simulation without any changes, return time to reach new values
    pub fn continue_sim(&mut self) -> Duration {
        todo!("continue")
    }

    /// Which direction the equation should go, based on k_expr and the system's goal k_expr
    fn direction_to_favour(&self) -> Direction {
        let Some(cur_k_expr) = self.eq.k_expr() else {
            let left_is_zero = self.eq.left().iter().map(|c| c.concentration).any(|c| c == 0.0);
            let right_is_zero = self.eq.right().iter().map(|c| c.concentration).any(|c| c == 0.0);
            if left_is_zero && right_is_zero {
                panic!("Concentration on both sides of equation are 0")
            } else if left_is_zero {
                return Direction::Reverse;
            } else if right_is_zero {
                return Direction::Forward;
            } else {
                unreachable!("k-expr returns none when either a product or reactant is none")
            }
        };

        // for (name, cnc) in self.eq.name_and_concentration() {
        //     eprintln!("{:4} = {}", name, cnc);
        // }

        if approx_eq!(f32, self.goal_k, cur_k_expr, ulps = 7) {
            Direction::None
        } else if self.goal_k > cur_k_expr {
            Direction::Forward
        } else {
            Direction::Reverse
        }
    }

    /// Get the internal equation
    pub fn equation(&self) -> &Equation {
        &self.eq
    }

    /// Getter for the goal k-expr
    pub fn goal_k(&self) -> f32 {
        self.goal_k
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
    #[error("Equation doesn't have a reversible reaction")]
    NotReversible,
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
        ($eq:literal, $goal_k:expr, $micros:expr) => {
            System::new(
                Equation::new($eq).unwrap(),
                $goal_k,
                Duration::from_micros($micros),
            )
        };
    }

    #[test]
    fn init_sim() {
        assert_eq!(
            system_eq!("H2 + O2 -> H2O", 1.0, 10),
            Err(SystemError::NotReversible)
        );
    }
}
