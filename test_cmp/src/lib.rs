#[allow(unused)]
macro_rules! bit {
    // $bit -> [u8; $digit]
    ($digit:literal, $bit:expr) => {{
        let mut r: [u8; $digit] = [0; $digit];
        format!("{}", $bit)
            .chars()
            .enumerate()
            .for_each(|(i, b)| r[i] = b.to_string().parse::<u8>().unwrap());
        r
    }};
}

pub mod ch_3;
