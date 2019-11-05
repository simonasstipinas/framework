use crate::error::Error;

pub fn integer_squareroot(_n: u64) -> Result<u64, Error> {
    Ok(1)
}

pub fn xor(_bytes_1: &[u8], _bytes_2: &[u8]) -> Result<Vec<u8>, Error> {
    Ok([].to_vec())
}

pub fn int_to_bytes(_int: u64, _length: usize) -> Result<Vec<u8>, Error> {
    Ok([].to_vec())
}

pub fn bytes_to_int(_bytes: &[u8]) -> u64 {
    0
}
