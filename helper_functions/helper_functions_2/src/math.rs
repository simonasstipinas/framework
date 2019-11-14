pub fn xor(bytes_1: &str, bytes_2: &str) -> String {
    if bytes_1.chars().count() != 32 && bytes_2.chars().count() != 32 {
        panic!("One of the input arguments is too short to be a sha256 hash.");
    }
    if bytes_1.len() != 32 || bytes_2.len() != 32 {
        panic!("Illegal characters in one of the input strings");
    }
    let mut string_to_return = String::new();
    let bytes_1_as_bytes = bytes_1.as_bytes();
    let bytes_2_as_bytes = bytes_2.as_bytes();
    for i in 0..32 {
        if bytes_1_as_bytes[i] == bytes_2_as_bytes[i] {
            string_to_return += "1";
        } else {
            string_to_return += "0";
        }
    }
    string_to_return
}

pub fn integer_squareroot(n: u64) -> u64 {
    let sqrt = (n as f64).sqrt();
    let mut sqrt_floor = sqrt as u64;
    if (sqrt_floor + 1) * (sqrt_floor + 1) <= n {
        sqrt_floor += 1;
    }
    sqrt_floor
}

// TODO: int_to_bytes, bytes_to_int

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        let test_str_1: &str = "A4x3A4x3A4x3A4x3A4x3A4x3A4x3A4x3";
        let mut test_str_2: &str = "A4x3A4x3A4x3A4x3A4x3A4x3A4x3A4x3";
        assert_eq!(
            xor(&test_str_1, &test_str_2),
            "11111111111111111111111111111111",
        );
        test_str_2 = "AAAABBBBCCCCDDDDAAAABBBBCCCCDDDD";
        assert_eq!(
            xor(&test_str_1, &test_str_2),
            "10000000000000001000000000000000",
        );
        assert_ne!(
            xor(&test_str_1, &test_str_2),
            "11000000000000001000000000000000",
        );
    }
    #[test]
    #[should_panic]
    fn test_too_short_hashes() {
        let test_str_1: &str = "ABC";
        let test_str_2: &str = "ABC";
        assert_eq!(xor(&test_str_1, &test_str_2), "111");
    }
    #[test]
    #[should_panic]
    fn test_invalid_symbols_in_hashes() {
        let test_str_1: &str = "ĄĄĄĄĘĘĘĘĮĮĮĮŠŠŠŠĄĄĄĄĘĘĘĘĮĮĮĮŠŠŠŠ";
        let test_str_2: &str = "ĄĄĄĄĘĘĘĘĮĮĮĮŠŠŠŠĄĄĄĄĘĘĘĘĮĮĮĮŠŠŠŠ";
        assert_eq!(
            xor(&test_str_1, &test_str_2),
            "11111111111111111111111111111111",
        );
    }
}
