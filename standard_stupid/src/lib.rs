pub mod thread_manager;

use core::str;

use errors_stupid::*;
use sha1::{Digest, Sha1, Sha1Core};

pub fn find_substring_bytes_start(
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

pub fn hash_text_sha1<T: AsRef<str>>(text: T) -> Result<Vec<u8>, StdStupidError> {
    let mut hasher = Sha1::new();

    hasher.update(text.as_ref());

    let ok = hasher.finalize();

    Ok(ok[..].to_vec())
}

#[cfg(test)]
mod standard_stupid_tests {
    use crate::find_substring_bytes_start;

    #[test]
    #[should_panic]
    fn sub_string_doesnt_match() {
        let input = "Does not Match";
        let sub_string = "hello";

        find_substring_bytes_start(input.as_bytes(), sub_string.as_bytes()).unwrap();
    }

    #[test]
    fn sub_string_does_match() {
        let input = "Does match";
        let sub_string = "match";

        let location = find_substring_bytes_start(input.as_bytes(), sub_string.as_bytes()).unwrap();

        assert_eq!(location, 5);
    }

    #[test]
    fn sub_string_exact_math() {
        let input = "Match";
        let sub_string = "Match";

        let location = find_substring_bytes_start(input.as_bytes(), sub_string.as_bytes()).unwrap();

        assert_eq!(location, 0)
    }
}
