//! Implementation for [`Element`]

use mendeleev::{Element as MendeleevElement, ALL_ELEMENTS};

use crate::{error::ElementError, parse};

/// Smaller version of an element that's parsed from an equation
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SimpleElement {
    /// The name of the element
    /// Eg. Fe3 will have a name Fe
    pub name: String,
    /// The amount of the element
    /// Eg. H2 will have a count of 2
    pub count: usize,
}

impl SimpleElement {
    pub(crate) fn into_element(self) -> Result<Element, ElementError> {
        Element::new(self)
    }
}

/// An individual element. Containing an element from the periodic table
/// and the count of how many there are.
///
/// Eg: O2
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Element {
    /// Chemical properties and information about this element
    #[cfg_attr(feature = "serde", serde(skip))]
    el: &'static MendeleevElement,
    /// How many of this element there are.
    /// In O2 the count will be 2 and in 2NO3 it will be 3
    pub count: usize,
}

impl std::ops::Deref for Element {
    type Target = MendeleevElement;

    fn deref(&self) -> &'static Self::Target {
        self.el
    }
}

impl Element {
    /// Construct an [`Element`] using a [`SimpleElement`]
    pub(crate) fn new(sim: SimpleElement) -> Result<Self, ElementError> {
        // check if element is valid
        let Some(elm) = ALL_ELEMENTS.iter().find(|n| n.symbol() == sim.name.as_str()) else {
            return Err(ElementError::NotInPeriodicTable(sim.name));
        };

        Ok(Self {
            el: elm,
            count: sim.count,
        })
    }

    /// Parse an element from a str
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::{Element, error::ElementError};
    ///
    /// let el = Element::parse("O2");
    /// assert!(el.is_ok());
    ///
    /// let el = Element::parse("H2O");
    /// assert_eq!(el.unwrap_err(), ElementError::TooMuchInput("O".to_string()));
    /// ```
    pub fn parse(input: &str) -> Result<Self, ElementError> {
        match parse::parse_element(input) {
            Ok((i, eq)) if i.trim().is_empty() => Ok(eq),
            Ok((i, _)) => Err(ElementError::TooMuchInput(i.to_string())),
            Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
                Err(ElementError::ParseError(e.into()))
            }
            // no streaming parsers were used
            Err(nom::Err::Incomplete(_)) => unreachable!(),
        }
    }
}

impl TryFrom<SimpleElement> for Element {
    type Error = ElementError;

    fn try_from(s: SimpleElement) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl Default for Element {
    fn default() -> Self {
        Self {
            el: &MendeleevElement::H,
            count: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_element() {
        let simple = SimpleElement {
            name: "Pb".to_string(),
            count: 2,
        };
        assert!(simple.into_element().is_ok());
    }

    #[test]
    fn invalid_element() {
        let simple = SimpleElement {
            name: "Bill".to_string(),
            count: 0xCAFE,
        };
        assert_eq!(
            simple.into_element(),
            Err(ElementError::NotInPeriodicTable("Bill".to_string()))
        )
    }
}
