extern crate arustid;
#[macro_use]
extern crate clap;

use arustid::lsystem::{LSystem, Rule};

use clap::{Arg, App};
use std::process;

fn main() {
    let matches = App::new("arustid")
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
        .arg(Arg::with_name("output")
                 .short("o")
                 .long("output")
                 .value_name("FILE")
                 .help("The ouput file name")
                 .takes_value(true))
        .get_matches();

    let iterations: u32 = matches.value_of("iterations")
        .unwrap_or("0")
        .parse()
        .unwrap();
    // TODO For some reason the image is distorted if this value is not divisible
    // by two
    let length: i32 = matches.value_of("length")
        .unwrap_or("10")
        .parse()
        .unwrap();

    let axiom = String::from(matches.value_of("axiom").expect("No axiom chosen"));
    let angle: i32 = matches.value_of("angle")
        .expect("No angle chosen")
        .parse()
        .unwrap();
    let rules: Vec<Rule> = matches.values_of("rule")
        .unwrap()
        .map(|s| Rule::from_string(s).unwrap())
        .collect();

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

    let mut mode = "window";

    if matches.is_present("output") {
        config.output_filename = Some(String::from(matches.value_of("output").unwrap()));
        mode = "image";
    }

    if let Err(err) = arustid::run(mode, config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
