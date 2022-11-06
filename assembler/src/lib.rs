use code::{comp_code, dest_code, jump_code};
use parser::{ACommand, CCommand, CommandType, Parser};
use std::{fs::File, io::Write, path::Path};

pub mod code;
pub mod parser;

pub fn assemble<P>(path: P, output: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let mut parser = Parser::new(path)?;
    let bin = core_assembler(&mut parser);
    let mut out = File::create(output)?;
    out.write_all(bin.as_bytes())?;
    Ok(())
}

pub fn assemble_raw(asm: &[u8]) -> anyhow::Result<String> {
    let mut parser = Parser::with_bytes(asm)?;
    let bin = core_assembler(&mut parser);
    Ok(bin)
}

fn core_assembler(parser: &mut Parser) -> String {
    let mut bin: Vec<String> = vec![];
    for command in parser {
        match command {
            CommandType::ACommand(ACommand { value }) => {
                if let Ok(value) = value.parse::<i16>() {
                    let value = format!("{:<016b}", value);
                    bin.push(value);
                }
            }
            CommandType::CCommand(c_command) => {
                let a = c_command.a();
                let CCommand { dest, comp, jump } = c_command;
                let dest = dest_code(&dest.unwrap_or_default());
                let comp = comp_code(&comp, a);
                let jump = jump_code(&jump.unwrap_or_default());
                let mut code = String::with_capacity(16);
                code.push_str("111");
                code.push_str(if a { "1" } else { "0" });
                code.push_str(&comp);
                code.push_str(&dest);
                code.push_str(&jump);
                bin.push(code);
            }
            CommandType::LCommand(_) => {}
        }
    }
    bin.join("\n")
}
