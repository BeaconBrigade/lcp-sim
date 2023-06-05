use std::time::Duration;

use chem_eq::{Direction, Equation};

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContinuousSystem {
    eq: Equation,
    k_f: f32,
    k_r: f32,
}

impl ContinuousSystem {
    /// Construct a [`System`] with any equation passed. Make sure to initialize initial
    /// concentrations for each element.
    pub fn new(eq: Equation, k_f: f32, k_r: f32) -> Self {
        Self { eq, k_f, k_r }
    }

    /// Using the initial concentrations and rate constant calculate how the concentration changes
    /// over time. `time_step` is the frequency we calculate a change in concentrations. The
    /// simulation will use the millisecond value. `stop` is how long the simulation runs in milliseconds.
    ///
    /// The return value contains a vector for each compounds concentration over time. So if:
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use chem_eq::Equation;
    /// use chatelier::continuous::ContinuousSystem;
    /// use std::time::Duration;
    ///
    /// let mut equation = Equation::new("2H2 + O2 -> 2H2O").unwrap();
    /// equation.set_concentrations(&[0.12, 0.3, 0.0]).unwrap();
    /// let mut system = ContinuousSystem::new(equation, 1.0, 0.0);
    ///
    /// let concentration_over_time = system.solve(Duration::from_millis(1), Duration::from_secs(40));
    ///
    /// for (cnc, name) in concentration_over_time.into_iter().zip(system.eq().compound_names()) {
    ///     println!("{name}: {cnc:?}");
    /// }
    /// ```
    pub fn solve(&mut self, time_step: Duration, stop: Duration) -> Vec<Vec<f32>> {
        let time_steps = (0_usize..stop.as_millis().try_into().unwrap())
            .step_by(time_step.as_millis().try_into().unwrap());
        let length = (stop.as_millis() as usize) / (time_step.as_millis() as usize);
        let time_step = time_step.as_millis() as f32 / 1000.0;
        println!("time_step: {time_step}");

        // allocate memory
        let mut data = Vec::with_capacity(self.eq.num_compounds());
        let mut prev_concentrations = Vec::with_capacity(self.eq.num_compounds());
        for idx in 0..self.eq.num_compounds() {
            let mut cnc = Vec::with_capacity(length);
            let c = self.eq.nth_compound(idx).unwrap().concentration;
            cnc.push(c);
            data.push(cnc);
            prev_concentrations.push(c);
        }

        let left_len = self.eq.left().len();
        for _ in time_steps {
            // forward and reverse reaction rates
            let (rf, rr) = calculate_rate(&self.eq, self.k_f, self.k_r, &prev_concentrations);
            // left side
            for (i, cmp) in data[..left_len].iter_mut().enumerate() {
                let coef = self.eq.nth_compound(i).unwrap().coefficient as f32;
                cmp.push(
                    prev_concentrations[i] - (coef * rf * time_step) + (coef * rr * time_step),
                );
                prev_concentrations[i] = *cmp.last().unwrap();
            }
            // right side
            for (i, cmp) in data[left_len..].iter_mut().enumerate() {
                let i = i + left_len;
                let coef = self.eq.nth_compound(i).unwrap().coefficient as f32;
                cmp.push(
                    prev_concentrations[i] + (coef * rf * time_step) - (coef * rr * time_step),
                );
                prev_concentrations[i] = *cmp.last().unwrap();
            }
        }

        data
    }

    pub fn eq(&self) -> &Equation {
        &self.eq
    }

    pub fn eq_mut(&mut self) -> &mut Equation {
        &mut self.eq
    }
}

fn calculate_rate(eq: &Equation, k_f: f32, k_r: f32, prev_concentrations: &[f32]) -> (f32, f32) {
    let left_len = eq.left().len();
    match eq.direction() {
        Direction::Right => {
            let side = &prev_concentrations[0..left_len];
            let r = side.iter().enumerate().fold(k_f, |acc, (i, c)| {
                acc * c.powi(eq.nth_compound(i).unwrap().coefficient.try_into().unwrap())
            });
            (r, 0.0)
        }
        Direction::Left => {
            let side = &prev_concentrations[left_len..];
            let r = side.iter().enumerate().fold(k_r, |acc, (i, c)| {
                acc * c.powi(eq.nth_compound(i).unwrap().coefficient.try_into().unwrap())
            });
            (0.0, r)
        }
        Direction::Reversible => {
            let rf = {
                let side = &prev_concentrations[0..left_len];
                let r = side.iter().enumerate().fold(k_f, |acc, (i, c)| {
                    acc * c.powi(eq.nth_compound(i).unwrap().coefficient.try_into().unwrap())
                });
                r
            };

            let rr = {
                let side = &prev_concentrations[left_len..];
                let r = side.iter().enumerate().fold(k_r, |acc, (i, c)| {
                    acc * c.powi(eq.nth_compound(i).unwrap().coefficient.try_into().unwrap())
                });
                r
            };

            (rf, rr)
        }
    }
}
