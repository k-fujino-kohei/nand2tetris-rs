use crate::ch_1::Bit16;

#[allow(dead_code)]
struct Keyboard;

impl Keyboard {
    #[allow(dead_code)]
    pub fn input(key: &str) -> Bit16 {
        let key = format!("{:>016b}", Keyboard::keymap(key));

        let mut r: Bit16 = [0; 16];
        key.chars()
            .enumerate()
            .for_each(|(i, b)| r[i] = b.to_string().parse::<u8>().unwrap());
        r
    }

    fn keymap(key: &str) -> u16 {
        match key {
            "newline" => 128,
            "backspace" => 129,
            "leftarrow" => 130,
            "uparrow" => 131,
            "rightarrow" => 132,
            "downarrow" => 133,
            "home" => 134,
            "end" => 135,
            "pageup" => 136,
            "pagedown" => 137,
            "insert" => 138,
            "delete" => 139,
            "esc" => 140,
            "f1" => 141,
            "f2" => 142,
            "f3" => 143,
            "f4" => 144,
            "f5" => 145,
            "f6" => 146,
            "f7" => 147,
            "f8" => 148,
            "f9" => 149,
            "f10" => 150,
            "f11" => 151,
            "f12" => 152,
            _ => {
                let bytes = key.as_bytes();
                if bytes.len() > 1 {
                    panic!("unexpected key. {}", key)
                }
                bytes[0] as u16
            }
        }
    }
}
