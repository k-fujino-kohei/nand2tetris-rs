use super::AdderOutput;
use crate::ch_1::{and, xor, Bit};

#[allow(dead_code)]
pub fn harf_adder(a: Bit, b: Bit) -> AdderOutput {
    AdderOutput {
        carry: and(a, b),
        sum: xor(a, b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch_1::Bit;
    use rstest::*;

    #[rstest(input, output,
        case((0, 0), (0, 0)),
        case((0, 1), (0, 1)),
        case((1, 0), (0, 1)),
        case((1, 1), (1, 0))
    )]
    fn test_harf_adder(input: (Bit, Bit), output: (Bit, Bit)) {
        assert_eq!(harf_adder(input.0, input.1), output.into());
    }
}
