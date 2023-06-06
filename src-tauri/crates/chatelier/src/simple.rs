//! Simulate changes in concentration for Le Chatelier's Principle.
//! It calculates the end result of changes to the system, no intermediate
//! values.

use chem_eq::{error::ConcentrationNameError, Equation, ReactionQuotient};
use float_cmp::approx_eq;
use thiserror::Error;

/// A simulation of Le Chatelier's Principle.
///
/// It will produce how the system should react to certain changes by calculating
/// end values. It doesn't calculate the intermediate values of a system.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(featue = "serde", derive(serde::Serialize, serde::Deserialize))]
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
                if conc == 0.0 {
                    return Err(AdjustError::ZeroConcentration);
                }
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
        let mut res = Ok(());

        loop {
            let dir = match self.direction_to_favour() {
                Direction::None => break,
                d => d,
            };

            // we're switching direction and moving by smaller increments
            if dir != prev || res.is_err() {
                prev = dir;
                addend /= 2.0;
            }

            res = self.react_addend(addend, dir);
        }
    }

    /// Tries to subtract and add the addend to each compound based on the direction and
    /// size of the addend. Returns [`AddendTooBig`] if the addend couldn't be subtracted
    /// without losing data.
    fn react_addend(&mut self, addend: f32, direction: Direction) -> Result<(), AddendTooBig> {
        match direction {
            Direction::Forward => {
                // assert everything can be subtracted, otherwise addend is too big
                for cmp in self.eq.left_mut() {
                    // do nothing if the subtract doesn't fit
                    if addend.mul_add(-(cmp.coefficient as f32), cmp.concentration) <= 0.0 {
                        return Err(AddendTooBig);
                    }
                }
                for cmp in self.eq.left_mut() {
                    cmp.concentration -= addend * cmp.coefficient as f32;
                }
                for cmp in self.eq.right_mut() {
                    cmp.concentration += addend * cmp.coefficient as f32;
                }

                Ok(())
            }
            Direction::Reverse => {
                // assert everything can be subtracted, otherwise addend is too big
                for cmp in self.eq.right_mut() {
                    if addend.mul_add(-(cmp.coefficient as f32), cmp.concentration) <= 0.0 {
                        return Err(AddendTooBig);
                    }
                }
                for cmp in self.eq.right_mut() {
                    cmp.concentration -= addend * cmp.coefficient as f32;
                }
                for cmp in self.eq.left_mut() {
                    cmp.concentration += addend * cmp.coefficient as f32;
                }

                Ok(())
            }
            Direction::None => Ok(()),
        }
    }

    /// Which direction the equation should go, based on k_expr and the system's goal k_expr
    fn direction_to_favour(&self) -> Direction {
        match self.eq.reaction_quotient() {
            ReactionQuotient::BothSidesZero => {
                panic!("both sides of equation have concentration of 0")
            }
            ReactionQuotient::LeftZero => Direction::Reverse,
            ReactionQuotient::RightZero => Direction::Forward,
            ReactionQuotient::Val(f) if approx_eq!(f32, self.k_expr, f) => Direction::None,
            ReactionQuotient::Val(f) if self.k_expr > f => Direction::Forward,
            ReactionQuotient::Val(_) => Direction::Reverse,
        }
    }

    /// Returns which the direction a given adjustment will cause the equilibrium to shift
    pub fn get_shift_direction(&self, adjust: Adjustment) -> Result<Direction, AdjustError> {
        match adjust {
            Adjustment::Concentration(cmp, conc) => {
                if conc == 0.0 {
                    return Err(AdjustError::ZeroConcentration);
                }

                // update the one concentration
                let mut sys = self.clone();
                sys.eq.set_concentration_by_name(cmp, conc)?;

                // get direction and return
                let res = sys.direction_to_favour();

                Ok(res)
            }
            Adjustment::Temperature(_) => todo!("shift temperature"),
            Adjustment::Volume(_) => todo!("shift volume"),
        }
    }

    /// Move system to match k expression
    pub fn update(&mut self) {
        self.react_to_match_k();
    }

    /// Set the k expression
    pub fn get_k_expr(&self) -> f32 {
        self.k_expr
    }

    /// Multiply the k expression
    pub fn mul_k_expr(&mut self, v: f32) {
        self.k_expr *= v;
    }

    /// Set the k expression
    pub fn set_k_expr(&mut self, k: f32) {
        self.k_expr = k;
    }

    /// Get the internal equation
    pub const fn equation(&self) -> &Equation {
        &self.eq
    }

    /// Get mutable access to the internal equation
    pub fn equation_mut(&mut self) -> &mut Equation {
        &mut self.eq
    }
}

/// An change to a [`System`]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Adjustment<'a> {
    /// Change in the temperature, passing the new value
    Temperature(f32),
    /// Change in volume, passing the new value
    Volume(f32),
    /// Change in concentration, passing the name and new value
    Concentration(&'a str, f32),
}

/// The direction of an equilibrium to shift
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Direction {
    /// The equilibrium shifts to the forward direction or to the right
    Forward,
    /// The equilibrium shifts to the reverse direction or to the left
    Reverse,
    /// The equilibrium will not shift
    #[default]
    None,
}

/// An error on using [`System`]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SystemError {
    /// The reaction was not reversible
    #[error("equation doesn't have a reversible reaction")]
    NotReversible,
    #[error("a concentration is zero, the system isn't at equilibrium")]
    ConcentrationIsZero,
}

/// An error on using [`System`]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AdjustError {
    /// The reaction was not reversible
    #[error("concentration not adjusted: {0:?}")]
    CompoundNotFound(#[from] ConcentrationNameError),
    #[error("tried to set concentration to 0M")]
    ZeroConcentration,
}

/// The addend given was too big
#[derive(Debug, Error, Clone, Copy)]
#[error("the addend is too large, reduce it")]
struct AddendTooBig;

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

    #[test]
    fn adjust_concentration_no_trunctate() {
        let mut eq = Equation::new("2NH3(g) <-> N2(g) + 3H2(g)").unwrap();
        eq.set_concentrations(&[2.0, 1.0, 1.5]).unwrap();
        let mut system = System::new(eq).unwrap();
        assert_eq!(system.equation().equilibrium_constant(), Some(0.84375));

        system
            .adjust(Adjustment::Concentration("3H2(g)", 1.59))
            .unwrap();
        assert_eq!(system.equation().equilibrium_constant(), Some(0.8437499));

        assert_eq!(
            system
                .equation()
                .concentrations()
                .copied()
                .collect::<Vec<_>>(),
            vec![2.0399098, 0.98004514, 1.5301354]
        );
    }
}
