//! Balance a chemical equation
//!
//!

use std::collections::HashMap;

use crate::Equation;
use ndarray::prelude::*;
use num::{Integer, Rational64, Signed, Zero};

/// Takes an equation and balances it.
///
/// ## Examples
///
/// ```rust
/// use chem_eq::{Equation, balance::EquationBalancer};
///
/// let eq = Equation::new("H2 + O2 -> H2O").unwrap();
/// let balancer: EquationBalancer = eq.into();
/// let balanced_eq = balancer.balance();
///
/// assert_eq!(balanced_eq.equation, "2H2 + O2 -> 2H2O");
/// ```
#[derive(Debug, Clone)]
pub struct EquationBalancer {
    eq: Equation,
    matrix: Array2<Rational64>,
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
        let mut arr = Array2::<Rational64>::zeros((row, col));

        let mut left_or_right: Rational64 = 1.into();
        // fill in vector with counts of elements
        for (cmp, i) in eq.iter_compounds().zip(0..row) {
            for el in &cmp.elements {
                let index = *uniq_elements.get(el.name.as_str()).unwrap();
                arr[[i, index]] = <i64 as Into<Rational64>>::into(el.count as i64) * left_or_right;
            }
            // invert compounds on the right because they are products.
            // when they're brought to the other side of the equation, (because they start off
            // on the opposite side) the counts will be inverted (as math works).
            if i + 1 >= eq.left.len() {
                left_or_right = Rational64::from_integer(-1);
            }
        }

        Self {
            eq,
            matrix: arr.reversed_axes(),
        }
    }

    pub fn balance(mut self) -> Equation {
        if self.eq.is_balanced() {
            return self.eq;
        }

        let matrix = self.matrix;
        println!("================original_matrix===============\n{}", matrix);
        // reduced row echelon form, or kernel, or null space
        let null_space = rref(augment(rref(matrix.view()).t()).view());
        println!(
            "================null_space====================\n{}",
            null_space
        );
        // last column is the coefficients (as fractions)
        let vec = null_space
            .row(null_space.dim().0 - 1)
            .to_owned()
            .iter()
            .skip_while(|n| *n.numer() == 0)
            .map(Rational64::abs)
            .collect::<Vec<Rational64>>();
        let coef_col = Array1::from_vec(vec);
        println!(
            "================coef_col======================\n{}",
            coef_col
        );

        // get lcm of the denominators of the coefficients to scale them up
        let lcm = coef_col
            .iter()
            .map(Rational64::denom)
            .fold(1, |acc: i64, f| acc.lcm(f));
        println!("==================lcm=======================\n{}", lcm);

        // add the extra one for the free variable
        // let coef_col = {
        //     let mut vec = coef_col.to_vec();
        //     vec.push(1.into());
        //     Array1::from_vec(vec)
        // };
        // scale up the solutions
        let coef_col = coef_col * lcm;
        println!("================coef_col====================\n{}", coef_col);

        // replace the coefficients
        for (compound, coef) in self
            .eq
            .iter_compounds_mut()
            .zip(coef_col.iter().map(Rational64::numer))
        {
            compound.coefficient = *coef as _;
        }

        // replace equation field with correct coefficients
        let mut comp_str: Vec<String> = self
            .eq
            .equation
            .split(' ')
            .filter(|c| !matches!(*c, "+" | "<-" | "<->" | "->"))
            .map(Into::into)
            .collect();
        for (cmp, str) in self.eq.iter_compounds().zip(comp_str.iter_mut()) {
            if cmp.coefficient != 1 {
                str.insert_str(0, cmp.coefficient.to_string().as_str());
            }
        }
        // concatenate compounds with "+" signs
        let reactants = comp_str[..self.eq.left.len()].join(" + ");
        let products = comp_str[self.eq.left.len()..].join(" + ");

        // combine products and reactants with sign in the middle
        self.eq.equation = format!("{} {} {}", reactants, self.eq.direction, products);

        self.eq
    }
}

impl From<Equation> for EquationBalancer {
    /// Create matrix for solving out of equation
    fn from(eq: Equation) -> Self {
        Self::new(eq)
    }
}

// Thanks to u/mindv0rtex on reddit (https://www.reddit.com/r/rust/comments/yqtr7l/comment/iw3axav/?utm_source=share&utm_medium=web2x&context=3)
// playground is here: https://gist.github.com/rust-play/814106443584a5c7b2cbe04a9a6d55ae#file-playground-rs
fn rref(a: ArrayView2<Rational64>) -> Array2<Rational64> {
    let mut out = ArrayBase::zeros(a.raw_dim());
    out.zip_mut_with(&a, |x, y| *x = *y);

    let mut pivot = 0;
    let (rows, cols) = out.raw_dim().into_pattern();

    'outer: for r in 0..rows {
        if cols <= pivot {
            break;
        }
        let mut i = r;
        while (out[[i, pivot]] as Rational64).numer().is_zero() {
            i += 1;
            if i == rows {
                i = r;
                pivot += 1;
                if cols == pivot {
                    break 'outer;
                }
            }
        }
        for j in 0..rows {
            out.swap([r, j], [i, j]);
        }
        let divisor: Rational64 = out[[r, pivot]];
        if !divisor.numer().is_zero() {
            out.row_mut(r).iter_mut().for_each(|e| *e /= divisor);
        }
        for j in 0..rows {
            if j != r {
                let hold = out[[j, pivot]];
                for k in 0..cols {
                    let t = out[[r, k]];
                    out[[j, k]] -= hold * t;
                }
            }
        }
        pivot += 1;
    }

    out
}

fn augment(a: ArrayView2<Rational64>) -> Array2<Rational64> {
    ndarray::concatenate(Axis(1), &[a.view(), Array2::eye(a.shape()[0]).view()]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn balance_simple() {
    //     let solver: EquationBalancer = Equation::new("H2 + O2 -> H2O").unwrap().into();
    //     let eq = solver.balance();
    //     assert_eq!(eq.equation, "2H2 + O2 -> 2H2O");
    // }
    //
    // #[test]
    // fn balance_simple_backwards() {
    //     let solver: EquationBalancer = Equation::new("O2 + H2 -> H2O").unwrap().into();
    //     let eq = solver.balance();
    //     assert_eq!(eq.equation, "O2 + 2H2 -> 2H2O");
    // }
    //
    // #[test]
    // fn balance_other_simple() {
    //     let solver: EquationBalancer = Equation::new("Al + O2 -> Al2O3").unwrap().into();
    //     let eq = solver.balance();
    //     assert_eq!(eq.equation, "4Al + 3O2 -> 2Al2O3");
    // }
    //
    // #[test]
    // fn balance_already_done() {
    //     let solver: EquationBalancer = Equation::new("C2H4 + 3O2 -> 2CO2 + 2H2O").unwrap().into();
    //     let eq = solver.balance();
    //     assert_eq!(eq.equation, "C2H4 + 3O2 -> 2CO2 + 2H2O");
    // }

    #[test]
    fn balance_harder() {
        let solver: EquationBalancer = Equation::new("C2H6 + O2 -> CO2 + H2O").unwrap().into();
        let eq = solver.balance();
        assert_eq!(eq.equation, "2C2H6 + 7O2 -> 4CO2 + 6H2O");
    }
}
