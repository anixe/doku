# Doku &emsp; [![crates-badge]][crates-link]

[crates-badge]: https://img.shields.io/crates/v/doku.svg
[crates-link]: https://crates.io/crates/doku

Doku is a framework for building aesthetic, human-readable documentation
straight from the code; it allows you to effortlessly generate docs for
configuration files, HTTP endpoints, and so on.

Say goodbye to stale, hand-written documentation - with Doku, code _is_ the
documentation.

## Getting started

```toml
[dependencies]
doku = "0.10"
```

Doku has been made with [Serde](https://github.com/serde-rs/serde/) & [serde_json](https://github.com/serde-rs/json) in mind, so if you use them, starting with Doku is as easy as adding `#[derive(Document)]` to your types and invoking `doku::to_json::<YourType>()`.

(there's also `doku::JsonPrinter::new()`, if you want to have more control over the output's format.)

Somewhat [obviously](https://en.wikipedia.org/wiki/Halting_problem), Doku cannot introspect any hand-written (de)serializers, so if you use them, you'll have to inform Doku on how those manually-(de)serialized types look like (see [./doku/examples/custom-impl.rs](./doku/examples/custom-impl.rs)) - more often than not, the derive-macro should suffice, though.

## Using `#[doku]`

The `#[doku]` attribute allows to provide additional information about a type:

```
#[derive(Document)]
struct User {
    /// `example=` provides an example value for a field.
    ///
    /// Doku has a list of predefined examples for standard types (e.g. floats
    /// get documented as `123.45`), and you can use this attribute to provide
    /// a more meaningful example.
    ///
    /// Each field can have at most one example.
    #[doku(example = "alan.turing")]
    login: String,
    
    /// `as=` overrides which type this field gets documented as.
    ///
    /// This is useful when you e.g. depend on another crate that doesn't use
    /// Doku, and for which - due to Rust's orphan rules - you cannot impl
    /// `doku::Document` by hand.
    ///
    /// This is similar to `#[serde(remote = ...)]`.
    ///
    /// This doesn't affect Serde's (de)serialization mechanism.
    #[doku(as = "String")]
    favourite_terminal: iso_terminal_list::Terminal,

    /// `skip` allows to ignore given field in the documentation.
    ///
    /// Skipped fields don't have to have `impl doku::Document`, so this is
    /// kind of a get-out-of-jail-free card if you can live with this field 
    /// missing from the docs.
    ///
    /// This doesn't affect Serde's (de)serialization mechanism.
    #[doku(skip)]
    favourite_color: iso_color_list::Color,

    /// Last but not least, the `#[doku]` attribute allows to overwrite most of
    /// Serde's attributes for the purposes of documentation.
    ///
    /// E.g. this will cause for the type to be serialized with this field
    /// flattened, but the documentation will keep the field as-is, unflattened.
    #[serde(flatten)]
    #[doku(flatten = false)]
    overriden_flatten: Foo,
    
    /// cont'd - this field will not be (de)serialized by Serde, but will be
    /// present in the documentation.
    #[serde(skip)]
    #[doku(skip = false)]
    overriden_skip: bool,
}
```

## Can I use it without Serde?

Yes, totally! Doku only _understands_ Serde annotations - it doesn't _require_ any of them (see: the very first example in this readme).

## How does it work?

When you wrap a type with `#[derive(Document)]`:

```
#[derive(Document)]
struct User {
    /// Who? Who?
    #[doku(example = "alan.turing")]
    login: String,
}
```

... the derive-macro generates an `impl doku::Document for ...`:

```
impl doku::Document for User {
    fn ty() -> doku::Type {
        let login = doku::Field {
            ty: doku::Type {
                comment: Some("Who? Who?"),
                example: Some("alan.turing"),
                ..String::ty()
            },
            flattened: false,
        };

        doku::Type::from(doku::TypeKind::Struct {
            fields: doku::Fields::Named {
                fields: vec![
                    ("login", login)
                ],
            },
            transparent: false,
        })
    }
}
```

... and later, when you invoke `doku::to_json<User>()`, it just calls this `fn ty()` method:

```
fn to_json<Ty: Document>() -> String {
    let ty = Ty::ty();
    
    match &ty.kind {
        doku::TypeKind::String => print_string(/* ... */),
        doku::TypeKind::Struct { fields, .. } => print_struct(/* ... */),
        /* ... */
    }
}
```

There's no magic, no [RTTI](https://en.wikipedia.org/wiki/Run-time_type_information)-related hacks, no unsafety - just Rust being awesome.

Also, all those `doku::*` types are public - if you wanted, you could write _your own_ visitor that generates Doku-powered documentation for your own output format, not necessarily JSON!

## Quick help

### trait `Document` is not implemented for `...`

The offending type (enum / struct) is missing the `#[derive(Document)]`:

``` rust
#[derive(Document)]
struct Foo {
    bar: Bar, // trait `Document` is not implemented for `Bar`
}

struct Bar {
    /* ... */
}
```

## Compatibility (Rust)

MSRV: 1.51

## Compatibility (Serde)

Legend:
- ❌ = not supported (the derive-macro will return an error)
- ✅ = supported
- ✅ + no-op = supported, but doesn't affect the documentation

### `#[serde]` for [containers](https://serde.rs/container-attrs.html)

- ❌ `#[serde(rename = "...")]`
- ❌ `#[serde(rename(serialize = "..."))]`
- ❌ `#[serde(rename(deserialize = "..."))]`
- ❌ `#[serde(rename(serialize = "...", deserialize = "..."))]`
- ❌ `#[serde(rename_all = "...")] `
- ❌ `#[serde(rename_all(serialize = "..."))]`
- ❌ `#[serde(rename_all(deserialize = "..."))]`
- ❌ `#[serde(rename_all(serialize = "...", deserialize = "..."))]`
- ✅ `#[serde(deny_unknown_fields)]` (no-op)
- ✅ `#[serde(tag = "...")]`
- ✅ `#[serde(tag = "...", content = "...")]`
- ✅ `#[serde(untagged)]`
- ❌ `#[serde(bound = "...")]`
- ❌ `#[serde(bound(serialize = "..."))]`
- ❌ `#[serde(bound(deserialize = "..."))]`
- ❌ `#[serde(bound(serialize = "...", deserialize = "..."))]`
- ✅ `#[serde(default)]` (no-op)
- ✅ `#[serde(default = "...")]` (no-op)
- ❌ `#[serde(remote = "...")]`
- ✅ `#[serde(transparent)]`
- ❌ `#[serde(from = "...")]`
- ❌ `#[serde(try_from = "...")]`
- ❌ `#[serde(into = "...")]`
- ✅ `#[serde(crate = "...")]` (no-op)

### `#[serde]` for [variants](https://serde.rs/variant-attrs.html)

- ✅ `#[serde(rename = "...")]`
- ❌ `#[serde(rename(serialize = "..."))]`
- ❌ `#[serde(rename(deserialize = "..."))]`
- ❌ `#[serde(rename(serialize = "...", deserialize = "..."))]`
- ❌ `#[serde(alias = "...")]`
- ❌ `#[serde(rename_all = "...")]`
- ✅ `#[serde(skip)]`
- ✅ `#[serde(skip_serializing)]`
- ✅ `#[serde(skip_deserializing)]`
- ✅ `#[serde(serialize_with = "...")]` (no-op)
- ✅ `#[serde(deserialize_with = "...")]` (no-op)
- ✅ `#[serde(with = "...")]` (no-op)
- ❌ `#[serde(bound = "...")]`
- ❌ `#[serde(borrow)]`
- ❌ `#[serde(borrow = "...")]`
- ✅ `#[serde(other)]` (no-op)

### `#[serde]` for [fields](https://serde.rs/field-attrs.html)

- ✅ `#[serde(rename = "...")]`
- ❌ `#[serde(rename(serialize = "..."))]`
- ❌ `#[serde(rename(deserialize = "..."))]`
- ❌ `#[serde(rename(serialize = "...", deserialize = "..."))]`
- ❌ `#[serde(alias = "...")]`
- ✅ `#[serde(default)]` (no-op)
- ✅ `#[serde(default = "...'")]` (no-op)
- ✅ `#[serde(skip)]`
- ✅ `#[serde(skip_serializing)]`
- ✅ `#[serde(skip_deserializing)]`
- ✅ `#[serde(skip_serializing_if = "...")]` (no-op)
- ✅ `#[serde(serialize_with = "...")]` (no-op)
- ✅ `#[serde(deserialize_with = "...")]` (no-op)
- ✅ `#[serde(with = "...")]` (no-op)
- ❌ `#[serde(borrow)]` (no-op)
- ❌ `#[serde(borrow = "...")]` (no-op)
- ❌ `#[serde(getter = "...")]`

## Compatibility (miscellaneous)

- ❌ recursive types
- ❌ generic types (you can write `impl Document` by hand though)

## Contributing

Found a bug or something doesn't work as expected? Please let us know on GitHub; patches are welcome, too!

If you want to try hacking on Doku, the entry points are:
- [the derive-macro](./doku-derive/src/lib.rs),
- [the objects that describe types](./doku/src/objects.rs),
- [the JSON pretty-printer](./doku/src/printers/json.rs) and [its tests](./doku/tests/json.rs).

We've got integration tests inside `doku/tests`, and creating a new one is as easy as adding its path to the `json.rs` file, copy-pasting some another example there and adjusting the `code.rs` file. From within the `doku` directory, you can run tests using `cargo test`, and you can adjust the expected-files automatically using `make bless` (requires `make` and `fd`).

## License

Licensed under the MIT license.
