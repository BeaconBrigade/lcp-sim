//! # `chem-eq`
//!
//! `chem-eq` parses chemical equations into elements, mol ration,
//! direction of reaction and more.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

use std::str::FromStr;

use nom::error::Error;

#[cfg(feature = "balance")]
#[cfg_attr(docsrs, doc(cfg(feature = "balance")))]
pub mod balance;
mod display;
mod parse;

/// A Chemical Equation. Containing a left and right side. Also keeps
/// track of the mol ratio.
///
/// Eg: 4Fe + 3O2 -> 2Fe2O3
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Equation {
    pub left: Vec<Compound>,
    pub right: Vec<Compound>,
    pub direction: Direction,
    original_equation: String,
}

/// An inidiviual compound. Containing some elements and a coefficient.
///
/// Eg: 2Fe2O3
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Compound {
    pub elements: Vec<Element>,
    pub coefficient: usize,
    pub state: Option<State>,
}

/// An individual element. Containing an element from the periodic table
/// and the count of how many there are.
///
/// Eg: O2
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Element {
    pub name: String,
    pub count: usize,
}

/// The state of matter of a Compound. Including:
/// - solid
/// - liquid
/// - gas
/// - aqueous
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    Solid,
    Liquid,
    Gas,
    #[default]
    Aqueous,
}

impl FromStr for State {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(Self::Solid),
            "l" => Ok(Self::Liquid),
            "g" => Ok(Self::Gas),
            "aq" => Ok(Self::Aqueous),
            _ => Err("Invalid state."),
        }
    }
}

/// Direction a reaction is heading in.
/// - left
/// - right
/// - equilibrium
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    /// Products are on the left, reactants are on the right.
    Left,
    /// Products are on the right, reactants on the left.
    #[default]
    Right,
    /// Reaction can work in both directions.
    Reversible,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<-" => Ok(Self::Left),
            "->" => Ok(Self::Right),
            "<->" => Ok(Self::Reversible),
            _ => Err("Invalid direction."),
        }
    }
}

impl Equation {
    /// Create an [`Equation`] from a [`str`]. Fails if the string couldn't
    /// be parsed.
    pub fn new(input: &str) -> Result<Self, nom::Err<Error<&str>>> {
        let (_, eq) = parse::parse_equation(input)?;
        Ok(eq)
    }

    /// Get the mol ratio of the equation (left over right).
    pub fn mol_ratio(&self) -> (usize, usize) {
        let left = self
            .left
            .iter()
            .filter(|c| {
                if let Some(s) = &c.state {
                    matches!(s, State::Aqueous | State::Gas)
                } else {
                    true
                }
            })
            .map(|c| c.coefficient)
            .sum::<usize>();
        let right = self
            .right
            .iter()
            .filter(|c| {
                if let Some(s) = &c.state {
                    matches!(s, State::Aqueous | State::Gas)
                } else {
                    true
                }
            })
            .map(|c| c.coefficient)
            .sum::<usize>();
        (left, right)
    }

    /// Get the number of unique elements in the equation
    #[cfg(feature = "balance")]
    #[cfg_attr(docsrs, doc(cfg(feature = "balance")))]
    pub fn uniq_elements(&self) -> usize {
        // get the name of every element in the equation
        let mut element_names = self
            .left
            .iter()
            .chain(self.right.iter())
            .flat_map(|c| &c.elements)
            .map(|e| e.name.as_str())
            .collect::<Vec<&str>>();

        // shouldn't be a performance concern since most equations are very short
        element_names.sort();
        element_names.dedup();

        element_names.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniq_elements_no_repeat() {
        let eq = Equation::new("2O2 + H2 -> 2H2O").unwrap();
        assert_eq!(eq.uniq_elements(), 2);
    }

    #[test]
    fn uniq_elements_repeat() {
        let eq = Equation::new("C + 2H2O -> CO2 + 2H2").unwrap();
        assert_eq!(eq.uniq_elements(), 3);
    }

    #[test]
    fn uniq_long() {
        let eq =
            Equation::new("3(NH4)2SO4(aq) + Fe3(PO4)2(s) <- 2(NH4)3PO4(aq) + 3FeSO4(aq)").unwrap();
        assert_eq!(eq.uniq_elements(), 6);
    }
}
