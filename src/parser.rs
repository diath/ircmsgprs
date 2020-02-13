use std::fmt;
use std::iter::Peekable;
use std::str::Chars;
use std::vec::Vec;

#[derive(Default, Debug)]
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

pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse<T: AsRef<str>>(&mut self, line: T) -> Option<Message> {
        let mut data = line.as_ref().trim_end_matches("\r\n").chars().peekable();
        let mut message = Message {
            ..Default::default()
        };

        let chr = data.peek()?;
        if *chr == ':' {
            let prefix = self.parse_prefix(&mut data);
            if prefix.find('@').is_some() {
                let mut chunks = prefix.split('@');
                let name = chunks.nth(0).unwrap().to_string();
                if name.find('!').is_some() {
                    let mut inner_chunks = name.split('!');
                    message.nick = Some(inner_chunks.nth(0).unwrap().to_string());
                    message.user = Some(inner_chunks.nth(0).unwrap().to_string());
                } else {
                    message.nick = Some(name);
                }

                message.host = Some(chunks.nth(0).unwrap().to_string());
            } else {
                message.server = Some(prefix);
            }
        }

        if let Some(command) = self.parse_command(&mut data) {
            message.command = command;
        } else {
            return None;
        }

        if let Some(params) = self.parse_params(&mut data) {
            message.params = params;
        } else {
            return None;
        }

        Some(message)
    }

    fn parse_prefix(&mut self, data: &mut Peekable<Chars>) -> String {
        return data
            .by_ref()
            .skip(1)
            .take_while(|c| *c != ' ')
            .collect::<String>();
    }

    fn parse_command(&mut self, data: &mut Peekable<Chars>) -> Option<String> {
        let chr = data.peek()?;
        if chr.is_numeric() {
            let numeric = data
                .by_ref()
                .take_while(|c| c.is_numeric())
                .collect::<String>();

            // Numeric replies must be exactly 3 digits long
            if numeric.len() == 3 {
                return Some(numeric);
            }

            return None;
        } else {
            return Some(data.by_ref().take_while(|c| *c != ' ').collect::<String>());
        }
    }

    fn parse_params(&mut self, data: &mut Peekable<Chars>) -> Option<Vec<String>> {
        let mut params = Vec::new();
        while let Some(chr) = data.peek() {
            if *chr == ':' {
                params.push(data.by_ref().skip(1).collect::<String>());
            } else {
                params.push(data.by_ref().take_while(|c| *c != ' ').collect::<String>());
            }

            if params.len() > 15 {
                return None;
            }
        }

        return Some(params);
    }
}
