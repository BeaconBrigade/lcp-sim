use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    ChemEq(chem_eq::Error),
    NotEquilibrium,
    WaitingForEquation,
}

impl From<chem_eq::Error> for Error {
    fn from(e: chem_eq::Error) -> Self {
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
