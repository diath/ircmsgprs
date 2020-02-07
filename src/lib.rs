pub mod parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_servername() {
        let mut parser = parser::Parser::new(":server FOO");
        let result = parser.parse().unwrap();
        assert_eq!(result.server.as_ref().unwrap(), "server");
    }

    #[test]
    fn test_parse_nick_host() {
        let mut parser = parser::Parser::new(":nick@host FOO");
        let result = parser.parse().unwrap();
        assert_eq!(result.nick.unwrap(), "nick".to_string());
        assert_eq!(result.host.unwrap(), "host".to_string());
    }

    #[test]
    fn test_parse_nick_user_host() {
        let mut parser = parser::Parser::new(":nick!user@host FOO");
        let result = parser.parse().unwrap();
        assert_eq!(result.nick.unwrap(), "nick".to_string());
        assert_eq!(result.user.unwrap(), "user".to_string());
        assert_eq!(result.host.unwrap(), "host".to_string());
    }

    #[test]
    fn test_parse_command() {
        let mut parser = parser::Parser::new(":nick@host FOO");
        let result = parser.parse().unwrap();
        assert_eq!(result.command, "FOO".to_string());
    }

    #[test]
    fn test_parse_command_numeric() {
        let mut parser = parser::Parser::new(":nick@host 333");
        let result = parser.parse().unwrap();
        assert_eq!(result.command, "333".to_string());
    }

    #[test]
    fn test_parse_command_short_numeric() {
        let mut parser = parser::Parser::new(":nick@host 33");
        let result = parser.parse();
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn test_parse_command_long_numeric() {
        let mut parser = parser::Parser::new(":nick@host 3333");
        let result = parser.parse();
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn test_parse_params() {
        let mut parser =
            parser::Parser::new(":nick@host FOO param1 param2 :trailing param with spaces");
        let result = parser.parse().unwrap();

        let mut iter = result.params.iter();
        assert_eq!(iter.next(), Some(&"param1".to_string()));
        assert_eq!(iter.next(), Some(&"param2".to_string()));
        assert_eq!(iter.next(), Some(&"trailing param with spaces".to_string()));
    }
}