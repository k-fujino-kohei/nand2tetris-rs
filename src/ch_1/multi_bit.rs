use super::{and, mux, not, or, xor, Bit, Bit16};

#[allow(dead_code)]
pub fn not16(a: &Bit16) -> Bit16 {
    [
        not(a[0]),
        not(a[1]),
        not(a[2]),
        not(a[3]),
        not(a[4]),
        not(a[5]),
        not(a[6]),
        not(a[7]),
        not(a[8]),
        not(a[9]),
        not(a[10]),
        not(a[11]),
        not(a[12]),
        not(a[13]),
        not(a[14]),
        not(a[15]),
    ]
}

#[allow(dead_code)]
pub fn and16(a: &Bit16, b: &Bit16) -> Bit16 {
    [
        and(a[0], b[0]),
        and(a[1], b[1]),
        and(a[2], b[2]),
        and(a[3], b[3]),
        and(a[4], b[4]),
        and(a[5], b[5]),
        and(a[6], b[6]),
        and(a[7], b[7]),
        and(a[8], b[8]),
        and(a[9], b[9]),
        and(a[10], b[10]),
        and(a[11], b[11]),
        and(a[12], b[12]),
        and(a[13], b[13]),
        and(a[14], b[14]),
        and(a[15], b[15]),
    ]
}

#[allow(dead_code)]
pub fn or16(a: &Bit16, b: &Bit16) -> Bit16 {
    [
        or(a[0], b[0]),
        or(a[1], b[1]),
        or(a[2], b[2]),
        or(a[3], b[3]),
        or(a[4], b[4]),
        or(a[5], b[5]),
        or(a[6], b[6]),
        or(a[7], b[7]),
        or(a[8], b[8]),
        or(a[9], b[9]),
        or(a[10], b[10]),
        or(a[11], b[11]),
        or(a[12], b[12]),
        or(a[13], b[13]),
        or(a[14], b[14]),
        or(a[15], b[15]),
    ]
}

#[allow(dead_code)]
pub fn xor16(a: &Bit16, b: &Bit16) -> Bit16 {
    [
        xor(a[0], b[0]),
        xor(a[1], b[1]),
        xor(a[2], b[2]),
        xor(a[3], b[3]),
        xor(a[4], b[4]),
        xor(a[5], b[5]),
        xor(a[6], b[6]),
        xor(a[7], b[7]),
        xor(a[8], b[8]),
        xor(a[9], b[9]),
        xor(a[10], b[10]),
        xor(a[11], b[11]),
        xor(a[12], b[12]),
        xor(a[13], b[13]),
        xor(a[14], b[14]),
        xor(a[15], b[15]),
    ]
}

#[allow(dead_code)]
pub fn mux16(a: &Bit16, b: &Bit16, sel: Bit) -> Bit16 {
    [
        mux(a[0], b[0], sel),
        mux(a[1], b[1], sel),
        mux(a[2], b[2], sel),
        mux(a[3], b[3], sel),
        mux(a[4], b[4], sel),
        mux(a[5], b[5], sel),
        mux(a[6], b[6], sel),
        mux(a[7], b[7], sel),
        mux(a[8], b[8], sel),
        mux(a[9], b[9], sel),
        mux(a[10], b[10], sel),
        mux(a[11], b[11], sel),
        mux(a[12], b[12], sel),
        mux(a[13], b[13], sel),
        mux(a[14], b[14], sel),
        mux(a[15], b[15], sel),
    ]
}
