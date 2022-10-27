use super::{cpu::CPU, memory::Memory, rom32k::Rom32k};
use crate::ch_1::{Bit, Bit14, Bit16};

#[derive(Default)]
pub struct Computer {
    cpu: CPU,
    memory: Memory,
    rom: Rom32k,
    pub debug: bool,
}

impl Computer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_program(&mut self, data: Vec<[u8; 16]>) {
        self.rom.load(data);
    }

    pub fn run(&mut self, reset: Bit, repeat: u64) {
        let mut i = 1;
        loop {
            self.clock(reset);

            if self.debug {
                println!("{i:<04}> {:?}", self.cpu);
            }

            i += 1;
            if i > repeat {
                break;
            }
        }
    }

    pub fn clock(&mut self, reset: Bit) {
        let cpu = {
            let in_m = self.memory.out(self.cpu.address_m());
            let instruction = self.rom.out(self.cpu.pc());
            self.cpu.clock(in_m, instruction, reset);
            self.cpu.out(in_m, instruction, reset)
        };
        self.memory.clock(cpu.out_m, cpu.write_m, cpu.address_m);
    }
}

// for Debug
impl Computer {
    pub fn dbg_set_ram(&mut self, input: Bit16, address: Bit14) {
        if self.debug {
            self.memory.set_ram(input, address)
        }
    }

    pub fn dbg_print_ram_out(&self, address: Bit14) {
        if self.debug {
            let ram = self.memory.ram_out(address);
            let ram_str = format_bit!(ram);
            let ram_num = i16_from_bit16!(ram);
            let address = format_bit!(address);
            println!(r#"RAM[{address}] {ram_str} : {ram_num}"#)
        }
    }

    pub fn ram(&self, address: Bit14) -> Bit16 {
        self.memory.ram_out(address)
    }
}
