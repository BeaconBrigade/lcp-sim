//! # `chem-eq`
//!
//! `chem-eq` parses chemical equations into elements, mol ratio,
//! direction of reaction and more.
//!
//! The main type is [`Equation`]
//!

#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]

use std::str::FromStr;

pub use crate::{compound::Compound, element::Element, equation::Equation};

#[cfg(feature = "balance")]
#[cfg_attr(docsrs, doc(cfg(feature = "balance")))]
pub mod balance;
mod compound;
mod display;
mod element;
mod equation;
pub mod error;
mod parse;

/// Avagadro's number, approximately equal to 6.02 * 10^23
pub const AVAGADRO_CONSTANT: f64 = 6.02214e23; 

/// The state of matter of a Compound. Including:
/// - solid
/// - liquid
/// - gas
/// - aqueous
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    /// Solid
    Solid,
    /// Liquid
    Liquid,
    /// Gas
    Gas,
    /// Aqueous
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
/// - reversible
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
