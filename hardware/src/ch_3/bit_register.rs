use super::dff::DFF;
use crate::ch_1::{mux, Bit};

#[derive(Default)]
pub struct BitRegister {
    dff: DFF,
}

impl BitRegister {
    pub fn new(v: Bit) -> Self {
        Self { dff: DFF::new(v) }
    }

    pub fn sync(&mut self, input: Bit, load: Bit) -> Bit {
        let out = mux(self.dff.out(), input, load);
        self.dff.sync(out)
    }

    pub fn out(&self) -> Bit {
        self.dff.out()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_cmp::ch_3::bit::*;

    #[test]
    fn test_bit_register() {
        let mut bit = BitRegister::new(0);
        for cmp in cmp() {
            assert_eq!(
                bit.sync(cmp.r#in, cmp.load),
                cmp.out,
                "failed at time: {}",
                cmp.time
            );
        }
    }
}
