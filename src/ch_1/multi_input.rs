use super::{Bit, Bit16, Bit8, Bit3, Bit2, dmux, mux16, or};

#[allow(dead_code)]
pub fn or8way(a: &Bit8) -> Bit {
    or(
        a[0],
        or(a[1], or(a[2], or(a[3], or(a[4], or(a[5], or(a[6], a[7])))))),
    )
}

#[allow(dead_code)]
pub fn mux4way16(a: &Bit16, b: &Bit16, c: &Bit16, d: &Bit16, sel: &Bit2) -> Bit16 {
    let ab = mux16(a, b, sel[1]);
    let cd = mux16(c, d, sel[1]);
    mux16(&ab, &cd, sel[0])
}

#[allow(dead_code)]
pub fn mux8way16(
    a: &Bit16,
    b: &Bit16,
    c: &Bit16,
    d: &Bit16,
    e: &Bit16,
    f: &Bit16,
    g: &Bit16,
    h: &Bit16,
    sel: &Bit3,
) -> Bit16 {
    let ab = mux16(a, b, sel[2]);
    let cd = mux16(c, d, sel[2]);
    let ef = mux16(e, f, sel[2]);
    let gh = mux16(g, h, sel[2]);
    let abcd = mux16(&ab, &cd, sel[1]);
    let efgh = mux16(&ef, &gh, sel[1]);
    mux16(&abcd, &efgh, sel[0])
}

#[allow(dead_code)]
pub fn dmux4way(input: Bit, sel: &Bit2) -> (Bit, Bit, Bit, Bit) {
    let (o1, o2) = dmux(input, sel[0]);
    let (a, b) = dmux(o1, sel[1]);
    let (c, d) = dmux(o2, sel[1]);
    (a, b, c, d)
}

