use hardware::{bit, load_hack, Computer};

pub fn main() {
    let data = load_hack!("Max.hack");
    let mut computer = Computer::new();
    computer.debug = true;
    computer.load_program(data);
    // RAM[0] = 3
    computer.dbg_set_ram(bit!(16, "0000000000000011"), bit!(14, "00000000000000"));
    // RAM[1] = 5
    computer.dbg_set_ram(bit!(16, "0000000000000101"), bit!(14, "00000000000001"));
    computer.run(0, 14);
    // expect RAM[2] == 5
    computer.dbg_print_ram_out(bit!(14, "00000000000010"));

    // reset
    computer.run(1, 1);

    // RAM[0] = 23456
    computer.dbg_set_ram(bit!(16, "0101101110100000"), bit!(14, "00000000000000"));
    // RAM[1] = 12345
    computer.dbg_set_ram(bit!(16, "0011000000111001"), bit!(14, "00000000000001"));
    computer.run(0, 10);
    // expect RAM[2] == 23456
    computer.dbg_print_ram_out(bit!(14, "00000000000010"));
}
