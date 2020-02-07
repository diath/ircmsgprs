use std::fmt;
use std::iter::Peekable;
use std::str::Chars;
use std::vec::Vec;

#[derive(Default)]
pub struct Message {
    pub server: Option<String>,
    pub nick: Option<String>,
    pub user: Option<String>,
    pub host: Option<String>,
    pub command: String,
    pub params: Vec<String>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.server.is_some() {
            write!(f, "Server: {} ", self.server.as_ref().unwrap())?;
        } else if self.nick.is_some() {
            if self.user.is_some() {
                write!(
                    f,
                    "User: {}!{}",
                    self.nick.as_ref().unwrap(),
                    self.user.as_ref().unwrap()
                )?;
            } else {
                write!(f, "User: {}", self.nick.as_ref().unwrap())?;
            }

            if self.host.is_some() {
                write!(f, "@{}, ", self.host.as_ref().unwrap())?;
            }
        }

        write!(f, "Command: {}, Params: {:?}", self.command, self.params)
    }
}

pub struct Parser<'a> {
    data: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(line: &str) -> Parser {
        Parser {
            data: line.trim_end_matches("\r\n").chars().peekable(),
        }
    }

    pub fn parse(&mut self) -> Option<Message> {
        let mut message = Message {
            ..Default::default()
        };

        let chr = self.data.peek()?;
        if *chr == ':' {
            let prefix = self.parse_prefix();
            if prefix.find('@').is_some() {
                let mut chunks = prefix.split('@');
                let name = chunks.nth(0).unwrap().to_string();
                if name.find('!').is_some() {
                    let mut inner_chunks = name.split('!');
                    message.nick = Some(inner_chunks.nth(0).unwrap().to_string());
                    message.host = Some(inner_chunks.nth(0).unwrap().to_string());
                } else {
                    message.nick = Some(name);
                }

                message.host = Some(chunks.nth(0).unwrap().to_string());
            } else {
                message.server = Some(prefix);
            }
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
