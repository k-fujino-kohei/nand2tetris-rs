use std::{fs::File, io::Read, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandType {
    ACommand(ACommand),
    CCommand(CCommand),
    LCommand(LCommand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACommand {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CCommand {
    pub dest: Option<String>,
    pub comp: String,
    pub jump: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LCommand {
    pub symbol: String,
}

pub struct Parser {
    commands: Vec<CommandType>,
    position: usize,
}

impl Parser {
    pub fn new<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        Self::with_bytes(&buf)
    }

    pub fn with_bytes(asm: &[u8]) -> anyhow::Result<Self> {
        let mut lexer = Lexer::new(asm);
        let tokens = lexer.lex();
        let commands = Self::parse(tokens);
        Ok(Self {
            commands,
            position: 0,
        })
    }

    pub fn has_more_commands(&self) -> bool {
        self.commands.get(self.position + 1).is_some()
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn command_type(&self) -> Option<CommandType> {
        self.commands.get(self.position).cloned()
    }

    pub fn symbol(&self) -> String {
        match self.command_type() {
            Some(CommandType::ACommand(ACommand { value })) => value,
            Some(CommandType::LCommand(LCommand { symbol })) => symbol,
            _ => panic!("Must be called when the command is a A/L command"),
        }
    }

    pub fn dest(&self) -> Option<String> {
        if let Some(CommandType::CCommand(command)) = self.command_type() {
            command.dest
        } else {
            panic!("Must be called when the command is a C command")
        }
    }

    pub fn comp(&self) -> String {
        if let Some(CommandType::CCommand(command)) = self.command_type() {
            command.comp
        } else {
            panic!("Must be called when the command is a C command")
        }
    }

    pub fn jump(&self) -> Option<String> {
        if let Some(CommandType::CCommand(command)) = self.command_type() {
            command.jump
        } else {
            panic!("Must be called when the command is a C command")
        }
    }

    fn parse(tokens: Vec<(Token, String)>) -> Vec<CommandType> {
        let mut commands = vec![];
        for line in tokens.split(|(token, _)| token == &Token::Lf) {
            let mut line = line.iter().peekable();
            let lead_token = if let Some(l) = line.next() {
                l
            } else {
                continue;
            };

            fn read_next<'a>(
                line: &mut impl Iterator<Item = &'a (Token, String)>,
            ) -> (Token, String) {
                line.next().expect("unexpected statement").clone()
            }

            // A Command = A1
            // A1 = @value
            if Token::At == lead_token.0 {
                let value = read_next(&mut line);
                commands.push(CommandType::ACommand(ACommand {
                    value: value.1.to_string(),
                }));
                continue;
            }

            if Token::Ident == lead_token.0 {
                // C Command = C1 | C2 | C3
                // C1 = dest=comp;jump
                // C2 = dest=comp
                // C3 = comp;jump
                let c_command = {
                    let dest = if let Some((Token::Eq, _)) = line.peek() {
                        let _eq = read_next(&mut line);
                        Some(lead_token.1.to_owned())
                    } else {
                        None
                    };
                    let comp = read_next(&mut line).1;
                    let jump = if let Some((Token::SemiColon, _)) = line.peek() {
                        let _semicolon = read_next(&mut line);
                        Some(read_next(&mut line).1)
                    } else {
                        None
                    };
                    CCommand { dest, comp, jump }
                };
                commands.push(CommandType::CCommand(c_command));
            }
        }
        commands
    }
}

impl Iterator for Parser {
    type Item = CommandType;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.command_type();
        self.advance();
        item
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    /// @
    At,
    /// Constant or Symbol.
    /// (\[a-z]|\[A-Z]|_|\.|$\:)(\[0-9]\[a-z]|\[A-Z]|_|\.|$\:)*
    Value,
    /// =
    Eq,
    /// ;
    SemiColon,
    /// /
    Slash,
    Ident,
    /// LF,
    Lf,
}

pub struct Lexer<'a> {
    asm: &'a [u8],
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(asm: &'a [u8]) -> Self {
        Self { asm, position: 0 }
    }

    pub fn lex(&mut self) -> Vec<(Token, String)> {
        let mut tokens = vec![];
        while let Some(char) = self.read_char() {
            match char {
                ' ' => {
                    // Ignore WhiteSpace
                }
                '\n' => {
                    tokens.push((Token::Lf, String::new()));
                }
                '/' if self.peek_char() == Some('/') => {
                    // Ignore Comment
                    self.read_to_lf();
                }
                '@' => {
                    tokens.push((Token::At, String::new()));
                    // A Instruction: @value
                    tokens.push((Token::Value, self.read_to_lf()));
                    tokens.push((Token::Lf, String::new()));
                }
                '=' => {
                    tokens.push((Token::Eq, String::new()));
                }
                ';' => {
                    tokens.push((Token::SemiColon, String::new()));
                }
                _ => {
                    let mut current = String::from(char);
                    'inner: while let Some(char) = self.read_char() {
                        if char.is_ascii_alphabetic()
                            || ['1', '-', '+', '&', '|', '!'].contains(&char)
                        {
                            current.push(char);
                        } else {
                            tokens.push((Token::Ident, current));
                            self.position -= 1;
                            break 'inner;
                        }
                    }
                }
            }
        }
        tokens
    }

    fn read_char(&mut self) -> Option<char> {
        self.next().map(char::from)
    }

    fn read_to_lf(&mut self) -> String {
        let mut line = String::new();
        while let Some(char) = self.read_char() {
            if char == '\n' {
                break;
            }
            line.push(char);
        }
        line
    }

    fn peek_char(&mut self) -> Option<char> {
        self.asm.get(self.position + 1).copied().map(char::from)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.asm.get(self.position).copied();
        self.position += 1;
        next
    }
}
