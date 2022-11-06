use assembler::parser::{ACommand, CCommand, CommandType, Parser};

#[test]
fn test_parse() {
    let asm = include_str!("resources/MaxL.asm");
    let parser = Parser::with_bytes(asm.as_bytes()).unwrap();
    let expect_commands = [
        CommandType::ACommand(ACommand {
            value: String::from("0"),
        }),
        CommandType::CCommand(CCommand {
            dest: Some(String::from("D")),
            comp: String::from("M"),
            jump: None,
        }),
        CommandType::ACommand(ACommand {
            value: String::from("1"),
        }),
        CommandType::CCommand(CCommand {
            dest: Some(String::from("D")),
            comp: String::from("D-M"),
            jump: None,
        }),
        CommandType::ACommand(ACommand {
            value: String::from("10"),
        }),
        CommandType::CCommand(CCommand {
            dest: None,
            comp: String::from("D"),
            jump: Some(String::from("JGT")),
        }),
        CommandType::ACommand(ACommand {
            value: String::from("1"),
        }),
        CommandType::CCommand(CCommand {
            dest: Some(String::from("D")),
            comp: String::from("M"),
            jump: None,
        }),
        CommandType::ACommand(ACommand {
            value: String::from("12"),
        }),
        CommandType::CCommand(CCommand {
            dest: None,
            comp: String::from("0"),
            jump: Some(String::from("JMP")),
        }),
        CommandType::ACommand(ACommand {
            value: String::from("0"),
        }),
        CommandType::CCommand(CCommand {
            dest: Some(String::from("D")),
            comp: String::from("M"),
            jump: None,
        }),
        CommandType::ACommand(ACommand {
            value: String::from("2"),
        }),
        CommandType::CCommand(CCommand {
            dest: Some(String::from("M")),
            comp: String::from("D"),
            jump: None,
        }),
        CommandType::ACommand(ACommand {
            value: String::from("14"),
        }),
        CommandType::CCommand(CCommand {
            dest: None,
            comp: String::from("0"),
            jump: Some(String::from("JMP")),
        }),
    ];
    for (i, command) in parser.enumerate() {
        assert_eq!(&expect_commands[i], &command);
    }
}
