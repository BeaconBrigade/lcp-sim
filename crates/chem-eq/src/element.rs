//! Implementation for [`Element`]

use mendeleev::{Element as MendeleevElement, ALL_ELEMENTS};

use crate::error::ElementError;

/// Smaller version of an element that's parsed from an equation
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SimpleElement {
    /// The name of the element
    /// Eg. Fe3 will have a name Fe
    pub name: String,
    /// The amount of the element
    /// Eg. H2 will have a count of 2
    pub count: usize,
}

#[cfg(test)]
impl SimpleElement {
    pub fn into_element(self) -> Result<Element, ElementError> {
        Element::new(self)
    }
}

/// An individual element. Containing an element from the periodic table
/// and the count of how many there are.
///
/// Eg: O2
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Element {
    /// Chemical properties and information about this element
    pub el: &'static MendeleevElement,
    /// How many of this element there are.
    /// In O2 the count will be 2 and in 2NO3 it will be 3
    pub count: usize,
}

impl Element {
    /// Construct an [`Element`] using a [`SimpleElement`]
    pub(crate) fn new(sim: SimpleElement) -> Result<Self, ElementError> {
        // check if element is valid
        let Some(elm) = ALL_ELEMENTS.iter().find(|n| n.symbol() == sim.name.as_str()) else {
            return Err(ElementError::NotInPeriodicTable);
        };

        Ok(Element {
            el: elm,
            count: sim.count,
        })
    }
}

impl TryFrom<SimpleElement> for Element {
    type Error = ElementError;

    fn try_from(s: SimpleElement) -> Result<Self, Self::Error> {
        Self::new(s)
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
        assert_eq!(simple.into_element(), Err(ElementError::NotInPeriodicTable))
    }
}
