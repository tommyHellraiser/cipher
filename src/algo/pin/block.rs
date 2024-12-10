use super::PinBlockFiller;
use crate::{
    error::{CipherError, CipherResult},
    utils,
};
use std::fmt::Write;

const BLOCK_LENGTH: usize = 16;

/// Calculates the PIN blokc for a given set of inputs
pub fn calculate_pin_block(
    pin: String,
    pan: String,
    /* key: String,  */ filler: PinBlockFiller,
) -> CipherResult<String> {
    //  Attempt to parse the pin into a number to verify it's a valid PIN
    if pin.chars().any(|item| !item.is_ascii_digit()) {
        return Err(CipherError::InvalidPin("Invalid digit in PIN".to_string()));
    }
    if pin.len() < 4 || pin.len() > 12 {
        return Err(CipherError::InvalidPin(
            "PIN length cannot be smaller than 4 or longer than 12".to_string(),
        ));
    }

    //  Same thing with the PAN
    if pan.chars().any(|item| !item.is_ascii_digit()) {
        return Err(CipherError::IncorrectPan(
            "Incorrect digit found in PAN".to_string(),
        ));
    }
    if pan.len() < 16 || pan.len() > 19 {
        return Err(CipherError::IncorrectPan(
            "PAN length cannot be greater than 19 or smaller than 16".to_string(),
        ));
    }

    /* //  Validate key content
    if key.chars().any(|item| !item.is_ascii_alphabetic() && !item.is_ascii_digit()) {
        return Err(CipherError::InvalidKey("Key contains invalid characters".to_string()))
    }
    //  Valid key length, it should be 32 hex digits
    if key.len() != 32 {
        return Err(CipherError::InvalidKey("Invalid length in key".to_string()))
    } */

    let block1 = build_pin_block_1(pin, filler)?;
    let block2 = build_pin_block_2(pan)?;

    //  Validate length of both strings
    if block1.len() != block2.len() {
        return Err(CipherError::InternalError(
            "Invalid length found between block 1 and 2".to_string(),
        ));
    }

    let block1 =
        hex::decode(block1).map_err(|error| CipherError::ParsingError(error.to_string()))?;
    let block2 =
        hex::decode(block2).map_err(|error| CipherError::ParsingError(error.to_string()))?;
    let pin_block_bytes: Vec<u8> = block1
        .iter()
        .zip(block2.iter())
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect();

    let pin_block = pin_block_bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02X}");
        output
    });

    Ok(pin_block)
}

fn build_pin_block_1(pin: String, filler: PinBlockFiller) -> CipherResult<String> {
    let mut block1 = String::with_capacity(BLOCK_LENGTH);
    block1.push('0'); //  Always start with 0 for ISO Format 0
    block1.push_str(pin.len().to_string().as_str()); //  PIN Length
    block1.push_str(pin.as_str());

    let filler_char = match filler {
        PinBlockFiller::Zero => '0',
        PinBlockFiller::F => 'F',
    };

    block1.push_str(
        utils::string::fill_left_with_char(
            String::new(),
            filler_char,
            BLOCK_LENGTH - block1.len(),
        )?
        .as_str(),
    );

    Ok(block1)
}

fn build_pin_block_2(mut pan: String) -> CipherResult<String> {
    let drain_end = pan.len() - 1;
    let drain_start = drain_end - 12;
    let pan_filtered = pan.drain(drain_start..drain_end).collect::<String>();

    Ok(format!("0000{}", pan_filtered))
}

#[cfg(test)]
mod test {
    use crate::algo::pin::block::build_pin_block_1;
    use crate::algo::pin::block::build_pin_block_2;
    use crate::algo::pin::block::PinBlockFiller;

    use super::calculate_pin_block;

    #[test]
    fn test_pin_block_1() {
        let output = build_pin_block_1("2511".to_string(), PinBlockFiller::F).unwrap();

        assert_eq!(output.as_str(), "042511FFFFFFFFFF")
    }
    #[test]
    fn test_pin_block_2() {
        let output = build_pin_block_2("1234567891234567894".to_string()).unwrap();

        assert_eq!(output.as_str(), "0000789123456789")
    }
    #[test]
    fn test_pin_block() {
        let pan = "4564325134571357".to_string();
        let pin = "2511".to_string();

        let output = calculate_pin_block(pin, pan, PinBlockFiller::F).unwrap();

        assert_eq!(output.as_str(), "042552DAECBA8ECA")
    }
}
