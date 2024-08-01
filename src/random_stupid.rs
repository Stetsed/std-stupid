use core::fmt;
use std::default::Default;
use std::error;
use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, Error};

mod random_stupid {}

#[derive(Debug)]
pub struct randomNumberGenerator {
    pub seed: u64,
    pub latestNumber: u64,
    randFileDescriptor: File,
}

impl error::Error for randomNumberGenerator {}

impl Display for randomNumberGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error ya wank")
    }
}

impl randomNumberGenerator {
    pub fn getRandomNumber(mut self) -> u64 {
        let mut buffer: [u8; 8] = [0; 8];
        let random_garbage = self
            .randFileDescriptor
            .read(&mut buffer)
            .unwrap()
            .to_string();

        let mut final_spit: u64 = 0;

        for instance in buffer {
            final_spit += instance as u64;
        }

        self.latestNumber = final_spit;
        return final_spit;
    }
    pub fn new() -> Result<randomNumberGenerator, Error> {
        let randFileDescriptor = File::open("/dev/urandom")?;

        Ok(randomNumberGenerator {
            seed: 0,
            latestNumber: 0,
            randFileDescriptor,
        })
    }
}
