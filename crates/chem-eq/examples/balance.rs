use std::{io::{self, Write}, error::Error};

use chem_eq::{Equation, balance::EquationBalancer};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    print!("Input equation: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut buf)?;
    let eq = Equation::new(buf.as_str())?;

    let solver: EquationBalancer = eq.into();
    let solved = solver.balance();
    println!("solved: {}", solved);

    Ok(())
}
