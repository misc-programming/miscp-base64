use crate::base64::app_error::{AppError};

static BASE64_PADDING: &'static char = &'=';

fn byte_to_base64(byte: u8) -> Result<char, AppError> {
    let result = match byte {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        26 => 'a',
        27 => 'b',
        28 => 'c',
        29 => 'd',
        30 => 'e',
        31 => 'f',
        32 => 'g',
        33 => 'h',
        34 => 'i',
        35 => 'j',
        36 => 'k',
        37 => 'l',
        38 => 'm',
        39 => 'n',
        40 => 'o',
        41 => 'p',
        42 => 'q',
        43 => 'r',
        44 => 's',
        45 => 't',
        46 => 'u',
        47 => 'v',
        48 => 'w',
        49 => 'x',
        50 => 'y',
        51 => 'z',
        52 => '0',
        53 => '1',
        54 => '2',
        55 => '3',
        56 => '4',
        57 => '5',
        58 => '6',
        59 => '7',
        60 => '8',
        61 => '9',
        62 => '+',
        63 => '/',
        _ => return Err(AppError { message: format!("Invalid base64 byte: {}", byte) })
    };
    Ok(result)
}

fn validate_array_length(slice: &[u8], expected_len: usize) -> Result<(), AppError> {
    if slice.len() != expected_len {
        return Err(AppError { message: format!("Invalid array length: Expected {}, got {} ", expected_len, slice.len()) });
    }
    Ok(())
}

fn three_bytes_to_base64(bytes: &[u8]) -> Result<String, AppError> {
    validate_array_length(bytes, 3)?;
    let char1 = byte_to_base64(&bytes[0] >> 2)?;
    let char2 = byte_to_base64(((&bytes[0] & 0b00000011) << 4) | (&bytes[1] >> 4))?;
    let char3 = byte_to_base64(((&bytes[1] & 0b00001111) << 2) | (&bytes[2] >> 6))?;
    let char4 = byte_to_base64(&bytes[2] & 0b00111111)?;
    let base64 = format!("{}{}{}{}", char1, char2, char3, char4);
    Ok(base64)
}

fn two_bytes_to_base64(bytes: &[u8]) -> Result<String, AppError> {
    validate_array_length(&bytes, 2)?;
    let char1 = byte_to_base64(&bytes[0] >> 2)?;
    let char2 = byte_to_base64(((&bytes[0] & 0b00000011) << 4) | (&bytes[1] >> 4))?;
    let char3 = byte_to_base64((&bytes[1] & 0b00001111) << 2)?;
    let base64 = format!("{}{}{}{}", char1, char2, char3, BASE64_PADDING);
    Ok(base64)
}

fn one_byte_to_base64(byte: &u8) -> Result<String, AppError> {
    let char1 = byte_to_base64(byte >> 2)?;
    let char2 = byte_to_base64((byte & 0b00000011) << 4)?;
    let base64 = format!("{}{}{}{}", char1, char2, BASE64_PADDING, BASE64_PADDING);
    Ok(base64)
}

fn u8_vec_to_base64(vec: Vec<u8>) -> Result<String, AppError> {
    let mut accumulator = vec![];
    let chunks = vec.chunks(3);
    for c in chunks {
        if c.len() == 3 {
            let base64 = three_bytes_to_base64(c)?;
            accumulator.push(base64);
        } else if c.len() == 2 {
            let base64 = two_bytes_to_base64(c)?;
            accumulator.push(base64);
        } else if c.len()  == 1 {
            let base64 = one_byte_to_base64(&c[0])?;
            accumulator.push(base64);
        } else {
            panic!("Impossible");
        }
    }
    Ok(accumulator.concat())
}

#[cfg(test)]
mod tests {
    use crate::base64::base64::u8_vec_to_base64;
    use crate::base64::string_utils::{hex_str_to_u8_vec};

    #[test]
    fn convert_hex_string_to_base64_string() {
        let hex_0 = "4d616e";
        let base64_0 = "TWFu";
        let hex0_bytes = hex_str_to_u8_vec(hex_0).unwrap();
        assert_eq!(u8_vec_to_base64(hex0_bytes).unwrap(), base64_0);

        let hex_1 = "4d61";
        let base64_1 = "TWE=";
        let hex1_bytes = hex_str_to_u8_vec(hex_1).unwrap();
        assert_eq!(u8_vec_to_base64(hex1_bytes).unwrap(), base64_1);

        let hex_2 = "4d";
        let base64_2 = "TQ==";
        let hex2_bytes = hex_str_to_u8_vec(hex_2).unwrap();
        assert_eq!(u8_vec_to_base64(hex2_bytes).unwrap(), base64_2);

        let hex_3 = "e768a120717569636b2062726f776e20666f78206a756d7073206f766572203133206c617a7920646f67732e";
        let base64_3 = "52ihIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIDEzIGxhenkgZG9ncy4=";
        let hex3_bytes = hex_str_to_u8_vec(hex_3).unwrap();
        assert_eq!(u8_vec_to_base64(hex3_bytes).unwrap(), base64_3);
    }
}