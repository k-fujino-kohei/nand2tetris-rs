use hardware::{load_hack, Computer};

pub fn main() {
    let data = load_hack!("Add.hack");
    let mut computer = Computer::new();
    computer.debug = true;
    computer.load_program(data);
    computer.run(0, 6);
    computer.dbg_print_ram_out([0; 14]); // => 5

    computer.dbg_set_ram([0; 16], [0; 14]);
    computer.run(1, 1);

    computer.run(0, 6);
    computer.dbg_print_ram_out([0; 14]); // => 5
}
