use crate::ch_1::Bit;

#[allow(clippy::upper_case_acronyms)]
pub struct DFF(Bit);

impl DFF {
    pub fn new(v: Bit) -> Self {
        Self(v)
    }

    pub fn out(&self) -> Bit {
        self.0
    }

    pub fn sync(&mut self, input: Bit) -> Bit {
        let out = self.0;
        self.0 = input;
        out
    }
}
