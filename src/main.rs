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
        .arg(Arg::with_name("iterations")
             .short("n")
             .long("iterations")
             .value_name("N")
             .help("Number of iterations to run")
             .takes_value(true))
        .get_matches();

    // TODO Use something like this to get pre-defined system
    // let mode = String::from(matches.value_of("system").unwrap_or("koch"));
    // let system = lsystem::get_system(&config.mode);

    let iterations: u32 = matches.value_of("iterations").unwrap_or("0").parse().unwrap();

    let axiom = String::from(matches.value_of("axiom").expect("No axiom chosen"));
    let angle: i32 = matches.value_of("angle").expect("No angle chosen").parse().unwrap();

    let system = LSystem {
        axiom: axiom,
        rules: vec![Rule::new('F', "FFF")],
        angle: angle,
    };

    let config = arustid::Config {
        system: system,
        iterations: iterations,
        output_filename: String::from("output.png"),
    };

    if let Err(err) = arustid::run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
