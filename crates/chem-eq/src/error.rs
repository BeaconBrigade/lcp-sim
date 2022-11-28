use crate::parse::util::Error;
///
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

/// Error for [`Equation::set_concentrations`]
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum ConcentrationError {
    /// Slice lenght doesn't match [`Equation::num_compounds`]
    #[error("Slice doesn't match Equation::num_compounds()")]
    WrongSliceSize,
    /// A concentration value was NAN which is invalid
    #[error("A concentration value was NAN which is invalid")]
    NAN,
}

/// Error for [`Equation::set_concentration_by_name`]
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum ConcentrationNameError<'a> {
    /// Requested compound couldn't be found
    #[error("Compound not found: {0}")]
    NotFound(&'a str),
    /// Concentration value was NAN, which is invalid
    #[error("Concentration value was NAN which is invalid")]
    NAN,
}
