// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

use std::ops::{Add, AddAssign};

impl Add for WrappingU32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.value.wrapping_add(rhs.value))
    }
}

impl AddAssign for WrappingU32 {
    fn add_assign(&mut self, rhs: Self) {
        self.value = self.value.wrapping_add(rhs.value);
    }
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
