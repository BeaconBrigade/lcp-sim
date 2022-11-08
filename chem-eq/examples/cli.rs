use std::io::{self, Write};

use chem_eq::Equation;

fn main() -> io::Result<()> {
    let mut input = String::default();
    print!("Input equation: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let eq = Equation::new(input.as_str()).unwrap();
    println!("eq = {:?}", eq);
    println!("back = {}", eq);

    Ok(())
}
