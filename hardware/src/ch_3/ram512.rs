use super::ram64::RAM64;
use crate::ch_1::{dmux8way, mux8way16, Bit, Bit16, Bit9};

#[derive(Default)]
#[allow(dead_code)]
pub struct RAM512 {
    rams: [RAM64; 8],
}

impl RAM512 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            rams: [
                RAM64::default(),
                RAM64::default(),
                RAM64::default(),
                RAM64::default(),
                RAM64::default(),
                RAM64::default(),
                RAM64::default(),
                RAM64::default(),
            ],
        }
    }

    #[allow(dead_code)]
    pub fn sync(&mut self, input: Bit16, address: Bit9, load: Bit) -> Bit16 {
        let higher = [address[0], address[1], address[2]];
        let lower = [
            address[3], address[4], address[5], address[6], address[7], address[8],
        ];
        let load = dmux8way(load, &higher);
        mux8way16(
            &(&mut self.rams[0]).sync(input, lower, load.0),
            &(&mut self.rams[1]).sync(input, lower, load.1),
            &(&mut self.rams[2]).sync(input, lower, load.2),
            &(&mut self.rams[3]).sync(input, lower, load.3),
            &(&mut self.rams[4]).sync(input, lower, load.4),
            &(&mut self.rams[5]).sync(input, lower, load.5),
            &(&mut self.rams[6]).sync(input, lower, load.6),
            &(&mut self.rams[7]).sync(input, lower, load.7),
            &higher,
        )
    }

    #[allow(dead_code)]
    pub fn out(&self, address: Bit9) -> Bit16 {
        let higher = [address[0], address[1], address[2]];
        let lower = [
            address[3], address[4], address[5], address[6], address[7], address[8],
        ];
        mux8way16(
            &(self.rams[0]).out(lower),
            &(self.rams[1]).out(lower),
            &(self.rams[2]).out(lower),
            &(self.rams[3]).out(lower),
            &(self.rams[4]).out(lower),
            &(self.rams[5]).out(lower),
            &(self.rams[6]).out(lower),
            &(self.rams[7]).out(lower),
            &higher,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_cmp::ch_3::ram512::*;

    #[test]
    fn test_ram512() {
        let mut ram512 = RAM512::new();
        for cmp in cmp() {
            let fact = if cmp.clock() {
                ram512.sync(cmp.r#in, cmp.address, cmp.load)
            } else {
                ram512.out(cmp.address)
            };
            assert_eq!(fact, cmp.out, "failed at time: {}", cmp.time);
        }
    }
}
