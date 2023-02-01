use std::fmt::Write;

use chatelier::{Adjustment, System};
use chem_eq::Equation;
use once_cell::sync::Lazy;
use reedline_repl_rs::{
    clap::{Arg, ArgMatches, Command},
    Error, Repl, Result as ReplResult,
};

static EQUATION: Lazy<Equation> = Lazy::new(|| {
    let mut eq = Equation::new("2NH3(g) <-> N2(g) + 3H2(g)").unwrap();
    eq.set_concentrations(&[2.0, 1.0, 1.5]).unwrap();
    eq
});

const VOLUME: f32 = 1.0;

fn main() -> ReplResult<()> {
    let system = System::new(EQUATION.clone()).unwrap();

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
        .with_command(Command::new("reset").about("reset simulation"), reset)
        .with_command(
            Command::new("k-expr").about("print k-expression"),
            print_k_expr,
        )
        .with_command(Command::new("volume").about("print volume"), print_volume)
        .with_command(
            Command::new("equation").about("print equation"),
            print_equation,
        )
        .with_command(
            Command::new("temperature").about("print temperature"),
            print_temperature,
        )
        .with_command(
            Command::new("adjust")
                .args(&[
                    Arg::new("type")
                        .help("type of adjustment: c|v|t")
                        .required(true),
                    Arg::new("num").help("new value").required(true),
                    Arg::new("name").help("name of compound"),
                ])
                .about("adjust simulation temperature, volume or concentration"),
            adjust,
        );
    repl.run()
}

fn reset(_args: ArgMatches, context: &mut System) -> ReplResult<Option<String>> {
    *context = System::new(EQUATION.clone()).unwrap();
    Ok(Some("System reset".to_string()))
}

fn print_concentration(args: ArgMatches, context: &mut System) -> ReplResult<Option<String>> {
    if let Some(name) = args.get_one::<String>("compound") {
        Ok(Some(format!(
            "{} = {}",
            name,
            context
                .equation()
                .get_concentration_by_name(name)
                .map(|f| f.to_string())
                .unwrap_or_else(|_| "not found".to_string())
        )))
    } else {
        let mut buf = String::from("Concentrations:\n");
        for (name, cnc) in context.equation().name_and_concentration() {
            writeln!(buf, "\t{} = {}M", name, cnc).unwrap();
        }

        Ok(Some(buf))
    }
}

fn print_units(args: ArgMatches, context: &mut System) -> ReplResult<Option<String>> {
    if let Some(name) = args.get_one::<String>("compound") {
        Ok(Some(format!(
            "{} = {}",
            name,
            context
                .equation()
                .get_compound_by_name(name)
                .map(|c| c.get_units(VOLUME))
                .map(|u| u.to_string())
                .unwrap_or_else(|| "not found".to_string())
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

fn print_k_expr(_args: ArgMatches, context: &mut System) -> ReplResult<Option<String>> {
    Ok(Some(format!(
        "Kc = {:?}",
        context.equation().reaction_quotient()
    )))
}

fn print_volume(_args: ArgMatches, context: &mut System) -> ReplResult<Option<String>> {
    Ok(Some(format!(
        "V = {}L",
        context.equation().volume().unwrap_or(1.0)
    )))
}

fn print_temperature(_args: ArgMatches, context: &mut System) -> ReplResult<Option<String>> {
    Ok(Some(format!(
        "T = {}°C",
        context.equation().temperature().unwrap_or(0.0)
    )))
}

fn print_equation(_args: ArgMatches, context: &mut System) -> ReplResult<Option<String>> {
    Ok(Some(format!("eq: {}", context.equation())))
}

fn adjust(args: ArgMatches, context: &mut System) -> ReplResult<Option<String>> {
    let num = args.get_one::<String>("num").unwrap().parse::<f32>()?;
    if num < 0.0 {
        return Ok(Some("Error: num must be positive".to_string()));
    }
    let name = args.get_one::<String>("name");
    let adjustment = match args.get_one::<String>("type").unwrap().as_str() {
        "t" => Adjustment::Temperature(num),
        "v" => Adjustment::Volume(num),
        "c" => Adjustment::Concentration(
            name.ok_or_else(|| {
                Error::MissingRequiredArgument("adjust c".to_string(), "name".to_string())
            })?,
            num,
        ),
        _ => {
            return Err(Error::UnknownCommand(
                "Invalid type: options are c|v|t".to_string(),
            ))
        }
    };

    Ok(Some(
        context
            .adjust(adjustment)
            .map(|_| "Adjusted system".to_string())
            .unwrap_or_else(|e| e.to_string()),
    ))
}
