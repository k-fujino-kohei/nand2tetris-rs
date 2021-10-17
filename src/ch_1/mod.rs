mod basic;
mod multi_bit;
mod multi_input;

pub use basic::*;
pub use multi_bit::*;
pub use multi_input::*;

pub type Bit = u8;
pub type Bit2 = [Bit; 2];
pub type Bit3 = [Bit; 3];
pub type Bit8 = [Bit; 8];
pub type Bit16 = [Bit; 16];

/// the primitive gate
pub fn nand(a: Bit, b: Bit) -> Bit {
    bool2bit(!(bit2bool(a) && bit2bool(b)))
}

fn bit2bool(bit: Bit) -> bool {
    if bit == 1 { return true }
    if bit == 0 { return false }
    panic!("bit must be `0` or `1`")
}

fn bool2bit(bit: bool) -> Bit {
    if bit { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest(input, output,
        case((0, 0), 1),
        case((0, 1), 1),
        case((1, 0), 1),
        case((1, 1), 0),
    )]
    fn test_nand(input: (Bit, Bit), output: Bit) {
        assert_eq!(nand(input.0, input.1), output);
    }

    #[rstest(input, output,
        case(0, false),
        case(1, true),
        #[should_panic]
        case(2, true),
        #[should_panic]
        case(3, false),
    )]
    fn test_bit2bool(input: Bit, output: bool) {
        assert_eq!(bit2bool(input), output);
    }

    #[rstest(input, output,
        case(false, 0),
        case(true, 1),
    )]
    fn test_bool2bit(input: bool, output: Bit) {
        assert_eq!(bool2bit(input), output);
    }
}
