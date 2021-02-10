use crate::base64::app_error::{AppError};

pub fn hex_str_to_u8_vec(hex_str: &str) -> Result<Vec<u8>, AppError> {
    if hex_str.len() % 2 != 0 {
        return Err(AppError { message: "String length must be a multiple of 2".to_string() });
    }

    let str_byte = hex_str.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();

    let mut result = Vec::with_capacity(hex_str.len() / 2);

    for b in str_byte {
        let byte = u8::from_str_radix(b.as_str(), 16)
            .map_err(|_| AppError { message: format!("Invalid hexadecimal byte format: {}", b) })?;
        result.push(byte);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::base64::string_utils::hex_str_to_u8_vec;

    #[test]
    fn convert_hex_string_to_u8_vec() {
        let hex_str = "4d";
        assert_eq!(hex_str_to_u8_vec(hex_str).unwrap(), vec![77]);
        let hex_str_2 = "4d61";
        assert_eq!(hex_str_to_u8_vec(hex_str_2).unwrap(), vec![77, 97]);
    }
}