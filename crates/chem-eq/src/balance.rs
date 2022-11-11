//! Balance a chemical equation
//!
//!

// use nalgebra::DMatrix;

use crate::Equation;

#[derive(Debug, Default, Clone)]
pub struct EquationBalancer {
    eq: Equation,
    // matrix: DMatrix<isize>,
}

impl EquationBalancer {
    pub fn new(eq: Equation) -> Self {
        // let matrix =
        Self {
            eq,
            // matrix: Default::default(),
        }
    }

    pub fn balance(self) -> Equation {
        if self.eq.is_balanced() {
            return self.eq;
        }

        todo!()
    }
}

impl From<Equation> for EquationBalancer {
    /// Create matrix for solving out of equation
    fn from(eq: Equation) -> Self {
        // let matrix =

        Self {
            eq,
            // matrix: Default::default(),
        }
    }
}
