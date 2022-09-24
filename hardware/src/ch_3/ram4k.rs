use super::ram512::RAM512;
use crate::ch_1::{dmux8way, mux8way16, Bit, Bit12, Bit16};

#[derive(Default)]
pub struct RAM4K {
    rams: [RAM512; 8],
}

impl RAM4K {
    #[allow(dead_code)]
    pub fn sync(&mut self, input: Bit16, address: Bit12, load: Bit) -> Bit16 {
        let higher = [address[0], address[1], address[2]];
        let lower = [
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
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
    pub fn out(&self, address: Bit12) -> Bit16 {
        let higher = [address[0], address[1], address[2]];
        let lower = [
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
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
    use test_cmp::ch_3::ram4k::*;

    #[test]
    fn test_ram4k() {
        let mut ram4k = RAM4K::default();
        for cmp in cmp() {
            let fact = if cmp.clock() {
                ram4k.sync(cmp.r#in, cmp.address, cmp.load)
            } else {
                ram4k.out(cmp.address)
            };
            assert_eq!(fact, cmp.out, "failed at time: {}", cmp.time);
        }
    }
}
