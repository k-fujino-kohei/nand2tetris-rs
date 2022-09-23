use super::bit_register::BitRegister;
use crate::ch_1::{Bit, Bit16};

pub struct Register {
    bits: [BitRegister; 16],
}

impl Register {
    pub fn new(v: Bit16) -> Self {
        Self {
            bits: v.map(BitRegister::new),
        }
    }

    pub fn sync(&mut self, input: Bit16, load: Bit) -> Bit16 {
        [
            self.bits[0].sync(input[0], load),
            self.bits[1].sync(input[1], load),
            self.bits[2].sync(input[2], load),
            self.bits[3].sync(input[3], load),
            self.bits[4].sync(input[4], load),
            self.bits[5].sync(input[5], load),
            self.bits[6].sync(input[6], load),
            self.bits[7].sync(input[7], load),
            self.bits[8].sync(input[8], load),
            self.bits[9].sync(input[9], load),
            self.bits[10].sync(input[10], load),
            self.bits[11].sync(input[11], load),
            self.bits[12].sync(input[12], load),
            self.bits[13].sync(input[13], load),
            self.bits[14].sync(input[14], load),
            self.bits[15].sync(input[15], load),
        ]
    }

    pub fn out(&self) -> Bit16 {
        [
            self.bits[0].out(),
            self.bits[1].out(),
            self.bits[2].out(),
            self.bits[3].out(),
            self.bits[4].out(),
            self.bits[5].out(),
            self.bits[6].out(),
            self.bits[7].out(),
            self.bits[8].out(),
            self.bits[9].out(),
            self.bits[10].out(),
            self.bits[11].out(),
            self.bits[12].out(),
            self.bits[13].out(),
            self.bits[14].out(),
            self.bits[15].out(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_cmp::ch_3::register::*;

    #[test]
    fn test_register() {
        let mut register = Register::new([0; 16]);
        for cmp in cmp() {
            assert_eq!(
                register.sync(cmp.r#in, cmp.load),
                cmp.out,
                "failed at time: {}",
                cmp.time
            );
        }
    }
}
