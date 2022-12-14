//! # `chatelier`
//!
//! Types to simulate Le Chatelier's Principle

use std::{fmt, time::Duration};

use chem_eq::Equation;

/// A simulation of Le Chatelier's Principle.
///
/// It will produce how the system should react to certain changes
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_inspector_egui::Inspectable))]
pub struct System {
    eq: Equation,
    goal_k: f32,
}

impl System {
    /// Construct a [`System`]
    pub fn new(eq: Equation, goal_k: f32) -> Result<Self, SystemError> {
        if !matches!(eq.direction(), chem_eq::Direction::Reversible) {
            return Err(SystemError::NotReversible);
        }
        Ok(Self { eq, goal_k })
    }

    /// Take initial [`Equation`] and bring it to equilibrium, return time to reach initial
    /// reaction
    pub fn init(&mut self, time_per_reaction: Duration) -> Duration {
        // react until k-expr matches the goal
        // increment time by `time_per_reaction`
        let mut time_taken = Duration::default();
        loop {
            let dir = match self.direction_to_favour() {
                Direction::None => break,
                d => d,
            };
            self.react(dir);
            time_taken += time_per_reaction;
        }

        time_taken
    }

    /// React in one direction or another
    fn react(&mut self, direction: Direction) {
        match direction {
            Direction::Forward => {}
            Direction::Reverse => {}
            Direction::None => {}
        }
    }

    /// Take a transformation to the reaction, return time to reach new values
    pub fn adjust(&mut self, _adjust: Adjustment) -> Duration {
        todo!("adjust")
    }

    /// Continue reacting simulation without any changes, return time to reach new values
    pub fn continue_sim(&mut self) -> Duration {
        todo!("continue")
    }

    /// Which direction the equation should go, based on k_expr and the system's goal k_expr
    fn direction_to_favour(&self) -> Direction {
        let cur_k_expr = self.eq.k_expr();
        if self.goal_k == cur_k_expr {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemError {
    /// The reaction was not reversible
    NotReversible,
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NotReversible => "Equation doesn't have a reversible reaction",
            }
        )
    }
}

impl std::error::Error for SystemError {}
#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! system_eq {
        ($eq:literal, $goal_k:expr) => {
            System::new(Equation::new($eq).unwrap(), $goal_k)
        };
    }

    #[test]
    fn init_sim() {
        assert_eq!(
            system_eq!("H2 + O2 -> H2O", 1.0),
            Err(SystemError::NotReversible)
        );
    }
}
