use std::{
    error::Error,
    io::{self, Write},
};

use chem_eq::{balance::EquationBalancer, Equation};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    print!("Input equation: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut buf)?;
    let eq = &Equation::new(buf.as_str())?;

    let solver: EquationBalancer = eq.into();
    let solved = solver.balance();
    println!("solved: {}", solved);

    Ok(())
}
