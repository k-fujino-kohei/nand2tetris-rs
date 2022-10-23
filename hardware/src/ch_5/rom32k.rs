use crate::{
    ch_1::{mux8way16, Bit15, Bit16},
    ch_3::ram4k::RAM4K,
};

struct Rom32k {
    rams: [RAM4K; 8],
}

impl Rom32k {
    #[allow(dead_code)]
    pub fn new(v: [[[[[Bit16; 8]; 8]; 8]; 8]; 8]) -> Self {
        Self {
            rams: v.map(RAM4K::new),
        }
    }

    #[allow(dead_code)]
    pub fn out(&mut self, input: Bit15) -> Bit16 {
        let address = [
            input[3], input[4], input[5], input[6], input[7], input[8], input[9], input[10],
            input[11], input[12], input[13], input[14],
        ];
        mux8way16(
            &self.rams[0].out(address),
            &self.rams[1].out(address),
            &self.rams[2].out(address),
            &self.rams[3].out(address),
            &self.rams[4].out(address),
            &self.rams[5].out(address),
            &self.rams[6].out(address),
            &self.rams[7].out(address),
            &[input[0], input[1], input[2]],
        )
    }
}
