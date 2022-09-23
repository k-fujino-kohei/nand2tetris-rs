use super::ram8::RAM8;
use crate::ch_1::{dmux8way, mux8way16, Bit, Bit16, Bit6};

#[allow(dead_code)]
pub struct RAM64 {
    rams: [RAM8; 8],
}

impl RAM64 {
    #[allow(dead_code)]
    pub fn new(v: [[Bit16; 8]; 8]) -> Self {
        Self {
            rams: v.map(RAM8::new),
        }
    }

    #[allow(dead_code)]
    pub fn sync(&mut self, input: Bit16, address: Bit6, load: Bit) -> Bit16 {
        let load = dmux8way(load, &[address[0], address[1], address[2]]);
        mux8way16(
            &(&mut self.rams[0]).sync(input, [address[3], address[4], address[5]], load.0),
            &(&mut self.rams[1]).sync(input, [address[3], address[4], address[5]], load.1),
            &(&mut self.rams[2]).sync(input, [address[3], address[4], address[5]], load.2),
            &(&mut self.rams[3]).sync(input, [address[3], address[4], address[5]], load.3),
            &(&mut self.rams[4]).sync(input, [address[3], address[4], address[5]], load.4),
            &(&mut self.rams[5]).sync(input, [address[3], address[4], address[5]], load.5),
            &(&mut self.rams[6]).sync(input, [address[3], address[4], address[5]], load.6),
            &(&mut self.rams[7]).sync(input, [address[3], address[4], address[5]], load.7),
            &[address[0], address[1], address[2]],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_cmp::ch_3::ram64::*;

    #[test]
    fn test_ram8() {
        let mut ram64 = RAM64::new([[[0; 16]; 8]; 8]);
        for cmp in cmp() {
            assert_eq!(
                ram64.sync(cmp.r#in, cmp.address, cmp.load),
                cmp.out,
                "failed at time: {}",
                cmp.time
            );
        }
    }
}
