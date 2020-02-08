# ircmsgprs

A simple Rust library that parses IRC protocol messages and turns them into the following struct:
```Rust
#[derive(Default, Debug)]
pub struct Message {
    pub server: Option<String>,
    pub nick: Option<String>,
    pub user: Option<String>,
    pub host: Option<String>,
    pub command: String,
    pub params: Vec<String>,
}
```

# Usage
```Rust
let mut parser = parser::Parser::new();
let result = parser
    .parse(":nick!user@host COMMAND param1 param2 :trailing param with spaces")
    .unwrap();
println!("{}", result);
```

# License

Licensed under the MIT license. See the [license file](LICENSE.md) for details.
