use std::str::FromStr;

use itertools::Itertools;

use crate::{
    compound::Compound,
    error::{ConcentrationError, ConcentrationNameError, EquationError},
    parse, Direction, State,
};

// Used for rustdoc
#[cfg(all(doc, feature = "balance"))]
#[allow(unused)]
use crate::balance::EquationBalancer;

/// A Chemical Equation. Containing a left and right side. Also keeps
/// track of the mol ratio.
///
/// Eg: `4Fe + 3O2 -> 2Fe2O3`
#[derive(Debug, Default, Clone, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Equation {
    pub(crate) left: Vec<Compound>,
    pub(crate) right: Vec<Compound>,
    pub(crate) direction: Direction,
    pub(crate) equation: String,
    pub(crate) delta_h: f32,
    pub(crate) temperature: Option<f32>,
    pub(crate) volume: Option<f32>,
}

impl PartialEq for Equation {
    fn eq(&self, other: &Self) -> bool {
        self.left() == other.left()
            && self.right() == other.right()
            && self.direction() == other.direction()
            && self.equation() == other.equation()
            && self.delta_h() == other.delta_h()
            && self.temperature() == other.temperature()
            && self.volume() == other.volume()
    }
}
impl Eq for Equation {}

impl TryFrom<&str> for Equation {
    type Error = EquationError;

    fn try_from(s: &str) -> Result<Self, EquationError> {
        Self::new(s)
    }
}

impl FromStr for Equation {
    type Err = EquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Equation {
    /// Create an [`Equation`] from a [`str`]. Fails if the string couldn't
    /// be parsed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, error::EquationError};
    ///
    /// let eq = Equation::new("2H2 + O2 -> 2H2O");
    /// assert!(eq.is_ok());
    ///
    /// let eq = Equation::new("H2b + bad_name == joe");
    /// assert!(matches!(eq, Err(EquationError::ParsingError(_))));
    /// ```
    pub fn new(input: &str) -> Result<Self, EquationError> {
        match parse::parse_equation(input) {
            Ok((i, _)) if !i.trim().is_empty() => Err(EquationError::TooMuchInput(i.to_string())),
            Ok((_, eq)) if eq.is_valid() => Ok(eq),
            Ok(_) => Err(EquationError::IncorrectEquation),
            Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
                Err(EquationError::ParsingError(e.into()))
            }
            // no streaming parsers were used
            Err(nom::Err::Incomplete(_)) => unreachable!(),
        }
    }

