use std::cmp::Ordering;

use crate::error::{CipherError, CipherResult};

pub(crate) fn fill_left_with_char(
    string: String,
    char: char,
    length: usize,
) -> CipherResult<String> {
    match string.len().cmp(&length) {
        Ordering::Greater => Err(CipherError::ErrorProcessingString(
            "String is longer than desired length".to_string(),
        )),
        Ordering::Equal => Ok(string),
        Ordering::Less => {
            let mut temp = String::with_capacity(length - string.len());
            for _ in 0..(length - string.len()) {
                temp.push(char);
            }
            temp.push_str(string.as_str());
            Ok(temp)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fill_left_with_char;

    #[test]
    fn test_fill_left() {
        let output = fill_left_with_char("hola".to_string(), '5', 10).unwrap();

        assert_eq!(output, "555555hola".to_string())
    }
    #[test]
    fn test_fill_left_equal() {
        let output = fill_left_with_char("hola".to_string(), '5', 4).unwrap();

        assert_eq!(output, "hola".to_string())
    }
    #[test]
    fn test_fill_left_smaller() {
        let output = fill_left_with_char("hola".to_string(), '5', 2);

        assert!(output.is_err())
    }
}
