pub mod thread_manager;

use crate::thread_manager::*;

use errors_stupid::*;

pub fn map_range(
    min_in: i64,
    max_in: i64,
    min_out: i64,
    max_out: i64,
    value: i64,
) -> Result<i64, StdStupidError> {
    if min_in > value || max_in < value {
        return Err(IntValueError::new("Value out of bounds of input").into());
    }

    if min_in > max_in || max_out < min_out {
        return Err(IntValueError::new(
            "Value provided for the in/output bounds are invalid as max is not greater than min",
        )
        .into());
    }

    let slope: f64 = (max_out - min_out) as f64 / (max_in - min_in) as f64;

    let output: i64 = (min_out as f64 + (slope * (value - min_in) as f64)) as i64;

    Ok(output)
}

pub fn findSubStringWithBytes(
    array: &[u8],
    sub_string_as_bytes: &[u8],
) -> Result<u32, StdStupidError> {
    let sub_string_length = sub_string_as_bytes.len();
    let array_length = array.len();
    let mut location: Option<u32> = None;

    if sub_string_length == 0 {
        location = None;
    } else if sub_string_length == array_length && *array == *sub_string_as_bytes {
        location = Some(0)
    } else if sub_string_length < array_length {
        let mut counter = 0;
        for (i, ct) in array.iter().enumerate() {
            if sub_string_as_bytes[counter] == *ct {
                counter += 1;
                if counter == sub_string_length {
                    location = Some((i - sub_string_length + 1) as u32);
                    break;
                }
            } else {
                counter = 0;
            }
        }
    }

    match location {
        Some(e) => Ok(e),
        None => Err(SubStringError::new("Substring has not been found in provided input").into()),
    }
}

#[cfg(test)]
mod standard_stupid_tests {
    use crate::findSubStringWithBytes;

    #[test]
    #[should_panic]
    fn sub_string_doesnt_match() {
        let input = "Does not Match";
        let sub_string = "hello";

        findSubStringWithBytes(input.as_bytes(), sub_string.as_bytes()).unwrap();
    }

    #[test]
    fn sub_string_does_match() {
        let input = "Does match";
        let sub_string = "match";

        let location = findSubStringWithBytes(input.as_bytes(), sub_string.as_bytes()).unwrap();

        assert_eq!(location, 5);
    }

    #[test]
    fn sub_string_exact_math() {
        let input = "Match";
        let sub_string = "Match";

        let location = findSubStringWithBytes(input.as_bytes(), sub_string.as_bytes()).unwrap();

        assert_eq!(location, 0)
    }
}
