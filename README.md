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

# License

Licensed under the MIT license. See the [license file](LICENSE.md) for details.
