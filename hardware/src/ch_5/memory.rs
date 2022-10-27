use crate::{
    ch_1::{dmux, mux16, Bit, Bit14, Bit15, Bit16},
    ch_3::ram16k::RAM16K,
};

use super::{keyboard::Keyboard, screen::Screen};

#[derive(Default)]
pub struct Memory {
    ram: RAM16K,
    pub screen: Screen,
    keyboard: Keyboard,
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clock(&mut self, input: Bit16, load: Bit, address: Bit15) -> Bit16 {
        // Range of Address
        // memory          : Bit15 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        // end_of_memory   : Bit15 = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        // screen          : Bit15 = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        // end_of_screen   : Bit15 = [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        // keyboard        : Bit15 = [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        // end_of_keyboard : Bit15 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

        // address  [0] [1]
        // memory    0   _
        // screen    1   0
        // keyboard  1   1
        let (lm, load) = dmux(load, address[0]);
        let (ls, _lk) = dmux(load, address[1]);
        Self::select_out(
            self.ram.sync(
                input,
                [
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
                    address[13],
                    address[14],
                ],
                lm,
            ),
            self.screen.input(
                input,
                ls,
                [
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
                    address[14],
                ],
            ),
            self.keyboard.out(),
            address,
        )
    }

    pub fn out(&self, address: Bit15) -> Bit16 {
        Self::select_out(
            self.ram.out([
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
                address[13],
                address[14],
            ]),
            self.screen.out([
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
                address[14],
            ]),
            self.keyboard.out(),
            address,
        )
    }

    fn select_out(
        memory_out: Bit16,
        screen_out: Bit16,
        keyboard_out: Bit16,
        address: Bit15,
    ) -> Bit16 {
        mux16(
            &memory_out,
            &mux16(&screen_out, &keyboard_out, address[1]),
            address[0],
        )
    }

    pub fn set_ram(&mut self, input: Bit16, address: Bit14) {
        self.ram.sync(input, address, 1);
    }

    pub fn ram_out(&self, address: Bit14) -> Bit16 {
        self.ram.out(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_cmp::ch_5::memory::*;

    #[test]
    fn test_memory() {
        let mut memory = Memory::new();
        for (index, cmp) in cmp().iter().enumerate() {
            if index == 34 {
                memory.keyboard.input("K");
            }
            if index == 52 {
                memory.keyboard.input("Y");
            }
            let out = memory.clock(cmp.r#in, cmp.load, cmp.address);
            assert_eq!(cmp.out, out, "failed at line {}", index + 1);
        }
    }
}
