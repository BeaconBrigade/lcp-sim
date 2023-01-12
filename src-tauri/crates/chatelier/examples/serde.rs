use std::error::Error;

use chatelier::Adjustment;

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

    Ok(())
}
