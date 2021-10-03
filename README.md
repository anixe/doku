# Doku &emsp; [![crates-badge]][crates-link] [![docs-badge]][docs-link]

[crates-badge]: https://img.shields.io/crates/v/doku.svg
[crates-link]: https://crates.io/crates/doku
[docs-badge]: https://img.shields.io/badge/docs.rs-latest-informational
[docs-link]: https://docs.rs/doku

Doku is a framework for building aesthetic, human-readable documentation
directly from the code; it allows you to effortlessly generate docs for
configuration files, HTTP endpoints, and so on.

Say goodbye to stale, hand-written documentation - with Doku, code _is_ the
documentation!

## Example

```toml
[dependencies]
doku = "0.10"
```

```rust
use doku::Document;
use serde::Deserialize;

#[derive(Deserialize, Document)]
struct Config {
    /// Database's engine
    db_engine: DbEngine,

    /// Database's host
    db_host: String,

    /// Database's port
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
    println!("{}", doku::to_json::<Config>());
}
```

``` 
{
  // Database's engine
  "db_engine": "pgsql" | "mysql",
  // Database's host
  "db_host": "string",
  // Database's port
  "db_port": 123
}
```

You'll find more examples in [./doku/examples](./doku/examples); there's also a
documentation at <https://docs.rs/doku/>.

## Contributing

Found a bug, have an idea? Please let us know on GitHub - patches are welcome,
too!

If you want to try hacking on Doku, the entry points are:

- [the JSON pretty-printer](./doku/src/printers/json.rs).
- [the data model](./doku/src/objects.rs),
- [the derive macro](./doku-derive/src/lib.rs),

As for the end-to-end tests, you'll find them inside [./doku/tests](./doku/tests).

The way those tests work is that each test-directory contains an input file
called `input.rs` and a set of output files called `output.<something>.json`.

The `input.rs` contains a Rust code defining a type called `Ty` - which can be
an enum, a struct, whatever is required for the test - and a top-level comment
that specifies [which testing-function](./doku/tests/printers.rs) you want to
execute on that type - e.g.:

``` 
// run: to_json()

#[derive(Document)]
pub struct Ty {
    foo: Vec<String>,
}
```

[Our procedural macro](./doku-test/src/lib.rs) then finds all of those
`input.rs`-s, reads their `// run:`, and generates tests resembling: 

``` 
#[test]
fn test() {
    mod source {
        include!("./path/to/that/input.rs");
    }
    
    to_json::<source::Ty>(/* ... */);
}
```

That `to_json()`, and rest of the testing-functions, generate a documentation
for `Ty` and compare the actual input (as generated from the code) to the
expected one (as present inside `output.<something>.json`) - when there's a
mismatch, the test fails:

```
thread '...' panicked at '
Found differences between `...` and `...`:
...
```

When a test fails, you'll see that next to the `output.<something>.json` file,
there appears a new one, called `output.<something>.json.new`; this `*.new` file
will contain the test's _actual_ result, and it's up to you to decide if this
actual result is the correct one.

If the `*.new` file looks fine, you can either:

- manually delete `output.<something>.json` and then rename
  `output.<something>.json.new` to `output.<something>.json`,
  
- or, if you have `make` and `fd` installed, you can execute `make bless` - this
  will automatically fix _all_ of the tests.

Happy hacking!

## License

Licensed under the MIT license.
