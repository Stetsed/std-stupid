pub mod standard {}

use std::error::Error;

use crate::errors_stupid::{self, *};

pub fn mapRange(
    min_in: i64,
    max_in: i64,
    min_out: i64,
    max_out: i64,
    value: i64,
) -> Result<i64, intValueError> {
    if (min_in > value || max_in < value) {
        Err::<i64, intValueError>(intValueError {
            source: String::from("Value out of bounds of input"),
        });
    }

    let calculate_range_differential: f64 =
        (1.0 * ((max_out - min_out) as f64) / ((max_in - max_out) as f64)) as f64;
    println!("Range Diff: {}", calculate_range_differential);
    let return_value: i64 =
        (min_out as f64 + calculate_range_differential * (value - min_in) as f64) as i64;
    Ok(return_value)
}
