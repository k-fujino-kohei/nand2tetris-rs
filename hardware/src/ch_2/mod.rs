use crate::ch_1::Bit;

mod adder;
mod alu;
mod full_adder;
mod harf_adder;
mod inc;

pub use adder::*;
pub use alu::*;
pub use full_adder::*;
pub use harf_adder::*;
pub use inc::*;

#[derive(Debug, PartialEq, Eq)]
pub struct AdderOutput {
    pub carry: Bit,
    pub sum: Bit,
}

impl From<(Bit, Bit)> for AdderOutput {
    fn from(output: (Bit, Bit)) -> Self {
        Self {
            carry: output.0,
            sum: output.1,
        }
    }
}
