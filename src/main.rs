extern crate arustid;
#[macro_use]
extern crate clap;

use arustid::Config;
use arustid::lsystem::{LSystem, Rule};

use clap::{Arg, App};
use std::process;
use std::error::Error;

type Mode = String;

fn create_app() -> App<'static, 'static> {
    App::new("arustid")
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("system")
                 .short("s")
                 .long("system")
                 .value_name("NAME")
                 .help("Select a system")
                 .takes_value(true))
        .arg(Arg::with_name("axiom")
                 .short("a")
                 .long("axiom")
                 .value_name("AXIOM")
                 .help("The starting string")
                 .takes_value(true))
        .arg(Arg::with_name("angle")
                 .short("v")
                 .long("angle")
                 .value_name("ANGLE")
                 .help("The angle to turn")
                 .default_value("90")
                 .takes_value(true))
        .arg(Arg::with_name("rule")
                 .short("r")
                 .long("rule")
                 .value_name("RULE")
                 .help("A production rule")
                 .multiple(true)
                 .number_of_values(1)
                 .takes_value(true))
        .arg(Arg::with_name("iterations")
                 .short("n")
                 .long("iterations")
                 .value_name("N")
                 .default_value("0")
                 .help("Number of iterations to run")
                 .takes_value(true))
        .arg(Arg::with_name("length")
                 .short("l")
                 .long("length")
                 .value_name("LENGTH")
                 .help("The length of each line segment")
                 .default_value("10")
                 .takes_value(true))
        .arg(Arg::with_name("output")
                 .short("o")
                 .long("output")
                 .value_name("FILE")
                 .help("The ouput file name")
                 .takes_value(true))
}

fn parse_args() -> Result<(Mode, Config), Box<Error>> {
    let app = create_app();
    let matches = app.get_matches();

    let angle: i32 = matches.value_of("angle").ok_or("No angle")?.parse()?;
    let iterations: u32 = matches.value_of("iterations").ok_or("No iterations")?.parse()?;
    let length: i32 = matches.value_of("length").ok_or("No length")?.parse()?;
    let axiom = String::from(matches.value_of("axiom").ok_or("No axiom")?);
    let rules: Vec<Rule> = matches.values_of("rule").ok_or("No rules")?
        .map(|s| Rule::from_string(s).unwrap()).collect();

    let system = LSystem {
        axiom: axiom,
        rules: rules,
        angle: angle,
    };

    let mut config = arustid::Config {
        system: system,
        iterations: iterations,
        length: length,
        output_filename: None,
    };

    let mut mode = String::from("window");

    if matches.is_present("output") {
        config.output_filename = Some(String::from(matches.value_of("output").unwrap()));
        mode = String::from("image");
    }

    Ok((mode, config))
}

fn main() {
    let (mode, config) = parse_args().unwrap_or_else(|err| {
        // TODO Print this to stderr instead
        println!("Application error: {}", err);
        process::exit(1);
    });

    if let Err(err) = arustid::run(&mode, config) {
        // TODO Print this to stderr instead
        println!("Application error: {}", err);
        process::exit(1);
    }
}
