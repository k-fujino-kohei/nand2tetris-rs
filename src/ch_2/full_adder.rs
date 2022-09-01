use super::{harf_adder::harf_adder, AdderOutput};
use crate::ch_1::{or, Bit};

#[allow(dead_code)]
pub fn full_adder(a: Bit, b: Bit, c: Bit) -> AdderOutput {
    let ab = harf_adder(a, b);
    let abc = harf_adder(ab.sum, c);
    AdderOutput {
        carry: or(ab.carry, abc.carry),
        sum: abc.sum,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[case((0, 0, 0), (0, 0))]
    #[case((0, 0, 1), (0, 1))]
    #[case((0, 1, 0), (0, 1))]
    #[case((0, 1, 1), (1, 0))]
    #[case((1, 0, 0), (0, 1))]
    #[case((1, 0, 1), (1, 0))]
    #[case((1, 1, 0), (1, 0))]
    #[case((1, 1, 1), (1, 1))]
    fn test_harf_adder(input: (Bit, Bit, Bit), output: (Bit, Bit)) {
        assert_eq!(full_adder(input.0, input.1, input.2), output.into());
    }
}
