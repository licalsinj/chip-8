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
        // TODO: Do this in a for loop
        // for _ in ..8
        result.push((self & 0b1000_0000) == 0b1000_0000);
        result.push((self & 0b0100_0000) == 0b0100_0000);
        result.push((self & 0b0010_0000) == 0b0010_0000);
        result.push((self & 0b0001_0000) == 0b0001_0000);
        result.push((self & 0b0000_1000) == 0b0000_1000);
        result.push((self & 0b0000_0100) == 0b0000_0100);
        result.push((self & 0b0000_0010) == 0b0000_0010);
        result.push((self & 0b0000_0001) == 0b0000_0001);
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

#[derive(Debug)]
// errors to be used if Vec<bool> can't cleanly go into T
pub enum BitwiseCreationErr {
    TooShort, // The vector provided is too short, it should be 8 bits exactly
    TooLong,  // The vector provided is too long, it should be 8 bits exactly
}

// TODO: Write some tests for this
// Such as:
// - [ ] Creation short & long are hit correctly
// - [ ] Creation works correctly
// - [ ] bit_vec works correctly
