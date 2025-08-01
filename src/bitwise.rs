// creating a trait to make bit manipulation of u8s easier

pub trait Bitwise {
    fn bit_vec(&self) -> Vec<bool>;
    fn from_bit_vec(v: Vec<bool>) -> Result<Self, BitwiseCreationErr>
    where
        Self: Sized;
}

impl Bitwise for u8 {
    fn bit_vec(&self) -> Vec<bool> {
        let mut result = Vec::new();
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
pub enum BitwiseCreationErr {
    TooShort, // The vector provided is too short to create datatype
    TooLong,  // The vector provided is too long to create the datatype
}

// TODO: Write some tests for this
// Such as:
// - [ ] Creation short & long are hit correctly
// - [ ] Creation works correctly
// - [ ] bit_vec works correctly
