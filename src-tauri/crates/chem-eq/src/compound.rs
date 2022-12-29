//! Implementation of [`Compound`]

use crate::{error::CompoundError, parse, Element, State, AVAGADRO_CONSTANT};

/// An inidiviual compound. Containing some elements and a coefficient.
///
/// Eg: 2Fe2O3
#[derive(Debug, Default, Clone, PartialOrd)]
pub struct Compound {
    /// The elements of a compound
    pub elements: Vec<Element>,
    /// The coefficient of the whole compound
    pub coefficient: usize,
    /// The state of the compound
    pub state: Option<State>,
    /// The concentration in M (mol/L) of the compound
    pub concentration: f32,
}

impl PartialEq for Compound {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
            && self.coefficient == other.coefficient
            && self.state == other.state
            && self.concentration == other.concentration
    }
}

impl Compound {
    /// Get the formula units, atoms or molecules of a compound
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, AVAGADRO_CONSTANT};
    ///
    /// let mut eq = Equation::new("H2 + O2 -> H2O").unwrap();
    /// eq.set_concentration_by_name("H2", 1.0).unwrap();
    /// eq.set_volume(1.0);
    /// let cmp = eq.iter_compounds().next().unwrap();
    /// assert_eq!(cmp.get_units(eq.volume().unwrap()), AVAGADRO_CONSTANT);
    /// ```
    pub fn get_units(&self, volume: f32) -> f64 {
        // c = n/v
        // n = cv
        let moles = self.concentration * volume;

        // N = nNₐ
        moles as f64 * AVAGADRO_CONSTANT
    }

    /// Parse a compound from str
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::{Compound, error::CompoundError};
    ///
    /// let cmp = Compound::parse("Fe2O3");
    /// assert!(cmp.is_ok());
    ///
    /// let cmp = Compound::parse("Fe2O3 + O2");
    /// assert_eq!(cmp.unwrap_err(), CompoundError::TooMuchInput(" + O2".to_string()));
    /// ```
    pub fn parse(input: &str) -> Result<Self, CompoundError> {
        match parse::parse_compound(input) {
            Ok((i, eq)) if i.trim().is_empty() => Ok(eq),
            Ok((i, _)) => Err(CompoundError::TooMuchInput(i.to_string())),
            Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
                Err(CompoundError::ParsingError(e.into()))
            }
            // no streaming parsers were used
            Err(nom::Err::Incomplete(_)) => unreachable!(),
        }
    }

    /// Add to the formula units, atoms or molecules of a compound
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, AVAGADRO_CONSTANT};
    ///
    /// let mut eq = Equation::new("H2 + O2 -> H2O").unwrap();
    /// eq.set_concentration_by_name("H2", 1.0).unwrap();
    /// eq.set_volume(1.0);
    /// let volume = eq.volume().unwrap();
    /// let cmp = eq.get_compound_by_name_mut("H2").unwrap();
    /// cmp.add_unit(volume, 1);
    /// assert_eq!(cmp.get_units(volume), AVAGADRO_CONSTANT + 1.0);
    /// ```
    pub fn add_unit(&mut self, volume: f32, addend: isize) {
        // we need N
        let units = self.get_units(volume) + addend as f64;

        // N = nNa, n = N/Na
        let moles = units / AVAGADRO_CONSTANT;

        // c = n/v
        self.concentration = moles as f32 / volume;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_unit() {
        let mut cmp = Compound::parse("H2").unwrap();
        cmp.concentration = 1.0;
        let volume = 1.0;
        cmp.add_unit(volume, 1);
        assert_eq!(cmp.get_units(volume), AVAGADRO_CONSTANT + 1.0);
    }

    #[test]
    fn sub_unit() {
        let mut cmp = Compound::parse("H2").unwrap();
        cmp.concentration = 1.0;
        let volume = 1.0;
        cmp.add_unit(volume, -1);
        assert_eq!(cmp.get_units(volume), AVAGADRO_CONSTANT - 1.0);
    }
}
