//! Implementation of [`Compound`]

use crate::{error::CompoundError, parse, Element, State, AVAGADRO_CONSTANT};

/// An inidiviual compound. Containing some elements and a coefficient.
///
/// Eg: 2Fe2O3
#[derive(Debug, Default, Clone, PartialOrd)]
#[cfg_attr(feature = "bevy", derive(bevy_inspector_egui::Inspectable))]
pub struct Compound {
    /// The elements of a compound
    #[cfg_attr(feature = "bevy", inspectable(collapse))]
    pub elements: Vec<Element>,
    /// The coefficient of the whole compound
    #[cfg_attr(feature = "bevy", inspectable(collapse))]
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
}
