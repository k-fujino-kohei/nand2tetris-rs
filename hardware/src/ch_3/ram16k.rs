use super::ram4k::RAM4K;
use crate::ch_1::{dmux4way, mux4way16, Bit, Bit14, Bit16};

#[derive(Default)]
pub struct RAM16K {
    rams: [RAM4K; 4],
}

impl RAM16K {
    #[allow(dead_code)]
    pub fn sync(&mut self, input: Bit16, address: Bit14, load: Bit) -> Bit16 {
        let higher = [address[0], address[1]];
        let lower = [
            address[2],
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
            address[12],
            address[13],
        ];
        let load = dmux4way(load, &higher);
        mux4way16(
            &(&mut self.rams[0]).sync(input, lower, load.0),
            &(&mut self.rams[1]).sync(input, lower, load.1),
            &(&mut self.rams[2]).sync(input, lower, load.2),
            &(&mut self.rams[3]).sync(input, lower, load.3),
            &higher,
        )
    }

    #[allow(dead_code)]
    pub fn out(&self, address: Bit14) -> Bit16 {
        let higher = [address[0], address[1]];
        let lower = [
            address[2],
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
            address[12],
            address[13],
        ];
        mux4way16(
            &(self.rams[0]).out(lower),
            &(self.rams[1]).out(lower),
            &(self.rams[2]).out(lower),
            &(self.rams[3]).out(lower),
            &higher,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_cmp::ch_3::ram16k::*;

    #[test]
    fn test_ram16k() {
        let mut ram16k = RAM16K::default();
        for cmp in cmp() {
            let fact = if cmp.clock() {
                ram16k.sync(cmp.r#in, cmp.address, cmp.load)
            } else {
                ram16k.out(cmp.address)
            };
            assert_eq!(fact, cmp.out, "failed at time: {}", cmp.time);
        }
    }
}
