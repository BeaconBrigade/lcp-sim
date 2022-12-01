//! Implementation for [`Element`]

/// An individual element. Containing an element from the periodic table
/// and the count of how many there are.
///
/// Eg: O2
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Element {
    /// The name of the element
    /// Eg. Fe3 will have a name Fe
    pub name: String,
    /// The amount of the element
    /// Eg. H2 will have a count of 2
    pub count: usize,
}
