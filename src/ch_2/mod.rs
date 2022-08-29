use crate::ch_1::Bit;

mod harf_adder;

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
