use std::error::Error;

use chatelier::{Adjustment, Direction};

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Adjust: {}",
        serde_json::to_string(&Adjustment::Concentration("N2", 2.0))?
    );
    println!(
        "Adjust: {}",
        serde_json::to_string(&Adjustment::Temperature(10.0))?
    );
    println!(
        "Adjust: {}",
        serde_json::to_string(&Adjustment::Volume(1.0))?
    );

    println!("Direction {}", serde_json::to_string(&Direction::Forward)?);
    println!("Direction {}", serde_json::to_string(&Direction::Reverse)?);
    println!("Direction {}", serde_json::to_string(&Direction::None)?);

    Ok(())
}