#[allow(dead_code)]
pub fn dmux8way(input: Bit, sel: &Bit3) -> (Bit, Bit, Bit, Bit, Bit, Bit, Bit, Bit) {
    let (o1, o2, o3, o4) = dmux4way(input, &[sel[0], sel[1]]);
    let (a, b) = dmux(o1, sel[2]);
    let (c, d) = dmux(o2, sel[2]);
    let (e, f) = dmux(o3, sel[2]);
    let (g, h) = dmux(o4, sel[2]);
    (a, b, c, d, e, f, g, h)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest(input, output,
        case(&[0, 0, 0, 0, 0, 0, 0, 0], 0),
        case(&[1, 0, 0, 0, 0, 0, 0, 0], 1),
        case(&[1, 1, 0, 0, 0, 0, 0, 0], 1),
    )]
    fn test_or8way(input: &Bit8, output: Bit) {
        assert_eq!(or8way(input), output)
    }

    fn rand_bit16() -> Bit16 {
        rand::random::<[bool; 16]>().map(|a| if a { 1 } else { 0 })
    }

    #[fixture]
    fn a() -> Bit16 {
        rand_bit16()
    }

    #[fixture]
    fn b() -> Bit16 {
        rand_bit16()
    }

    #[fixture]
    fn c() -> Bit16 {
        rand_bit16()
    }

    #[fixture]
    fn d() -> Bit16 {
        rand_bit16()
    }

    #[fixture]
    fn e() -> Bit16 {
        rand_bit16()
    }

    #[fixture]
    fn f() -> Bit16 {
        rand_bit16()
    }

    #[fixture]
    fn g() -> Bit16 {
        rand_bit16()
    }

    #[fixture]
    fn h() -> Bit16 {
        rand_bit16()
    }

    #[rstest(sel, output,
        case([0, 0], a),
        case([0, 1], b),
        case([1, 0], c),
        case([1, 1], d)
    )]
    fn test_mux4way16(a: Bit16, b: Bit16, c: Bit16, d: Bit16, sel: Bit2, output: Bit16) {
        assert_eq!(mux4way16(&a, &b, &c, &d, &sel), output);
    }

    #[rstest(sel, output,
        case([0, 0, 0], a),
        case([0, 0, 1], b),
        case([0, 1, 0], c),
        case([0, 1, 1], d),
        case([1, 0, 0], e),
        case([1, 0, 1], f),
        case([1, 1, 0], g),
        case([1, 1, 1], h),
    )]
    fn test_mux8way16(
        a: Bit16,
        b: Bit16,
        c: Bit16,
        d: Bit16,
        e: Bit16,
        f: Bit16,
        g: Bit16,
        h: Bit16,
        sel: Bit3,
        output: Bit16,
    ) {
        assert_eq!(mux8way16(&a, &b, &c, &d, &e, &f, &g, &h, &sel), output);
    }

    #[rstest(input, sel, output,
        case(1, [0, 0], (1, 0, 0, 0)),
        case(0, [0, 0], (0, 0, 0, 0)),
        case(1, [0, 1], (0, 1, 0, 0)),
        case(0, [0, 1], (0, 0, 0, 0)),
        case(1, [1, 0], (0, 0, 1, 0)),
        case(0, [1, 0], (0, 0, 0, 0)),
        case(1, [1, 1], (0, 0, 0, 1)),
        case(0, [1, 1], (0, 0, 0, 0)),
    )]
    fn test_dmux4way(input: Bit, sel: Bit2, output: (Bit, Bit, Bit, Bit)) {
        assert_eq!(dmux4way(input, &sel), output);
    }

    #[rstest(input, sel, output,
        case(1, [0, 0], (1, 0, 0, 0)),
        case(0, [0, 0], (0, 0, 0, 0)),
        case(1, [0, 1], (0, 1, 0, 0)),
        case(0, [0, 1], (0, 0, 0, 0)),
        case(1, [1, 0], (0, 0, 1, 0)),
        case(0, [1, 0], (0, 0, 0, 0)),
        case(1, [1, 1], (0, 0, 0, 1)),
        case(0, [1, 1], (0, 0, 0, 0)),
    )]
    fn test_dmux4way2(input: Bit, sel: Bit2, output: (Bit, Bit, Bit, Bit)) {
        assert_eq!(dmux4way(input, &sel), output);
    }

    #[rstest(input, sel, output,
        case(1, [0, 0, 0], (1, 0, 0, 0, 0, 0, 0, 0)),
        case(0, [0, 0, 0], (0, 0, 0, 0, 0, 0, 0, 0)),
        case(1, [0, 0, 1], (0, 1, 0, 0, 0, 0, 0, 0)),
        case(0, [0, 0, 1], (0, 0, 0, 0, 0, 0, 0, 0)),
        case(1, [0, 1, 0], (0, 0, 1, 0, 0, 0, 0, 0)),
        case(0, [0, 1, 0], (0, 0, 0, 0, 0, 0, 0, 0)),
        case(1, [0, 1, 1], (0, 0, 0, 1, 0, 0, 0, 0)),
        case(0, [0, 1, 1], (0, 0, 0, 0, 0, 0, 0, 0)),
        case(1, [1, 0, 0], (0, 0, 0, 0, 1, 0, 0, 0)),
        case(0, [1, 0, 0], (0, 0, 0, 0, 0, 0, 0, 0)),
        case(1, [1, 0, 1], (0, 0, 0, 0, 0, 1, 0, 0)),
        case(0, [1, 0, 1], (0, 0, 0, 0, 0, 0, 0, 0)),
        case(1, [1, 1, 0], (0, 0, 0, 0, 0, 0, 1, 0)),
        case(0, [1, 1, 0], (0, 0, 0, 0, 0, 0, 0, 0)),
        case(1, [1, 1, 1], (0, 0, 0, 0, 0, 0, 0, 1)),
        case(0, [1, 1, 1], (0, 0, 0, 0, 0, 0, 0, 0)),
    )]
    fn test_dmux8way(input: Bit, sel: Bit3, output: (Bit, Bit, Bit, Bit, Bit, Bit, Bit, Bit)) {
        assert_eq!(dmux8way(input, &sel), output);
    }
}
