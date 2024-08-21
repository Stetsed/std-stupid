use core::fmt;
use std::error;
use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, Error};

use crate::StdStupidError;

#[derive(Debug)]
pub struct RandomNumberGenerator {
    seed: u64,
    latestNumber: u64,
    randFileDescriptor: File,
}

impl error::Error for RandomNumberGenerator {}

impl Display for RandomNumberGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error ya wank")
    }
}

impl RandomNumberGenerator {
    pub fn getRandomNumber(mut self) -> Result<u64, StdStupidError> {
        let mut buffer: [u8; 8] = [0; 8];
        self.randFileDescriptor.read_exact(&mut buffer)?;

        let mut final_spit: u64 = 0;

        for instance in buffer {
            final_spit += instance as u64;
        }

        self.latestNumber = final_spit;
        Ok(final_spit)
    }
    pub fn new() -> Result<Self, Error> {
        let randFileDescriptor = File::open("/dev/urandom")?;

        Ok(RandomNumberGenerator {
            seed: 0,
            latestNumber: 0,
            randFileDescriptor,
        })
    }
}
