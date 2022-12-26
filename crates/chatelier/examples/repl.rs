use std::{fmt::Write, time::Duration};

use chatelier::System;
use chem_eq::Equation;
use once_cell::sync::Lazy;
use reedline_repl_rs::{
    clap::{Arg, ArgMatches, Command},
    Repl, Result,
};

static EQUATION: Lazy<Equation> = Lazy::new(|| {
    let mut eq = Equation::new("N2 + O2 <-> N2O2").unwrap();
    for cmp in eq.left_mut() {
        cmp.concentration = 1.0;
    }
    eq
});
const VOLUME: f32 = 1.0;

fn main() -> Result<()> {
    let system = System::new(EQUATION.clone(), VOLUME, Duration::from_millis(10)).unwrap();

    let mut repl = Repl::new(system)
        .with_name("Chatelier")
        .with_version("v0.1.0")
        .with_description("REPL for modifying chemical equation systems")
        .with_banner("chatelier repl")
        .with_stop_on_ctrl_c(true)
        .with_command(
            Command::new("concentrations")
                .about("get concentration info")
                .arg(Arg::new("compound").help("compound to print concentration of")),
            print_concentration,
        )
        .with_command(
            Command::new("units")
                .about("get units of each compound")
                .arg(Arg::new("compound").help("compound to print units of")),
            print_units,
        )
        .with_command(Command::new("init").about("initiate simulation"), init)
        .with_command(Command::new("reset").about("reset simulation"), reset)
        .with_command(
            Command::new("adjust")
                .arg(Arg::new("change").required(true))
                .about("adjust simulation temperature, volume or concentration"),
            adjust,
        );
    repl.run()
}

fn init(_args: ArgMatches, context: &mut System) -> Result<Option<String>> {
    context.init();
    Ok(Some("Initiated system".to_string()))
}

fn reset(_args: ArgMatches, context: &mut System) -> Result<Option<String>> {
    *context = System::new(EQUATION.clone(), VOLUME, Duration::from_micros(1)).unwrap();
    Ok(Some("System reset".to_string()))
}

fn adjust(args: ArgMatches, _context: &mut System) -> Result<Option<String>> {
    Ok(Some(format!("Hello, {}", args.value_of("change").unwrap())))
}

fn print_concentration(args: ArgMatches, context: &mut System) -> Result<Option<String>> {
    if let Some(name) = args.value_of("compound") {
        Ok(Some(format!(
            "{} = {}",
            name,
            context
                .equation()
                .get_concentration_by_name(name)
                .map(|f| f.to_string())
                .unwrap_or("not found".to_string())
        )))
    } else {
        let mut buf = String::from("Concentrations:\n");
        for (name, cnc) in context.equation().name_and_concentration() {
            writeln!(buf, "\t{} = {}", name, cnc).unwrap();
        }

        Ok(Some(buf))
    }
}

fn print_units(args: ArgMatches, context: &mut System) -> Result<Option<String>> {
    if let Some(name) = args.value_of("compound") {
        Ok(Some(format!(
            "{} = {}",
            name,
            context
                .equation()
                .get_compound_by_name(name)
                .map(|c| c.get_units(VOLUME))
                .map(|u| u.to_string())
                .unwrap_or("not found".to_string())
        )))
    } else {
        let mut buf = String::from("Units:\n");
        for (name, cmp) in context
            .equation()
            .compound_names()
            .zip(context.equation().iter_compounds())
        {
            writeln!(buf, "\t{} = {}", name, cmp.get_units(VOLUME)).unwrap();
        }

        Ok(Some(buf))
    }
}
