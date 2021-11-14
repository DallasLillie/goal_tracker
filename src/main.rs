use std::error::Error;
use std::process;

mod goals;

extern crate csv;
// this has to be in the crate root according to Rust's rules
#[macro_use]
extern crate serde_derive;

fn run() -> Result<(), Box<dyn Error>> {
    let mut goals = Vec::new();

    goals::load_goals(&mut goals)?;
    goals::save_goals(&goals)
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
