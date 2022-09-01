use super::full_adder::full_adder;
use crate::ch_1::Bit16;

#[allow(dead_code)]
fn add16(a: Bit16, b: Bit16) -> Bit16 {
    let r1 = full_adder(0, a[15], b[15]);
    let r2 = full_adder(r1.carry, a[14], b[14]);
    let r3 = full_adder(r2.carry, a[13], b[13]);
    let r4 = full_adder(r3.carry, a[12], b[12]);
    let r5 = full_adder(r4.carry, a[11], b[11]);
    let r6 = full_adder(r5.carry, a[10], b[10]);
    let r7 = full_adder(r6.carry, a[9], b[9]);
    let r8 = full_adder(r7.carry, a[8], b[8]);
    let r9 = full_adder(r8.carry, a[7], b[7]);
    let r10 = full_adder(r9.carry, a[6], b[6]);
    let r11 = full_adder(r10.carry, a[5], b[5]);
    let r12 = full_adder(r11.carry, a[4], b[4]);
    let r13 = full_adder(r12.carry, a[3], b[3]);
    let r14 = full_adder(r13.carry, a[2], b[2]);
    let r15 = full_adder(r14.carry, a[1], b[1]);
    let r16 = full_adder(r15.carry, a[0], b[0]);
    [
        r16.sum, r15.sum, r14.sum, r13.sum, r12.sum, r11.sum, r10.sum, r9.sum, r8.sum, r7.sum,
        r6.sum, r5.sum, r4.sum, r3.sum, r2.sum, r1.sum,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[case((
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1]
    ),  [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0])]
    fn test_add16(input: (Bit16, Bit16), output: Bit16) {
        assert_eq!(add16(input.0, input.1), output);
    }
}
