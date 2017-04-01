extern crate arustid;

use std::process;

fn main() {
    if let Err(err) = arustid::run() {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
