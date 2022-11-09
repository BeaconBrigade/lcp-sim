//! # `chem-eq`
//!
//! `chem-eq` parses chemical equations into elements, mol ration,
//! direction of reaction and more.
//!

use std::str::FromStr;

use nom::error::Error;

mod parse;
mod display;

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

    /// Get the mol ration of the equation (left over right).
    pub fn mol_ratio(&self) -> f64 {
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
        left as f64 / right as f64
    }
}
