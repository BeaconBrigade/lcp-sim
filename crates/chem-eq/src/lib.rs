//! # `chem-eq`
//!
//! `chem-eq` parses chemical equations into elements, mol ratio,
//! direction of reaction and more.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

use std::str::FromStr;

use itertools::Itertools;

#[cfg(feature = "balance")]
#[cfg_attr(docsrs, doc(cfg(feature = "balance")))]
pub mod balance;
mod display;
mod parse;

/// Errors type for issues with chemical equations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    /// The string couldn't be parsed into a chemical equation
    ParsingError,
    /// The equation is not valid. Eg: There are different elements on each side of the equation
    IncorrectEquation,
}

/// A Chemical Equation. Containing a left and right side. Also keeps
/// track of the mol ratio.
///
/// Eg: 4Fe + 3O2 -> 2Fe2O3
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Equation {
    pub left: Vec<Compound>,
    pub right: Vec<Compound>,
    pub direction: Direction,
    pub equation: String,
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
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    ///
    /// let eq = Equation::new("2H2 + O2 -> 2H2O");
    /// assert!(eq.is_ok());
    ///
    /// let eq = Equation::new("H2b + bad_name == joe");
    /// assert!(eq.is_err());
    /// ```
    pub fn new(input: &str) -> Result<Self, Error> {
        let (_, eq) = parse::parse_equation(input).map_err(|_| Error::ParsingError)?;
        if eq.is_valid() {
            Ok(eq)
        } else {
            Err(Error::IncorrectEquation)
        }
    }

    /// Get the mol ratio of the equation (left over right). Will count any compound
    /// with no specified state.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    ///
    /// // returns left over right
    /// // if states aren't given, everything is counted
    /// let eq = Equation::new("2H2 + O2 -> 2H2O").unwrap();
    /// assert_eq!(eq.mol_ratio(), (3, 2));
    ///
    /// // doesn't matter how bad an equation this is...
    /// let eq = Equation::new("4FeH3(s) + 3O2(g) -> 2Fe2O3(s) + 6H2(g)").unwrap();
    /// assert_eq!(eq.mol_ratio(), (3, 6));
    /// ```
    pub fn mol_ratio(&self) -> (usize, usize) {
        let left = self
            .left
            .iter()
            .filter(|c| {
                c.state.as_ref().map_or(true, |s| matches!(s, State::Aqueous | State::Gas))
            })
            .map(|c| c.coefficient)
            .sum::<usize>();
        let right = self
            .right
            .iter()
            .filter(|c| {
                c.state.as_ref().map_or(true, |s| matches!(s, State::Aqueous | State::Gas))
            })
            .map(|c| c.coefficient)
            .sum::<usize>();
        (left, right)
    }

    /// Get the number of unique elements in the equation
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    ///
    /// let eq = Equation::new("2O2 + H2 -> 2H2O").unwrap();
    /// assert_eq!(eq.uniq_elements().len(), 2);
    ///
    /// let eq =
    ///     Equation::new("3(NH4)2SO4(aq) + Fe3(PO4)2(s) <- 2(NH4)3PO4(aq) + 3FeSO4(aq)").unwrap();
    /// assert_eq!(eq.uniq_elements().len(), 6);
    /// ```
    pub fn uniq_elements(&self) -> Vec<&str> {
        // get the name of every element in the equation
        let element_names = self
            .iter_compounds()
            .flat_map(|c| &c.elements)
            .map(|e| e.name.as_str())
            .unique()
            .collect::<Vec<&str>>();

        element_names
    }

    /// Count how many compounds are in the whole equation.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    ///
    /// let eq = Equation::new("O2 + 2H2 -> 2H2O").unwrap();
    /// assert_eq!(eq.num_compounds(), 3);
    ///
    /// let eq = Equation::new("3(NH4)2SO4(aq) + Fe3(PO4)2(s) <- 2(NH4)3PO4(aq) + 3FeSO4(aq)").unwrap();
    /// assert_eq!(eq.num_compounds(), 4);
    /// ```
    pub fn num_compounds(&self) -> usize {
        self.left.len() + self.right.len()
    }

    /// Check if an equation is valid.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, Error};
    ///
    /// let eq = Equation::new("O2 + 2H2 -> 2H2O");
    /// assert!(eq.is_ok());
    ///
    /// let eq = Equation::new("Fe + S8 -> Fe2O3");
    /// // fails because the equation doesn't have sulfur or oxygen on both sides
    /// assert_eq!(eq, Err(Error::IncorrectEquation));
    /// ```
    fn is_valid(&self) -> bool {
        let mut left_elements = self
            .left
            .iter()
            .flat_map(|c| &c.elements)
            .map(|e| e.name.as_str())
            .unique()
            .collect::<Vec<&str>>();
        let mut right_elements = self
            .right
            .iter()
            .flat_map(|c| &c.elements)
            .map(|e| e.name.as_str())
            .unique()
            .collect::<Vec<&str>>();

        // sort to make sure comparisons work
        left_elements.sort_unstable();
        right_elements.sort_unstable();

        // simple verification that the same elements are on both sides
        left_elements == right_elements
    }

    /// Reconstruct original equation without using the saved original string.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    ///
    /// let eq = Equation::new("O2 + H2 -> H2O").unwrap();
    /// assert_eq!(eq.reconstruct(), "1O2 + 1H2 -> 1H2O1");
    /// ```
    pub fn reconstruct(&self) -> String {
        format!(
            "{} {} {}",
            self.left
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" + "),
            self.direction,
            self.right
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" + "),
        )
    }

    /// Create an iterator over all compounds of an equation
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, Compound};
    ///
    /// let eq = Equation::new("O2 + H2 -> H2O").unwrap();
    /// assert_eq!(eq.iter_compounds().collect::<Vec<&Compound>>().len(), 3);
    /// ```
    // Mostly as a convenience method as this appears in multiple places
    pub fn iter_compounds(&self) -> impl Iterator<Item = &Compound> {
        self.left.iter().chain(self.right.iter())
    }

    /// Create an iterator over all compounds of an equation
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, Compound};
    ///
    /// let mut eq = Equation::new("O2 + H2 -> H2O").unwrap();
    /// assert_eq!(eq.iter_compounds_mut().collect::<Vec<&mut Compound>>().len(), 3);
    /// ```
    // Mostly as a convenience method as this appears in multiple places
    pub fn iter_compounds_mut(&mut self) -> impl Iterator<Item = &mut Compound> {
        self.left.iter_mut().chain(self.right.iter_mut())
    }

    /// Check if the equation is balanced
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    ///
    /// let eq = Equation::new("C + 2H2O -> CO2 + 2H2").unwrap();
    /// assert!(eq.is_balanced());
    ///
    /// let eq = Equation::new("Mg(OH)2 + Fe -> Fe(OH)3 + Mg").unwrap();
    /// assert!(!eq.is_balanced());
    /// ```
    #[cfg(feature = "balance")]
    #[cfg_attr(docsrs, doc(cfg(feature = "balance")))]
    pub fn is_balanced(&self) -> bool {
        use std::collections::HashMap;
        let mut lhs: HashMap<&str, usize> = HashMap::default();
        let mut rhs: HashMap<&str, usize> = HashMap::default();

        // left hand side
        for cmp in &self.left {
            for el in &cmp.elements {
                let count = lhs.get(el.name.as_str()).unwrap_or(&0);
                lhs.insert(el.name.as_str(), count + el.count * cmp.coefficient);
            }
        }

        // right hand side
        for cmp in &self.right {
            for el in &cmp.elements {
                let count = rhs.get(el.name.as_str()).unwrap_or(&0);
                rhs.insert(el.name.as_str(), count + el.count * cmp.coefficient);
            }
        }

        // different amount of elements
        lhs.len() == rhs.len()
            && lhs.keys().all(|k| {
                if rhs.contains_key(k) {
                    return lhs.get(k).unwrap() == rhs.get(k).unwrap();
                }
                false
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mol_ratio_basic() {
        let eq = Equation::new("2O2 + H2 -> H2O").unwrap();
        assert_eq!(eq.mol_ratio(), (3, 1));
    }

    #[test]
    fn mol_ratio_states() {
        let eq = Equation::new("2O2(g) + H2(g) -> H2O(l)").unwrap();
        assert_eq!(eq.mol_ratio(), (3, 0));
    }

    #[test]
    fn mol_ratio_more_states() {
        // doesn't matter how bad an equation this is...
        let eq = Equation::new("4FeH3(s) + 3O2(g) -> 2Fe2O3(s) + 6H2(g)").unwrap();
        assert_eq!(eq.mol_ratio(), (3, 6));
    }

    #[test]
    fn uniq_elements_no_repeat() {
        let eq = Equation::new("2O2 + H2 -> 2H2O").unwrap();
        assert_eq!(eq.uniq_elements().len(), 2);
    }

    #[test]
    fn uniq_elements_repeat() {
        let eq = Equation::new("C + 2H2O -> CO2 + 2H2").unwrap();
        assert_eq!(eq.uniq_elements().len(), 3);
    }

    #[test]
    fn uniq_long() {
        let eq =
            Equation::new("3(NH4)2SO4(aq) + Fe3(PO4)2(s) <- 2(NH4)3PO4(aq) + 3FeSO4(aq)").unwrap();
        assert_eq!(eq.uniq_elements().len(), 6);
    }

    #[test]
    fn num_compounds_short() {
        let eq = Equation::new("O2 + 2H2 -> 2H2O").unwrap();
        assert_eq!(eq.num_compounds(), 3);
    }

    #[test]
    fn num_compounds_long() {
        let eq =
            Equation::new("3(NH4)2SO4(aq) + Fe3(PO4)2(s) <- 2(NH4)3PO4(aq) + 3FeSO4(aq)").unwrap();
        assert_eq!(eq.num_compounds(), 4);
    }

    #[test]
    #[cfg(feature = "balance")]
    fn is_balanced_correct() {
        let eq = Equation::new("C + 2H2O -> CO2 + 2H2").unwrap();
        assert!(eq.is_balanced());
    }

    #[test]
    #[cfg(feature = "balance")]
    fn is_balanced_incorrect() {
        let eq = Equation::new("Mg(OH)2 + Fe -> Fe(OH)3 + Mg").unwrap();
        assert!(!eq.is_balanced());
    }
}
