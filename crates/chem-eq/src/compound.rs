//! Implementation of [`Compound`]

use crate::{Element, State, AVAGADRO_CONSTANT};

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
}
