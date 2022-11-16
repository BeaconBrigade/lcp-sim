use std::fmt::Display;

use crate::{Compound, Direction, Element, Equation, State};

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.original_equation,
        )
    }
}

impl Display for Compound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match &self.state {
            Some(s) => format!("{}", s),
            None => Default::default(),
        };
        let mut elms = String::default();
        for el in &self.elements {
            elms.push_str(el.to_string().as_str())
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
            State::Solid => "s",
            State::Liquid => "l",
            State::Gas => "g",
            State::Aqueous => "aq",
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
