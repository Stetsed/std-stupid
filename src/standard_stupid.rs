pub mod standard {}

use crate::errors_stupid::*;

#[allow(dead_code)]
pub fn mapRange(
    min_in: i64,
    max_in: i64,
    min_out: i64,
    max_out: i64,
    value: i64,
) -> Result<i64, StdStupidError> {
    if min_in > value || max_in < value {
        let _ = Into::<StdStupidError>::into(IntValueError::new("Value out of bounds of input"));
    }

    let calculate_range_differential: f64 =
        1.0 * ((max_out - min_out) as f64) / ((max_in - max_out) as f64);
    println!("Range Diff: {}", calculate_range_differential);
    let return_value: i64 =
        (min_out as f64 + calculate_range_differential * (value - min_in) as f64) as i64;
    Ok(return_value)
}

#[allow(dead_code)]
pub fn findSubStringWithString(array: Vec<u8>, subString: String) -> Result<u32, StdStupidError> {
    let subStringAsBytes: Vec<u8> = b"subString".to_vec();
    let subStringLength = subString.len();
    let mut location: Option<u32> = None;

    for i in 0..(array.len() - 1) {
        if array[i] == subStringAsBytes[0] {
            let compare = array[i..i + subStringLength].to_vec();

            if compare == subStringAsBytes {
                location = Some(i as u32);
                break;
            }
        }
    }

    match location {
        Some(e) => Ok(e),
        None => Err(SubStringError::new("Substring has not been found in provided input").into()),
    }
}

pub fn findSubStringWithBytes(
    array: Vec<u8>,
    subStringAsBytes: &[u8],
) -> Result<u32, StdStupidError> {
    let subStringLength = subStringAsBytes.len();
    let mut location: Option<u32> = None;

    for i in 0..(array.len() - 1) {
        if array[i] == subStringAsBytes[0] {
            let compare = array[i..i + subStringLength].to_vec();

            if compare == subStringAsBytes {
                location = Some(i as u32);
                break;
            }
        }
    }

    match location {
        Some(e) => Ok(e),
        None => Err(SubStringError::new("Substring has not been found in provided input").into()),
    }
}
