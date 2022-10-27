use crate::{
    ch_1::{dmux8way, mux8way16, Bit15, Bit16},
    ch_2::inc16,
    ch_3::ram4k::RAM4K,
};

pub struct Rom32k {
    // use Vec for avoid stack overflow
    rams: Vec<RAM4K>,
}

impl Default for Rom32k {
    fn default() -> Self {
        Self {
            rams: vec![
                RAM4K::default(),
                RAM4K::default(),
                RAM4K::default(),
                RAM4K::default(),
                RAM4K::default(),
                RAM4K::default(),
                RAM4K::default(),
                RAM4K::default(),
            ],
        }
    }
}

impl Rom32k {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self, data: Vec<[u8; 16]>) {
        // NOTE: address[0] is unused.
        let mut address: Bit16 = [0; 16];
        for input in data {
            let load = dmux8way(1, &[address[1], address[2], address[3]]);
            let input_address = [
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
                address[14],
                address[15],
            ];
            self.rams[0].sync(input, input_address, load.0);
            self.rams[1].sync(input, input_address, load.1);
            self.rams[2].sync(input, input_address, load.2);
            self.rams[3].sync(input, input_address, load.3);
            self.rams[4].sync(input, input_address, load.4);
            self.rams[5].sync(input, input_address, load.5);
            self.rams[6].sync(input, input_address, load.6);
            self.rams[7].sync(input, input_address, load.7);
            address = inc16(address);
        }
    }

    #[allow(dead_code)]
    pub fn out(&mut self, address: Bit15) -> Bit16 {
        let address = [
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
            address[14],
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
            &[address[0], address[1], address[2]],
        )
    }
}