    /// Get the mol ratio of the equation (left over right). Will count any compound
    /// with no specified state.
    ///
    /// # Examples
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
                c.state
                    .as_ref()
                    .map_or(true, |s| matches!(s, State::Aqueous | State::Gas))
            })
            .map(|c| c.coefficient)
            .sum::<usize>();
        let right = self
            .right
            .iter()
            .filter(|c| {
                c.state
                    .as_ref()
                    .map_or(true, |s| matches!(s, State::Aqueous | State::Gas))
            })
            .map(|c| c.coefficient)
            .sum::<usize>();
        if left == 0 && right == 0 {
            (1, 1)
        } else {
            (left, right)
        }
    }

    /// Get a vec of each unique element name
    ///
    /// # Examples
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
            .map(|e| e.symbol())
            .unique()
            .collect::<Vec<&str>>();

        element_names
    }

    /// Count how many compounds are in the whole equation.
    ///
    /// # Examples
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
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, error::EquationError};
    ///
    /// let eq = Equation::new("O2 + 2H2 -> 2H2O");
    /// assert!(eq.is_ok());
    ///
    /// let eq = Equation::new("Fe + S8 -> Fe2O3");
    /// // fails because the equation doesn't have sulfur and oxygen on both sides
    /// assert_eq!(eq, Err(EquationError::IncorrectEquation));
    /// ```
    pub(crate) fn is_valid(&self) -> bool {
        let mut left_elements = self
            .left
            .iter()
            .flat_map(|c| &c.elements)
            .map(|e| e.symbol())
            .unique()
            .collect::<Vec<&str>>();
        let mut right_elements = self
            .right
            .iter()
            .flat_map(|c| &c.elements)
            .map(|e| e.symbol())
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
    /// # Examples
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
    /// # Examples
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

    /// Create a mutable iterator over all compounds of an equation.
    ///
    /// # Examples
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
    /// # Examples
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
                let count = lhs.get(el.symbol()).unwrap_or(&0);
                lhs.insert(el.symbol(), count + el.count * cmp.coefficient);
            }
        }

        // right hand side
        for cmp in &self.right {
            for el in &cmp.elements {
                let count = rhs.get(el.symbol()).unwrap_or(&0);
                rhs.insert(el.symbol(), count + el.count * cmp.coefficient);
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

    /// Create an [`EquationBalancer`], mostly as a convenience method.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    ///
    /// let eq = Equation::new("H2 + O2 -> H2O").unwrap().to_balancer().balance().unwrap();
    /// assert_eq!(eq.equation(), "2H2 + O2 -> 2H2O".to_string());
    /// ```
    #[cfg(feature = "balance")]
    #[cfg_attr(docsrs, doc(cfg(feature = "balance")))]
    pub fn to_balancer(&self) -> crate::balance::EquationBalancer {
        use crate::balance::EquationBalancer;

        EquationBalancer::new(self)
    }

    /// Check whether an equation is exothermic
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    ///
    /// let mut eq = Equation::new("2Mg(s) + O2(g) -> 2MgO(s)").unwrap();
    /// eq.set_delta_h(-601.1);
    /// assert!(eq.is_exothermic());
    /// ```
    pub fn is_exothermic(&self) -> bool {
        self.delta_h() < 0.0
    }

    /// Check whether an equation is endothermic
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    ///
    /// let mut eq = Equation::new("6CO2 + 6H2O -> C6H12O6 + 6O2").unwrap();
    /// eq.set_delta_h(2802.7);
    /// assert!(eq.is_endothermic());
    /// ```
    pub fn is_endothermic(&self) -> bool {
        self.delta_h() > 0.0
    }

    /// Get an iterator over each compounds name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    ///
    /// let eq = Equation::new("H2 + O2 -> H2O").unwrap();
    /// assert_eq!(vec!["H2", "O2", "H2O"], eq.compound_names().collect::<Vec<&str>>());
    ///
    /// let eq = Equation::new("Fe2O3 <- Fe + O2").unwrap();
    /// assert_eq!(vec!["Fe2O3", "Fe", "O2"], eq.compound_names().collect::<Vec<&str>>());
    /// ```
    pub fn compound_names(&self) -> impl Iterator<Item = &str> {
        self.equation()
            .split(' ')
            .filter(|s| !matches!(*s, "+" | "<-" | "<->" | "->"))
    }

    /// Get an iterator for each concentration in an equation
    pub fn concentrations(&self) -> impl Iterator<Item = &f32> {
        self.iter_compounds().map(|cmp| &cmp.concentration)
    }

    /// Get a mutable iterator for each concentration in an equation
    pub fn concentrations_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.iter_compounds_mut().map(|cmp| &mut cmp.concentration)
    }

    /// Get an iterator yielding compound names and concentrations
    pub fn name_and_concentration(&self) -> impl Iterator<Item = (&str, &f32)> {
        self.compound_names().zip(self.concentrations())
    }

    /// Get a mutable iterator yielding compound names and mutable concentrations
    pub fn name_and_concentration_mut(&mut self) -> impl Iterator<Item = (String, &mut f32)> {
        self.equation
            .split(' ')
            .filter(|s| !matches!(*s, "+" | "<-" | "<->" | "->"))
            .map(ToString::to_string)
            .collect_vec()
            .into_iter()
            .zip(self.concentrations_mut())
    }

    /// Get a vec of all concentrations
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    ///
    /// let mut eq = Equation::new("H2 + O2 -> H2O").unwrap();
    /// assert_eq!(eq.get_concentrations(), vec![0.0, 0.0, 0.0]);
    ///
    /// eq.set_concentrations(&[1.0, 2.0, 3.0]);
    /// assert_eq!(eq.get_concentrations(), vec![1.0, 2.0, 3.0]);
    /// ```
    pub fn get_concentrations(&self) -> Vec<f32> {
        self.concentrations().copied().collect()
    }

    /// Set concentrations with a slice. A convenience method to quickly set all
    /// compounds to have a concentration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, error::ConcentrationError};
    ///
    /// let mut eq = Equation::new("H2 + O2 -> H2O").unwrap();
    /// eq.set_concentrations(&[1.0, 2.0, 3.0]).unwrap();
    /// assert_eq!(eq.get_concentrations(), vec![1.0, 2.0, 3.0]);
    ///
    /// assert_eq!(eq.set_concentrations(&[1.0, 34.0]), Err(ConcentrationError::WrongSliceSize));
    /// ```
    pub fn set_concentrations(&mut self, concentrations: &[f32]) -> Result<(), ConcentrationError> {
        // check assumptions
        if concentrations.len() != self.num_compounds() {
            return Err(ConcentrationError::WrongSliceSize);
        }
        if concentrations.iter().any(|&c| c.is_nan()) {
            return Err(ConcentrationError::NAN);
        }

        for (orig, new) in self.concentrations_mut().zip(concentrations.iter()) {
            *orig = *new;
        }

        Ok(())
    }

    /// Get a singular compounds concentration by its name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, error::ConcentrationNameError};
    ///
    /// let mut eq = Equation::new("H2 + O2 -> H2O").unwrap();
    /// eq.set_concentration_by_name("O2", 0.25).unwrap();
    /// assert_eq!(eq.get_concentration_by_name("O2"), Ok(0.25));
    ///
    /// assert_eq!(eq.get_concentration_by_name("joe"), Err(ConcentrationNameError::NotFound));
    /// ```
    pub fn get_concentration_by_name(&self, name: &str) -> Result<f32, ConcentrationNameError> {
        // I don't like the collecting here...
        // but I can't avoid double borrowing self as mutable and immutable
        let (_name, cmp) = self
            .compound_names()
            .map(ToString::to_string)
            .collect_vec()
            .into_iter()
            .zip(self.iter_compounds())
            .find(|(n, _c)| *n == name)
            .ok_or(ConcentrationNameError::NotFound)?;
        Ok(cmp.concentration)
    }

    /// Get a compound by name
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, Compound};
    ///
    /// let mut eq = Equation::new("H2 + O2 -> H2O").unwrap();
    /// let cmp = eq.get_compound_by_name("O2");
    /// assert_eq!(cmp.unwrap(), &Compound::parse("O2").unwrap());
    ///
    /// assert!(eq.get_compound_by_name("joe").is_none());
    /// ```
    pub fn get_compound_by_name(&self, name: &str) -> Option<&Compound> {
        self.compound_names()
            .zip(self.iter_compounds())
            .find(|(n, _c)| *n == name)
            .map(|(_n, c)| c)
    }

    /// Get a mutable reference to a compound by name
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, Compound};
    ///
    /// let mut eq = Equation::new("H2 + O2 -> H2O").unwrap();
    /// let cmp = eq.get_compound_by_name_mut("O2");
    /// assert_eq!(cmp.unwrap(), &mut Compound::parse("O2").unwrap());
    ///
    /// assert!(eq.get_compound_by_name("joe").is_none());
    /// ```
    pub fn get_compound_by_name_mut(&mut self, name: &str) -> Option<&mut Compound> {
        let i = self
            .compound_names()
            .enumerate()
            .find(|(_i, n)| *n == name)
            .map(|(i, _n)| i)?;
        self.nth_compound_mut(i)
    }

    /// Set a singular compounds concentration by its name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, error::ConcentrationNameError};
    ///
    /// let mut eq = Equation::new("H2 + O2 -> H2O").unwrap();
    /// eq.set_concentration_by_name("O2", 0.25).unwrap();
    /// assert_eq!(eq.get_concentrations(), vec![0.0, 0.25, 0.0]);
    ///
    /// assert_eq!(eq.set_concentration_by_name("joe", 24.0), Err(ConcentrationNameError::NotFound));
    /// assert_eq!(eq.set_concentration_by_name("H2O", f32::NAN), Err(ConcentrationNameError::NAN));
    /// ```
    pub fn set_concentration_by_name(
        &mut self,
        name: &str,
        concentration: f32,
    ) -> Result<(), ConcentrationNameError> {
        if concentration.is_nan() {
            return Err(ConcentrationNameError::NAN);
        }
        // I don't like the collecting here...
        // but I can't avoid double borrowing self as mutable and immutable
        let (_name, cmp) = self
            .compound_names()
            .map(ToString::to_string)
            .collect_vec()
            .into_iter()
            .zip(self.iter_compounds_mut())
            .find(|(n, _c)| *n == name)
            .ok_or(ConcentrationNameError::NotFound)?;
        cmp.concentration = concentration;
        Ok(())
    }

    /// Get the k expression or Kc of an equation. Returns [`None`] if one side
    /// has a compound with a concentration of 0
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    ///
    /// let eq = Equation::new("H2 + O2 -> H2O").unwrap();
    /// // is nan because all compounds have an initial concentration of 0M
    /// assert!(eq.equilibrium_constant().is_none());
    ///
    /// let mut eq = Equation::new("N2 + 2O2 -> 2NO2").unwrap();
    /// eq.set_concentrations(&[0.25, 0.50, 0.75]).unwrap();
    /// assert_eq!(eq.equilibrium_constant().unwrap(), (0.75 * 0.75) / (0.25 * 0.5 * 0.5));
    /// ```
    pub fn equilibrium_constant(&self) -> Option<f32> {
        match self.reaction_quotient() {
            ReactionQuotient::Val(q) => Some(q),
            _ => None,
        }
    }

    /// Get Qc of the equation, called the reaction coefficient. It's
    /// calculated the same way as the k-expression, however it can apply to non
    /// equilibrium concentrations.
    ///
    /// ## Returns
    ///
    /// Assuming the hypothetical reaction where a, b, c, and d are the
    /// coefficents of A, B, C and D respectively:
    /// ```text
    /// aA + bB <-> cC + dD
    /// ```
    ///
    /// Since `Qc = ([C]^c * [D]^d) / ([A]^a * [B]^b)`,
    /// This function will return:
    /// - [`ReactionQuotient::BothSidesZero`] if `[C]^c * [D]^d` and `[A]^a * [B]^b` are both 0
    /// - [`ReactionQuotient::LeftZero`] if `[A]^a * [B]^b` is 0
    /// - [`ReactionQuotient::RightZero`] if `[C]^c * [D]^d` is 0
    /// - otherwise the result of the above equation contained in [`ReactionQuotient::Val`]
    pub fn reaction_quotient(&self) -> ReactionQuotient {
        // skip compounds that are solid or liquid
        let left = self
            .left
            .iter()
            .filter(|c| matches!(c.state, Some(State::Aqueous | State::Gas) | None))
            .fold(1.0, |acc, cmp| {
                acc * cmp.concentration.powf(cmp.coefficient as f32)
            });

        let right = self
            .right
            .iter()
            .filter(|c| matches!(c.state, Some(State::Aqueous | State::Gas) | None))
            .fold(1.0, |acc, cmp| {
                acc * cmp.concentration.powf(cmp.coefficient as f32)
            });

        if left == 0.0 && right == 0.0 {
            ReactionQuotient::BothSidesZero
        } else if right == 0.0 {
            ReactionQuotient::RightZero
        } else if left == 0.0 {
            ReactionQuotient::LeftZero
        } else {
            ReactionQuotient::Val(right / left)
        }
    }

    /// Get the nth compound of the equation.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, Compound};
    ///
    /// let eq = Equation::new("H2 + O2 -> H2O").unwrap();
    /// assert_eq!(eq.nth_compound(0).unwrap(), &Compound::parse("H2").unwrap());
    /// assert_eq!(eq.nth_compound(1).unwrap(), &Compound::parse("O2").unwrap());
    /// assert_eq!(eq.nth_compound(2).unwrap(), &Compound::parse("H2O").unwrap());
    /// assert!(eq.nth_compound(3).is_none());
    /// ```
    pub fn nth_compound(&self, idx: usize) -> Option<&Compound> {
        if idx < self.left.len() {
            Some(&self.left[idx])
        } else if idx < self.left.len() + self.right.len() {
            Some(&self.right[idx - self.left.len()])
        } else {
            None
        }
    }

    /// Get the nth compound of the equation mutably.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::{Equation, Compound};
    ///
    /// let mut eq = Equation::new("H2 + O2 -> H2O").unwrap();
    /// assert_eq!(eq.nth_compound_mut(0).unwrap(), &mut Compound::parse("H2").unwrap());
    /// assert_eq!(eq.nth_compound_mut(1).unwrap(), &mut Compound::parse("O2").unwrap());
    /// assert_eq!(eq.nth_compound_mut(2).unwrap(), &mut Compound::parse("H2O").unwrap());
    /// assert!(eq.nth_compound_mut(3).is_none());
    /// ```
    pub fn nth_compound_mut(&mut self, idx: usize) -> Option<&mut Compound> {
        if idx < self.left.len() {
            Some(&mut self.left[idx])
        } else if idx < self.left.len() + self.right.len() {
            Some(&mut self.right[idx - self.left.len()])
        } else {
            None
        }
    }

    /// Getter for the left side of the equation.
    pub fn left(&self) -> &[Compound] {
        self.left.as_ref()
    }

    /// Mut getter for the right side of the equation
    pub fn left_mut(&mut self) -> &mut Vec<Compound> {
        &mut self.left
    }

    /// Getter for the right side of the equation
    pub fn right(&self) -> &[Compound] {
        self.right.as_ref()
    }

    /// Mut getter for the right side of the equation
    pub fn right_mut(&mut self) -> &mut Vec<Compound> {
        &mut self.right
    }

    /// Getter for the direction of the equation
    pub const fn direction(&self) -> &Direction {
        &self.direction
    }

    /// Getter for the equation as text
    pub fn equation(&self) -> &str {
        self.equation.as_ref()
    }

    /// Getter for `delta_h` in kJ
    pub const fn delta_h(&self) -> f32 {
        self.delta_h
    }

    /// Setter for `delta_h` in kJ
    pub fn set_delta_h(&mut self, delta_h: f32) {
        self.delta_h = delta_h;
    }

    /// Getter for `temperature` in degrees Celsius
    pub const fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    /// Setter for `temperature` in degrees Celsius
    pub fn set_temperature(&mut self, temperature: f32) {
        self.temperature = Some(temperature);
    }

    /// Getter for the `volume` equation in litres
    pub const fn volume(&self) -> Option<f32> {
        self.volume
    }

    /// Setter for `volume` in Litres
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = Some(volume);
    }
}

/// The result of [`Equation::reaction_quotient`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReactionQuotient {
    /// The product of the concentration of the reactants was zero
    LeftZero,
    /// The product of the concentration of the products was zero
    RightZero,
    /// Both the reactant and products had product concentrations of zero
    BothSidesZero,
    /// The reaction quotient
    Val(f32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_much_input() {
        assert_eq!(
            Equation::new("H2 + O2 -> H2O-2wowthisistoolong"),
            Err(EquationError::TooMuchInput(
                "-2wowthisistoolong".to_string()
            ))
        )
    }

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
    fn mol_ratio_no_aq() {
        // doesn't matter how bad an equation this is...
        // this one is _really_ bad though...
        let eq = Equation::new("Fe(s) + K2(s) -> FeK(l)").unwrap();
        assert_eq!(eq.mol_ratio(), (1, 1));
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
