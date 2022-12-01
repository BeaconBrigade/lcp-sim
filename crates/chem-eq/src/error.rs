//! Error types for `chem-eq`

use crate::parse::util::Error;

/// Errors type for issues with chemical equations
#[derive(thiserror::Error, Clone, PartialEq, Eq)]
pub enum EquationError {
    /// The string couldn't be parsed into a chemical equation
    #[error("Couldn't parse the equation: {0}")]
    ParsingError(Error<String>),
    /// The equation is not valid. Eg: There are different elements on each side of the equation
    #[error("This equation is not valid")]
    IncorrectEquation,
}

impl std::fmt::Debug for EquationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

// done for rustdoc
#[cfg(doc)]
#[allow(unused)]
use crate::Equation;

/// Error for [`Equation::set_concentrations`]
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum ConcentrationError {
    /// Slice length doesn't match [`Equation::num_compounds`]
    #[error("Slice doesn't match number of compunds")]
    WrongSliceSize,
    /// A concentration value was NAN which is invalid
    #[error("A concentration value was NAN which is invalid")]
    NAN,
}

/// Error for [`Equation::set_concentration_by_name`] and [`Equation::get_concentration_by_name`]
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum ConcentrationNameError<'a> {
    /// Requested compound couldn't be found
    #[error("Compound not found: {0}")]
    NotFound(&'a str),
    /// Concentration value was NAN, which is invalid
    #[error("Concentration value was NAN which is invalid")]
    NAN,
}

// done for rustdoc
#[cfg(all(doc, feature = "balance"))]
#[allow(unused)]
use crate::balance::EquationBalancer;

/// Error for [`EquationBalancer::balance`]
#[cfg(feature = "balance")]
#[cfg_attr(docsrs, doc(cfg(feature = "balance")))]
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum BalanceError {
    /// The equation was invalid
    #[error("The equation is invalid")]
    InvalidEquation,
    /// The equation could not be balanced
    #[error("Equation could not be balanced")]
    Infeasable,
}
