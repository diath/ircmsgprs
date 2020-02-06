use std::iter::Peekable;
use std::str::Chars;
use std::vec::Vec;

#[derive(Default)]
pub struct Message {
    pub prefix: String,
    pub command: String,
    pub params: Vec<String>,
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
        if *chr == ':' {
            message.prefix = self.parse_prefix();
        }

        if let Some(command) = self.parse_command() {
            message.command = command;
        } else {
            return None;
        }

        message.params = self.parse_params();
        Some(message)
    }

    fn parse_prefix(&mut self) -> String {
        // Skip the colon character
        self.data.next();

        return self
            .data
            .by_ref()
            .take_while(|c| *c != ' ')
            .collect::<String>();
    }

    fn parse_command(&mut self) -> Option<String> {
        let chr = self.data.peek()?;
        if chr.is_numeric() {
            let numeric = self
                .data
                .by_ref()
                .take_while(|c| c.is_numeric())
                .collect::<String>();

            // Numeric replies must be exactly 3 digits long
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

    fn parse_params(&mut self) -> Vec<String> {
        let mut params = Vec::new();
        loop {
            let chr = self.data.peek();
            if chr == None {
                break;
            }

            if *chr.unwrap() == ':' {
                params.push(self.data.by_ref().skip(1).collect::<String>());
            } else {
                params.push(
                    self.data
                        .by_ref()
                        .take_while(|c| *c != ' ')
                        .collect::<String>(),
                );
            }
        }

        return params;
    }
}
