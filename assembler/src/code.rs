#[rustfmt::skip]
pub fn comp_code(ident: &str, a: bool) -> String {
    if a {
        match ident {
            "M"     => String::from("110000"),
            "!M"    => String::from("110001"),
            "-M"    => String::from("110011"),
            "M+1"   => String::from("110111"),
            "M-1"   => String::from("110010"),
            "D+M"   => String::from("000010"),
            "D-M"   => String::from("010011"),
            "M-D"   => String::from("000111"),
            "D&M"   => String::from("000000"),
            "D|M"   => String::from("010101"),
            _       => panic!("unexpected cmp: {} if a={}", ident, a),
        }
    } else {
        match ident {
            "0"     => String::from("101010"),
            "1"     => String::from("111111"),
            "-1"    => String::from("111010"),
            "D"     => String::from("001100"),
            "A"     => String::from("110000"),
            "!D"    => String::from("001101"),
            "!A"    => String::from("110001"),
            "-D"    => String::from("001111"),
            "-A"    => String::from("110011"),
            "D+1"   => String::from("011111"),
            "A+1"   => String::from("110111"),
            "D-1"   => String::from("001110"),
            "A-1"   => String::from("110010"),
            "D+A"   => String::from("000010"),
            "D-A"   => String::from("010011"),
            "A-D"   => String::from("000111"),
            "D&A"   => String::from("000000"),
            "D|A"   => String::from("010101"),
            _       => panic!("unexpected cmp: {} if a={}", ident, a),
        }
    }
}

#[rustfmt::skip]
pub fn dest_code(ident: &str) -> String {
    match ident {
        "M"   => String::from("001"),
        "D"   => String::from("010"),
        "MD"  => String::from("011"),
        "A"   => String::from("100"),
        "AM"  => String::from("101"),
        "AD"  => String::from("110"),
        "AMD" => String::from("111"),
        ""    => String::from("000"),
        _     => panic!("unexpected dest: {}", ident),
    }
}

#[rustfmt::skip]
pub fn jump_code(ident: &str) -> String {
    match ident {
        "JGT" => String::from("001"),
        "JEQ" => String::from("010"),
        "JGE" => String::from("011"),
        "JLT" => String::from("100"),
        "JNE" => String::from("101"),
        "JLE" => String::from("110"),
        "JMP" => String::from("111"),
        ""    => String::from("000"),
        _     => panic!("unexpected jump: {}", ident),
    }
}
