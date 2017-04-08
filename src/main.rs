extern crate arustid;
extern crate clap;

use std::process;

use clap::{Arg, App};

use arustid::lsystem::{LSystem, Rule};

fn main() {
    let matches = App::new("arustid")
        .version("0.1.0")
        .about("Draws L-Systems")
        .author("Simon Andersson <simonasker@posteo.net>")
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
             .help("Number of iterations to run")
             .takes_value(true))
        .arg(Arg::with_name("length")
             .short("l")
             .long("length")
             .value_name("LENGTH")
             .help("The length of each line segment")
             .takes_value(true))
        .get_matches();

    // TODO Use something like this to get pre-defined system
    // let mode = String::from(matches.value_of("system").unwrap_or("koch"));
    // let system = lsystem::get_system(&config.mode);

    let iterations: u32 = matches.value_of("iterations").unwrap_or("0").parse().unwrap();
    // TODO For some reason the image is distorted if this value is not divisible by two
    let length: i32 = matches.value_of("length").unwrap_or("10").parse().unwrap();

    let axiom = String::from(matches.value_of("axiom").expect("No axiom chosen"));
    let angle: i32 = matches.value_of("angle").expect("No angle chosen").parse().unwrap();
    let rules: Vec<Rule> = matches.values_of("rule").unwrap().map(|s| Rule::from_string(s).unwrap()).collect();

    let system = LSystem {
        axiom: axiom,
        rules: rules,
        angle: angle,
    };

    let config = arustid::Config {
        system: system,
        iterations: iterations,
        length: length,
        output_filename: String::from("output.png"),
    };

    if let Err(err) = arustid::run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
