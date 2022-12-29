use std::{
    error::Error,
    io::{self, Write},
};

use chem_eq::Equation;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    print!("Input equation: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut buf)?;
    let solved = Equation::new(buf.as_str())?.to_balancer().balance()?;
    println!("solved: {}", solved);

    Ok(())
}
