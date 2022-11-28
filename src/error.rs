use std::fmt;

use chem_eq::error::EquationError;

#[derive(Debug, Clone)]
pub enum Error {
    ChemEq(EquationError),
    NotEquilibrium,
    WaitingForEquation,
}

impl From<EquationError> for Error {
    fn from(e: EquationError) -> Self {
        Self::ChemEq(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ChemEq(e) => e.to_string(),
                Self::NotEquilibrium => "Not an equilibrium".to_string(),
                Self::WaitingForEquation => "Waiting for equation...".to_string(),
            }
        )
    }
}

impl std::error::Error for Error {}
