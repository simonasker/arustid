extern crate arustid;
extern crate clap;

use std::process;

use clap::{Arg, App};

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
        .arg(Arg::with_name("iterations")
             .short("n")
             .long("iterations")
             .value_name("N")
             .help("Number of iterations to run")
             .takes_value(true))
        .get_matches();

    let system = String::from(matches.value_of("system").unwrap_or("koch"));
    let iterations: u32 = matches.value_of("iterations").unwrap_or("0").parse().unwrap();

    let config = arustid::Config {
        mode: system,
        iterations: iterations,
        output_filename: String::from("output.png"),
    };

    if let Err(err) = arustid::run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
