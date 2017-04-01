extern crate arustid;

use std::env;
use std::process;

fn main() {

    let config = arustid::Config::new(env::args()).unwrap();

    if let Err(err) = arustid::run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
