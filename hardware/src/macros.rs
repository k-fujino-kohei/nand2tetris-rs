// #[allow(unused)]
#[macro_export]
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

/// `[u8]` to `String`
macro_rules! format_bit {
    ($bin:expr) => {{
        let mut buf = String::with_capacity($bin.len());
        $bin.iter().for_each(|b| buf.push_str(&b.to_string()));
        buf
    }};
}

/// `[u8; 16]` to `i16`
macro_rules! i16_from_bit16 {
    ($bin:expr) => {
        i32::from_str_radix(&format_bit!($bin), 2).unwrap()
    };
}

#[macro_export]
/// Load the *.hack* file
macro_rules! load_hack {
    ($file:literal) => {
        include_str!($file)
            .split('\n')
            .into_iter()
            .map(|a| $crate::bit!(16, a))
            .collect::<Vec<_>>()
    };
}
