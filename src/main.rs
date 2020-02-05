mod parser;

fn main() {
    let mut parser = parser::Parser::new("PING :1337\r\n");
    let _result = parser.parse();
}
