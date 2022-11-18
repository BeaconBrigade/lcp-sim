use std::fmt::Display;

use crate::{Compound, Direction, Element, Equation, State};

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.equation)
    }
}

impl Display for Compound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = self.state.as_ref().map_or_else(Default::default, |s| format!("{}", s));
        let mut elms = String::default();
        for el in &self.elements {
            elms.push_str(el.to_string().as_str());
        }
        write!(f, "{}{}{}", self.coefficient, elms, state)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name, self.count)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Solid => "s",
            Self::Liquid => "l",
            Self::Gas => "g",
            Self::Aqueous => "aq",
        };
        write!(f, "({})", s)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Left => "<-",
            Self::Right => "->",
            Self::Reversible => "<->",
        };
        write!(f, "{}", s)
    }
}
