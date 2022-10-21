# Doku &emsp; [![crates-badge]][crates-link] [![docs-badge]][docs-link]

[crates-badge]: https://img.shields.io/crates/v/doku.svg
[crates-link]: https://crates.io/crates/doku
[docs-badge]: https://img.shields.io/badge/docs.rs-latest-informational
[docs-link]: https://docs.rs/doku

Doku is a framework for documenting Rust data structures - it allows to generate
aesthetic, human-friendly descriptions of your configuration files, HTTP
endpoints and so on.

Say goodbye to stale, hand-written documentation - with Doku, code _is_ the
documentation!

## Example

```toml
[dependencies]
doku = "0.2.0-alpha.2"
```

```rust
use doku::Document;
use serde::Deserialize;

#[derive(Deserialize, Document)]
struct Config {
    /// Database's engine
    db_engine: DbEngine,

    /// Database's host
    #[doku(example = "localhost")]
    db_host: String,

    /// Database's port
    #[doku(example = "5432")]
    db_port: usize,
}

#[derive(Deserialize, Document)]
enum DbEngine {
    #[serde(rename = "pgsql")]
    PostgreSQL,

    #[serde(rename = "mysql")]
    MySQL,
}

fn main() {
    println!("```json");
    println!("{}", doku::to_json::<Config>());
    println!("```");
    println!();
    println!("```toml");
    println!("{}", doku::to_toml::<Config>());
    println!("```");
}
```

```
{
  // Database's engine
  "db_engine": "pgsql" | "mysql",
  // Database's host
  "db_host": "localhost",
  // Database's port
  "db_port": 5432
}
```

```
# Database's engine
db_engine = "pgsql" | "mysql"

# Database's host
db_host = "localhost"

# Database's port
db_port = 5432
```

You'll find more examples in [./doku/examples](./doku/examples); there's also a
documentation at <https://docs.rs/doku/>.

## Contributing

Found a bug, have an idea? Please let us know on GitHub - patches are welcome,
too!

If you want to try hacking on Doku, the entry points are:

- [the JSON pretty-printer](./doku/src/printers/json.rs),
- [the TOML pretty-printer](./doku/src/printers/toml.rs),
- [the data model](./doku/src/objects.rs),
- [the derive macro](./doku-derive/src/lib.rs).

There are also integration tests at [./doku/tests](./doku/tests).

## License

Licensed under the MIT license.
