mod parser;

fn main() {
    let mut parser = parser::Parser::new(":diath!diath@irc.diath.net PRIVMSG rurka :hello\r\n");
    let _result = parser.parse().unwrap();
    println!("{}", _result);
}
