use crate::ch_1::{dmux8way, mux8way16, Bit, Bit16, Bit3};

use super::register::Register;

#[derive(Default)]
pub struct RAM8 {
    registers: [Register; 8],
}

impl RAM8 {
    pub fn new(v: [Bit16; 8]) -> Self {
        Self {
            registers: v.map(Register::new),
        }
    }

    pub fn sync(&mut self, input: Bit16, address: Bit3, load: Bit) -> Bit16 {
        let load = dmux8way(load, &address);
        mux8way16(
            &(&mut self.registers[0]).sync(input, load.0),
            &(&mut self.registers[1]).sync(input, load.1),
            &(&mut self.registers[2]).sync(input, load.2),
            &(&mut self.registers[3]).sync(input, load.3),
            &(&mut self.registers[4]).sync(input, load.4),
            &(&mut self.registers[5]).sync(input, load.5),
            &(&mut self.registers[6]).sync(input, load.6),
            &(&mut self.registers[7]).sync(input, load.7),
            &address,
        )
    }

    pub fn out(&self, address: Bit3) -> Bit16 {
        mux8way16(
            &(self.registers[0].out()),
            &(self.registers[1].out()),
            &(self.registers[2].out()),
            &(self.registers[3].out()),
            &(self.registers[4].out()),
            &(self.registers[5].out()),
            &(self.registers[6].out()),
            &(self.registers[7].out()),
            &address,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_cmp::ch_3::ram8::*;

    #[test]
    fn test_ram8() {
        let mut ram8 = RAM8::new([[0; 16]; 8]);
        for cmp in cmp() {
            let fact = if cmp.clock() {
                ram8.sync(cmp.r#in, cmp.address, cmp.load)
            } else {
                ram8.out(cmp.address)
            };
            assert_eq!(fact, cmp.out, "failed at time: {}", cmp.time);
        }
    }
}
