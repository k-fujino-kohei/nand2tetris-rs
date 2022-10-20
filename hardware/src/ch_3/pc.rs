use super::register::Register;
use crate::{
    ch_1::{mux16, Bit, Bit16},
    ch_2::inc16,
};

#[derive(Default)]
#[allow(dead_code)]
pub struct Pc {
    register: Register,
}

impl Pc {
    #[allow(dead_code)]
    pub fn new(v: [u8; 16]) -> Self {
        Self {
            register: Register::new(v),
        }
    }

    #[allow(dead_code)]
    pub fn sync(&mut self, input: Bit16, inc: Bit, load: Bit, reset: Bit) -> Bit16 {
        #[rustfmt::skip]
        let out =
            mux16(
                &mux16(
                    &mux16(
                        &self.register.sync([0; 16], 0),
                        &inc16(self.register.sync([0; 16], 0)),
                        inc
                    ),
                    &input,
                    load
                ),
                &[0; 16],
                reset,
            );
        self.register.sync(out, 1)
    }

    #[allow(dead_code)]
    pub fn out(&self) -> Bit16 {
        self.register.out()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_cmp::ch_3::pc::*;

    #[test]
    fn test_pc() {
        let mut pc = Pc::new([0; 16]);
        for cmp in cmp() {
            let fact = if cmp.clock() {
                pc.sync(cmp.r#in, cmp.inc, cmp.load, cmp.reset)
            } else {
                pc.out()
            };
            assert_eq!(fact, cmp.out, "failed at time: {}", cmp.time);
        }
    }
}
