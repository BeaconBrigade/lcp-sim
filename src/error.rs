use std::fmt;

use chem_eq::error::{BalanceError, EquationError};

#[derive(Debug, Clone)]
pub enum Error {
    ChemEq(EquationError),
    Balance(BalanceError),
    NotEquilibrium,
    WaitingForEquation,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ChemEq(EquationError::ParsingError(_)) =>
                    "The equation could not be parsed".to_string(),
                Self::ChemEq(EquationError::IncorrectEquation) =>
                    "The equation is invalid".to_string(),
                Error::Balance(e) => e.to_string(),
                Self::NotEquilibrium => "Not a reversible reaction".to_string(),
                Self::WaitingForEquation => "Waiting for equation...".to_string(),
            }
        )
    }
}

impl std::error::Error for Error {}

impl From<EquationError> for Error {
    fn from(e: EquationError) -> Self {
        Self::ChemEq(e)
    }
}

impl From<BalanceError> for Error {
    fn from(e: BalanceError) -> Self {
        Self::Balance(e)
    }
}
