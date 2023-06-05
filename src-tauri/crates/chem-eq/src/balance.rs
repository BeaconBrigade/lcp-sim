//! Balance a chemical equation
//!
//!

use std::collections::HashMap;

use crate::{error::BalanceError, Equation};
use ndarray::prelude::*;
use num::{Integer, Rational64, Signed, Zero};

/// Takes an equation and balances it.
///
/// # Examples
///
/// ```rust
/// use chem_eq::{Equation, balance::EquationBalancer};
///
/// let eq = Equation::new("H2 + O2 -> H2O").unwrap();
/// let balancer = EquationBalancer::new(&eq);
/// let balanced_eq = balancer.balance().unwrap();
///
/// assert_eq!(balanced_eq.equation(), "2H2 + O2 -> 2H2O");
/// ```
#[derive(Debug, Clone)]
pub struct EquationBalancer<'a> {
    eq: &'a Equation,
    matrix: Array2<Rational64>,
}

impl<'a> EquationBalancer<'a> {
    /// Create an equation balancer of a given [`Equation`]
    pub fn new(eq: &'a Equation) -> Self {
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
                let index = *uniq_elements.get(el.symbol()).unwrap();
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

    /// Balance the internal equation consuming self and returning the balanced form.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, balance::EquationBalancer};
    ///
    /// let eq = Equation::new("Fe + O2 -> Fe2O3").unwrap();
    /// let solver = EquationBalancer::new(&eq);
    /// let solved = solver.balance().unwrap();
    ///
    /// assert_eq!(solved.equation(), "4Fe + 3O2 -> 2Fe2O3");
    /// ```
    pub fn balance(self) -> Result<Equation, BalanceError> {
        if !self.eq.is_valid() {
            return Err(BalanceError::InvalidEquation);
        }
        if self.eq.is_balanced() {
            return Ok(self.eq.clone());
        }
        let mut eq = self.eq.clone();

        let matrix = self.matrix;
        // reduced row echelon form, or kernel, or null space
        let null_space = rref(augment(rref(matrix.view()).t()).view());

        // last column is the coefficients (as fractions)
        let vec = null_space
            .row(null_space.dim().0 - 1)
            .to_owned()
            .iter()
            .skip_while(|n| *n.numer() == 0)
            .map(Rational64::abs)
            .collect::<Vec<Rational64>>();
        let coef_col = Array1::from_vec(vec);

        // get lcm of the denominators of the coefficients to scale them up
        let lcm = coef_col
            .iter()
            .map(Rational64::denom)
            .fold(1, |acc: i64, f| acc.lcm(f));

        // scale up the solutions
        let coef_col = coef_col * lcm;
        if coef_col.to_vec().contains(&Rational64::from_integer(0)) {
            return Err(BalanceError::Infeasable);
        }

        // replace the coefficients
        for (compound, coef) in eq
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
        for (cmp, str) in eq.iter_compounds().zip(comp_str.iter_mut()) {
            if cmp.coefficient != 1 {
                let mut to_remove = 0;
                for c in str.chars() {
                    if c.is_numeric() {
                        to_remove += 1;
                    } else {
                        break;
                    }
                }
                for _ in 0..to_remove {
                    str.remove(0);
                }
                str.insert_str(0, cmp.coefficient.to_string().as_str());
            }
        }
        // concatenate compounds with "+" signs
        let reactants = comp_str[..eq.left.len()].join(" + ");
        let products = comp_str[eq.left.len()..].join(" + ");

        // combine products and reactants with sign in the middle
        eq.equation = format!("{} {} {}", reactants, eq.direction, products);

        Ok(eq)
    }
}

impl<'a> From<&'a Equation> for EquationBalancer<'a> {
    /// Create matrix for solving out of equation
    fn from(eq: &'a Equation) -> Self {
        Self::new(eq)
    }
}

// Thanks to u/mindv0rtex on reddit, @mindv0rtex on github
// reduced row echelon form
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
        for j in 0..cols {
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

// ...
fn augment(a: ArrayView2<Rational64>) -> Array2<Rational64> {
    ndarray::concatenate(Axis(1), &[a.view(), Array2::eye(a.shape()[0]).view()]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance_simple() {
        let eq = Equation::new("H2 + O2 -> H2O")
            .unwrap()
            .to_balancer()
            .balance()
            .unwrap();
        assert_eq!(eq.equation, "2H2 + O2 -> 2H2O");
    }

    #[test]
    fn balance_simple_backwards() {
        let eq = Equation::new("O2 + H2 -> H2O")
            .unwrap()
            .to_balancer()
            .balance()
            .unwrap();
        assert_eq!(eq.equation, "O2 + 2H2 -> 2H2O");
    }

    #[test]
    fn balance_other_simple() {
        let eq = Equation::new("Al + O2 -> Al2O3")
            .unwrap()
            .to_balancer()
            .balance()
            .unwrap();
        assert_eq!(eq.equation, "4Al + 3O2 -> 2Al2O3");
    }

    #[test]
    fn balance_already_done() {
        let eq = Equation::new("C2H4 + 3O2 -> 2CO2 + 2H2O")
            .unwrap()
            .to_balancer()
            .balance()
            .unwrap();
        assert_eq!(eq.equation, "C2H4 + 3O2 -> 2CO2 + 2H2O");
    }

    #[test]
    fn balance_harder() {
        let eq = Equation::new("C2H6 + O2 -> CO2 + H2O")
            .unwrap()
            .to_balancer()
            .balance()
            .unwrap();
        assert_eq!(eq.equation, "2C2H6 + 7O2 -> 4CO2 + 6H2O");
    }

    #[test]
    fn try_balance_infeasible() {
        let res = Equation::new("K4Fe(CN)6 + K2S2O3 -> CO2 + K2SO4 + NO2 + FeS")
            .unwrap()
            .to_balancer()
            .balance();
        assert_eq!(res, Err(BalanceError::Infeasable));
    }

    #[test]
    fn try_balance_coefs_already_exist() {
        let res = Equation::new("H2 + I -> 2HI")
            .unwrap()
            .to_balancer()
            .balance()
            .unwrap();
        assert_eq!(res.equation(), "H2 + 2I -> 2HI");
    }

    #[test]
    fn try_balance_coefs_already_exist_two() {
        let res = Equation::new("N2 + H <-> 2NH3")
            .unwrap()
            .to_balancer()
            .balance()
            .unwrap();
        assert_eq!(res.equation(), "N2 + 6H <-> 2NH3");
    }
}
