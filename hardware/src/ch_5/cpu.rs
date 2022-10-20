use crate::{
    ch_1::{and, mux16, not, or, Bit, Bit15, Bit16, Bit2, Bit3, Bit6},
    ch_2::{alu, ALUOutput},
    ch_3::{pc::Pc, register::Register},
};

#[derive(Default)]
#[allow(dead_code, clippy::upper_case_acronyms)]
pub struct CPU {
    a_register: Register,
    d_register: Register,
    pc: Pc,
}

pub struct CPUOutput {
    pub out_m: Bit16,
    pub write_m: Bit,
    pub address_m: Bit15,
    pub pc: Bit15,
}

#[allow(dead_code)]
struct Instruction {
    i: Bit,
    xx: Bit2,
    a: Bit,
    cccccc: Bit6,
    ddd: Bit3,
    jjj: Bit3,
}

impl CPU {
    #[allow(dead_code)]
    pub fn out(&self, in_m: Bit16, instruction_raw: Bit16, _reset: Bit) -> CPUOutput {
        let instruction = self.decode(instruction_raw);
        let (i, ddd) = (instruction.i, instruction.ddd);
        let address_m = {
            let a = self.a_register.out();
            [
                a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13],
                a[14], a[15],
            ]
        };
        let pc = {
            let pc = self.pc.out();
            [
                pc[1], pc[2], pc[3], pc[4], pc[5], pc[6], pc[7], pc[8], pc[9], pc[10], pc[11],
                pc[12], pc[13], pc[14], pc[15],
            ]
        };
        CPUOutput {
            out_m: self.alu(in_m, instruction).out,
            write_m: and(i, ddd[2]),
            address_m,
            pc,
        }
    }

    #[allow(dead_code)]
    pub fn clock(&mut self, in_m: Bit16, instruction_raw: Bit16, reset: Bit) {
        let instruction = self.decode(instruction_raw);
        let (i, ddd, jjj) = (instruction.i, instruction.ddd, instruction.jjj);
        let ALUOutput { out: out_m, zr, ng } = self.alu(in_m, instruction);
        let in_a = mux16(&instruction_raw, &out_m, i);
        let in_d = out_m;
        let in_pc = self.a_register.out();
        let load_a = or(not(i), ddd[0]);
        let load_d = and(i, ddd[1]);
        #[rustfmt::skip]
        let jump = and(i, or(or(
            and(jjj[0], ng),
            and(jjj[1], zr)),
            and(jjj[2], not(or(ng, zr))
        )));
        self.a_register.sync(in_a, load_a);
        self.d_register.sync(in_d, load_d);
        self.pc.sync(in_pc, not(jump), jump, reset);
    }

    fn alu(&self, in_m: Bit16, instruction: Instruction) -> ALUOutput {
        let (a, cccccc) = (instruction.a, instruction.cccccc);
        alu(
            (
                self.d_register.out(),
                mux16(&self.a_register.out(), &in_m, a),
            ),
            (
                cccccc[0], cccccc[1], cccccc[2], cccccc[3], cccccc[4], cccccc[5],
            ),
        )
    }

    fn decode(&self, instruction: Bit16) -> Instruction {
        let i = instruction[0];
        let xx = [instruction[1], instruction[2]];
        let a = instruction[3];
        let cccccc = [
            instruction[4],
            instruction[5],
            instruction[6],
            instruction[7],
            instruction[8],
            instruction[9],
        ];
        let ddd = [instruction[10], instruction[11], instruction[12]];
        let jjj = [instruction[13], instruction[14], instruction[15]];
        Instruction {
            i,
            xx,
            a,
            cccccc,
            ddd,
            jjj,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_cmp::ch_5::cpu::*;

    #[test]
    fn test_cpu() {
        let mut cpu = CPU::default();
        for cmp in cmp() {
            if cmp.clock() {
                cpu.clock(cmp.in_m, cmp.instruction, cmp.reset);
            } else {
                let output = cpu.out(cmp.in_m, cmp.instruction, cmp.reset);
                if let Some(cmp_out_m) = cmp.out_m {
                    assert_eq!(
                        output.out_m, cmp_out_m,
                        "out_m: failed at time: {}",
                        cmp.time
                    );
                }
                assert_eq!(
                    output.write_m, cmp.write_m,
                    "write_m: failed at time: {}",
                    cmp.time
                );
                assert_eq!(
                    output.address_m, cmp.addre,
                    "address_m: failed at time: {}",
                    cmp.time
                );
                assert_eq!(output.pc, cmp.pc, "pc: failed at time: {}", cmp.time);
            }
        }
    }
}
