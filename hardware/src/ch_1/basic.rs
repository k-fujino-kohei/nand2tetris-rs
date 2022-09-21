use super::{nand, Bit};

#[allow(dead_code)]
pub fn not(a: Bit) -> Bit {
    nand(a, a)
}

#[allow(dead_code)]
pub fn and(a: Bit, b: Bit) -> Bit {
    nand(nand(a, b), nand(a, b))
}

#[allow(dead_code)]
pub fn or(a: Bit, b: Bit) -> Bit {
    nand(not(a), not(b))
}

#[allow(dead_code)]
pub fn xor(a: Bit, b: Bit) -> Bit {
    and(nand(a, b), or(a, b))
}

#[allow(dead_code)]
pub fn mux(a: Bit, b: Bit, sel: Bit) -> Bit {
    nand(nand(a, not(sel)), nand(sel, b))
}

#[allow(dead_code)]
pub fn dmux(a: Bit, sel: Bit) -> (Bit, Bit) {
    let w = and(a, sel);
    (and(a, not(w)), w)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[case(0, 1)]
    #[case(1, 0)]
    fn test_not(input: Bit, output: Bit) {
        assert_eq!(not(input), output)
    }

    #[case((0, 0), 0)]
    #[case((0, 1), 0)]
    #[case((1, 0), 0)]
    #[case((1, 1), 1)]
    fn test_and(input: (Bit, Bit), output: Bit) {
        assert_eq!(and(input.0, input.1), output);
    }

    #[case((0, 0), 0)]
    #[case((0, 1), 1)]
    #[case((1, 0), 1)]
    #[case((1, 1), 1)]
    fn test_or(input: (Bit, Bit), output: Bit) {
        assert_eq!(or(input.0, input.1), output);
    }

    #[case((0, 0), 0)]
    #[case((0, 1), 1)]
    #[case((1, 0), 1)]
    #[case((1, 1), 0)]
    fn test_xor(input: (Bit, Bit), output: Bit) {
        assert_eq!(xor(input.0, input.1), output);
    }

    #[case((0, 0, 0), 0)]
    #[case((0, 1, 0), 0)]
    #[case((1, 0, 0), 1)]
    #[case((1, 1, 0), 1)]
    #[case((0, 0, 1), 0)]
    #[case((0, 1, 1), 1)]
    #[case((1, 0, 1), 0)]
    #[case((1, 1, 1), 1)]
    fn test_mux(input: (Bit, Bit, Bit), output: Bit) {
        assert_eq!(mux(input.0, input.1, input.2), output);
    }

    #[case((0, 0), (0, 0))]
    #[case((0, 1), (0, 0))]
    #[case((1, 0), (1, 0))]
    #[case((1, 1), (0, 1))]
    fn test_dmux(input: (Bit, Bit), output: (Bit, Bit)) {
        assert_eq!(dmux(input.0, input.1), (output.0, output.1));
    }
}
