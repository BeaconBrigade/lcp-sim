//! Balance a chemical equation
//!
//!

use std::collections::HashMap;

use fraction::{prelude::*, Ratio, ToPrimitive};
use peroxide::prelude::*;

use crate::Equation;

#[derive(Debug, Clone)]
pub struct EquationBalancer {
    eq: Equation,
    matrix: Matrix,
}

impl EquationBalancer {
    pub fn new(eq: Equation) -> Self {
        // map each unique element to a column in the matrix
        let uniq_elements: HashMap<&str, usize> = eq
            .uniq_elements()
            .into_iter()
            .enumerate()
            .map(|(i, e)| (e, i))
            .collect();

        // construct vector with correct sizing
        let row = eq.num_compounds();
        let col = uniq_elements.len();
        let mut raw: Vec<f64> = vec![0.0; row * col];

        let mut left_or_right = 1.0;
        // fill in vector with counts of elements
        for (cmp, i) in eq.left.iter().chain(eq.right.iter()).zip(0..row) {
            let row = &mut raw[i * col..(i + 1) * col];
            for el in &cmp.elements {
                let index = *uniq_elements.get(el.name.as_str()).unwrap();
                row[index] = el.count as f64 * left_or_right;
            }
            // invert compounds on the right because they are products.
            // when they're brought to the other side of the equation, (because they start off
            // on the opposite side) the counts will be inverted (as math works).
            if i + 1 >= eq.left.len() {
                left_or_right = -1.0;
            }
        }

        let matrix = Matrix {
            data: raw,
            row,
            col,
            shape: Row,
        };

        Self {
            eq,
            matrix: matrix.transpose(),
        }
    }

    pub fn balance(self) -> Equation {
        if self.eq.is_balanced() {
            return self.eq;
        }

        // reduced row echelon form
        let reduced_row_echelon_form = self.matrix.rref();
        // println!("{}", reduced_row_echelon_form);

        // column containing coefficients
        let coef_col: Matrix = reduced_row_echelon_form
            .col(reduced_row_echelon_form.col - 1)
            .into();
        println!("{}", &coef_col);

        // lcm to multiply by coef_col
        // need to get lcm of the denominators... you dummy... use `fraction` crate to help
        let lowest_common_multiple = lcm(coef_col
            .as_slice()
            .iter()
            .map(|f| dbg!(Ratio::from_float(*f).unwrap()).denom().clone())
            .collect::<Vec<BigInt>>()
            .as_slice())
        .to_f64()
        .unwrap();

        println!("lcm = {}", lowest_common_multiple);

        // let coef_col = coef_col.as_slice() * lowest_common_multiple;
        // println!("{}", &coef_col);

        todo!()
    }
}

fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    if b == &BigInt::from(0) {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

fn lcm_short(a: &BigInt, b: &BigInt) -> BigInt {
    (a * b) / gcd(a, b)
}

fn lcm(m: &[BigInt]) -> BigInt {
    m.iter().fold(BigInt::from(1), |acc, v| lcm_short(v, &acc))
}

impl From<Equation> for EquationBalancer {
    /// Create matrix for solving out of equation
    fn from(eq: Equation) -> Self {
        Self::new(eq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_short() {
        assert_eq!(gcd(&9.into(), &6.into()), 3.into());
    }

    #[test]
    fn gcd_big_num() {
        assert_eq!(gcd(&323.into(), &456.into()), 19.into());
    }

    #[test]
    fn lcm_short_short_nums() {
        assert_eq!(lcm_short(&5.into(), &7.into()), 35.into());
    }

    #[test]
    fn lcm_short_long_nums() {
        assert_eq!(lcm_short(&234.into(), &6783.into()), 529_074.into());
    }

    #[test]
    fn list_lcm_short() {
        assert_eq!(
            lcm(vec![3.into(), 4.into(), 5.into()].as_slice()),
            60.into()
        );
    }

    #[test]
    fn list_lcm_long() {
        assert_eq!(
            lcm(vec![234.into(), 678.into(), 32.into(), 21.into(), 3.into()].as_slice()),
            2_961_504.into()
        );
    }

    // #[test]
    // fn balance_simple() {
    //     let solver: EquationBalancer = Equation::new("O2 + H2 -> H2O").unwrap().into();
    //     solver.balance();
    // }

    #[test]
    fn balance_harder() {
        let solver: EquationBalancer = Equation::new("C2H6 + O2 -> CO2 + H2O").unwrap().into();
        solver.balance();
    }
}
