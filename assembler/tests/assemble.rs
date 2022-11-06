use assembler::assemble_raw;

#[test]
fn test_assemble() {
    let asm = include_str!("resources/MaxL.asm");
    let bin = assemble_raw(asm.as_bytes()).unwrap();
    let hack = include_str!("resources/Max.hack");
    for (line, (actual, expect)) in bin.split('\n').zip(hack.split('\n')).enumerate() {
        assert_eq!(actual, expect, "failed at line:{}", line + 1);
    }
}
