mod errors_stupid;
mod random_stupid;
mod standard_stupid;

use core::fmt;
use std::default::Default;
use std::error;
use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, Error};
use std::process::Termination;

use errors_stupid::*;
use random_stupid::*;
use standard_stupid::*;

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let random: randomNumberGenerator = randomNumberGenerator::new()?;
    println!("{}", random.getRandomNumber());

    println!("{}", mapRange(100, 1000, 1000, 10000, 150).unwrap());
    Ok(())
}
