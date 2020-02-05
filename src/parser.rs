use std::iter::Peekable;
use std::str::Chars;

#[derive(Default)]
pub struct Message {
    pub prefix: String,
    pub command: String,
    pub params: String,
}

pub struct Parser<'a> {
    data: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(line: &str) -> Parser {
        Parser {
            data: line.chars().peekable(),
        }
    }

    pub fn parse(&mut self) -> Option<Message> {
        let mut message = Message {
            ..Default::default()
        };

        let chr = self.data.peek()?;
        if *chr == ':' && message.prefix.len() == 0 {
            if let Some(prefix) = self.parse_prefix() {
                message.prefix = prefix;
            } else {
                return None;
            }
        }

        if message.command.len() == 0 {
            if let Some(command) = self.parse_command() {
                message.command = command;
            } else {
                return None;
            }
        }

        if let Some(params) = self.parse_params() {
            message.params = params;
        } else {
            return None;
        }

        Some(message)
    }

    fn parse_prefix(&mut self) -> Option<String> {
        None
    }

    fn parse_command(&mut self) -> Option<String> {
        let chr = self.data.peek()?;
        if chr.is_numeric() {
            let numeric = self
                .data
                .by_ref()
                .take_while(|c| c.is_numeric())
                .collect::<String>();
            if numeric.len() == 3 {
                return Some(numeric);
            }

            return None;
        } else {
            return Some(
                self.data
                    .by_ref()
                    .take_while(|c| *c != ' ')
                    .collect::<String>(),
            );
        }
    }

    fn parse_params(&mut self) -> Option<String> {
        None
    }
}
