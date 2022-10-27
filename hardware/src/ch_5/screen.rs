use crate::{
    ch_1::{dmux, mux16, Bit, Bit13, Bit16},
    ch_3::ram4k::RAM4K,
};

#[derive(Default)]
pub struct Screen {
    rams: [RAM4K; 2],
}

impl Screen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input(&mut self, input: Bit16, load: Bit, address: Bit13) -> Bit16 {
        let (load_a, load_b) = dmux(load, address[0]);
        let lower = [
            address[1],
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
        ];
        mux16(
            &self.rams[0].sync(input, lower, load_a),
            &self.rams[1].sync(input, lower, load_b),
            address[0],
        );
        self.out(address)
    }

    pub fn out(&self, address: Bit13) -> Bit16 {
        let lower = [
            address[1],
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
        ];
        mux16(
            &self.rams[0].out(lower),
            &self.rams[1].out(lower),
            address[0],
        )
    }
}
