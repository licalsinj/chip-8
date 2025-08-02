// A trait to make bit manipulation of u8s easier
pub trait Bitwise {
    // should convert the bits in T to a Vec<bool>
    fn bit_vec(&self) -> Vec<bool>;
    // should convert a Vec<bool> back to T
    // Creation errors are available if the exact length isn't provided
    fn from_bit_vec(v: Vec<bool>) -> Result<Self, BitwiseCreationErr>
    where
        Self: Sized;
}

// Implementation of Bitwise trait for u8
impl Bitwise for u8 {
    // Will convert the bits in T to a Vec<bool>
    fn bit_vec(&self) -> Vec<bool> {
        let mut result = Vec::new();
        let mut power_2 = 0b1000_0000;
        for _ in 0..8 {
            result.push((self & power_2) == power_2);
            power_2 /= 2;
        }
        result
    }
    // Will convert a Vec<bool> back to T
    // Creation errors are available if exactly 8 bools aren't provided
    fn from_bit_vec(v: Vec<bool>) -> Result<u8, BitwiseCreationErr> {
        if v.len() < 8 {
            return Err(BitwiseCreationErr::TooShort);
        }
        if v.len() > 8 {
            return Err(BitwiseCreationErr::TooLong);
        }
        let mut result: u32 = 0;
        for b in v.iter() {
            result = result << 1 | (b == &true) as u32;
        }
        // mask the extra bits in u32 so that when I cast it there's no overflow
        result = result & 0xFF;
        Ok(result as u8)
    }
}

#[derive(Debug, PartialEq)]
// errors to be used if Vec<bool> can't cleanly go into T
pub enum BitwiseCreationErr {
    TooShort, // The vector provided is too short, it should be 8 bits exactly
    TooLong,  // The vector provided is too long, it should be 8 bits exactly
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // test that the bitwise implementation of u8 .bit_vec
    // works correctly when given ideal parameters
    fn correctly_generate_vec_from_u8() {
        let bit_vec = vec![true, false, true, false, false, true, false, true];
        let binary: u8 = 0b1010_0101;
        assert_eq!(bit_vec, binary.bit_vec());
    }

    #[test]
    // tests you can generate a number from a Vec<bool> when given ideal parameters
    fn correctly_generate_u8_from_vec() {
        let bit_vec = vec![true, false, true, false, false, true, false, true];
        assert_eq!(Ok(0b1010_0101), u8::from_bit_vec(bit_vec));
    }

    #[test]
    // tests that you get a BitwiseCreationErr::TooLong if you submit a Vec<bool> that's over 8
    // bools long
    fn vec_too_long_for_u8_creation() {
        let bit_vec = vec![true, false, true, false, false, true, false, true, false];
        assert_eq!(Err(BitwiseCreationErr::TooLong), u8::from_bit_vec(bit_vec));
    }
    #[test]
    // tests that you get a BitwiseCreationErr::TooShort if you submit a Vec<bool> that's under 8
    // bools long
    fn vec_too_short_for_u8_creation() {
        let bit_vec = vec![true, false, true, false, true, false, true];
        assert_eq!(Err(BitwiseCreationErr::TooShort), u8::from_bit_vec(bit_vec));
    }
}
