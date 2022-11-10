//! Balance a chemical equation
//!
//!

use nalgebra::Matrix;

use crate::Equation;

#[derive(Debug, Default, Clone)]
pub struct EquationBalancer {
    eq: Equation,
    matrix: Matrix<isize>,
}

impl EquationBalancer {
    pub fn new(eq: Equation) -> Self {
        Self {
            eq,
            matrix: Default::default(),
        }
    }
}

impl From<Equation> for EquationBalancer {
    /// Create matrix for solving out of equation
    fn from(eq: &Equation) -> Self {

        // let matrix = 

        Default::default()
    }
}

