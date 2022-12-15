//! Error types for `chem-eq`

use crate::parse::util::Error;

/// Errors type for issues with chemical equations
#[derive(thiserror::Error, Clone, PartialEq, Eq)]
pub enum EquationError {
    /// The string couldn't be parsed into a chemical equation
    #[error("couldn't parse the equation:\n{0}")]
    ParsingError(Error<String>),
    /// The equation is not valid. Eg: There are different elements on each side of the equation
    #[error("this equation is not valid")]
    IncorrectEquation,
    /// The compound was parsed, but there was remaining input
    #[error("too much input, remaining: {0:?}")]
    TooMuchInput(String),
}

impl std::fmt::Debug for EquationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

// done for rustdoc
#[cfg(doc)]
#[allow(unused)]
use crate::Compound;

/// Errors for parsing a [`Compound`]
#[derive(thiserror::Error, Clone, PartialEq, Eq)]
pub enum CompoundError {
    /// The input couldn't be parsed into a compound
    #[error("couldn't parse the compound:\n{0}")]
    ParsingError(Error<String>),
    /// The compound was parsed, but there was remaining input
    #[error("too much input, remaining: {0:?}")]
    TooMuchInput(String),
}

impl std::fmt::Debug for CompoundError {
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
    #[error("slice not right size")]
    WrongSliceSize,
    /// A concentration value was NAN which is invalid
    #[error("concentration value was NAN")]
    NAN,
}

/// Error for [`Equation::set_concentration_by_name`] and [`Equation::get_concentration_by_name`]
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum ConcentrationNameError<'a> {
    /// Requested compound couldn't be found
    #[error("compound not found: {0}")]
    NotFound(&'a str),
    /// Concentration value was NAN, which is invalid
    #[error("concentration value was NAN")]
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
    #[error("the equation is invalid")]
    InvalidEquation,
    /// The equation could not be balanced
    #[error("equation could not be balanced")]
    Infeasable,
}

#[cfg(doc)]
#[allow(unused)]
use crate::Element;

/// Error for constructing an [`Element`]
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum ElementError {
    /// This element is not a member of the periodic table
    #[error("Element was not part of periodic table: {0}")]
    NotInPeriodicTable(String),
    /// The input could no be parsed into an element
    #[error("The element could not be parsed")]
    ParseError(Error<String>),
    /// The element was parsed, but there was remaining input
    #[error("too much input, remaining: {0:?}")]
    TooMuchInput(String),
}
