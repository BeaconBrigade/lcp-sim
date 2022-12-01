//! Implementation of [`Compound`]

use crate::{Element, State};

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

impl Compound {}
