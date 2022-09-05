//! `fn(Code) -> Docs`
//!
//! # Overview
//!
//! Doku is a framework for building textual, aesthetic documentation directly
//! from the code; it allows to generate docs for configuration files, HTTP
//! endpoints, and so on.
//!
//! Say goodbye to stale, hand-written documentation - with Doku, code _is_ the
//! documentation!
//!
//! # Example
//!
//! Say, you're writing a tool that requires some JSON configuration to work:
//!
//! ```
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     /// Database's engine
//!     db_engine: DbEngine,
//!
//!     /// Database's host
//!     db_host: String,
//!
//!     /// Database's port
//!     db_port: usize,
//! }
//!
//! #[derive(Deserialize)]
//! enum DbEngine {
//!     #[serde(rename = "pgsql")]
//!     PostgreSQL,
//!
//!     #[serde(rename = "mysql")]
//!     MySQL,
//! }
//! ```
//!
//! Now, with Doku, generating a documentation for your users is as simple as
//! adding `#[derive(Document)]`:
//!
//! ```
//! # use serde::Deserialize;
//! use doku::Document;
//!
//! #[derive(Deserialize, Document)]
//! struct Config {
//!     /* ... */
//! }
//!
//! #[derive(Deserialize, Document)]
//! enum DbEngine {
//!     /* ... */
//! }
//! ```
//!
//! ... and calling `doku::to_json()`:
//!
//! ```
//! # use doku::Document;
//! # use serde::Deserialize;
//! #
//! # #[derive(Deserialize, Document)]
//! # struct Config {
//! #     /// Database's engine
//! #     db_engine: DbEngine,
//! #
//! #     /// Database's host
//! #     db_host: String,
//! #
//! #     /// Database's port
//! #     db_port: usize,
//! # }
//! #
//! # #[derive(Deserialize, Document)]
//! # enum DbEngine {
//! #     #[serde(rename = "pgsql")]
//! #     PostgreSQL,
//! #
//! #     #[serde(rename = "mysql")]
//! #     MySQL,
//! # }
//! #
//! let doc = doku::to_json::<Config>();
//!
//! println!("{}", doc); // says:
//!
//! # doku::assert_doc!(r#"
//!   {
//!     // Database's engine
//!     "db_engine": "pgsql" | "mysql",
//!     // Database's host
//!     "db_host": "string",
//!     // Database's port
//!     "db_port": 123
//!   }
//! # "#, doc);
//! ```
//!
//! The documentation can be then further fine-tuned e.g. by providing examples:
//!
//! ```
//! # use doku::Document;
//! # use serde::Deserialize;
//! #
//! #[derive(Deserialize, Document)]
//! struct Config {
//!     /// Database's engine
//!     db_engine: DbEngine,
//!
//!     /// Database's host
//!     #[doku(example = "localhost")]
//!     db_host: String,
//!
//!     /// Database's port
//!     #[doku(example = "5432")]
//!     db_port: usize,
//! }
//!
//! #[derive(Deserialize, Document)]
//! enum DbEngine {
//!     #[serde(rename = "pgsql")]
//!     PostgreSQL,
//!
//!     #[serde(rename = "mysql")]
//!     MySQL,
//! }
//!
//! let doc = doku::to_json::<Config>();
//!
//! println!("{}", doc); // says:
//!
//! # doku::assert_doc!(r#"
//!   {
//!     // Database's engine
//!     "db_engine": "pgsql" | "mysql",
//!     // Database's host
//!     "db_host": "localhost",
//!     // Database's port
//!     "db_port": 5432
//!   }
//! # "#, doc);
//! ```
//!
//! And voilà, ready to deploy!
//!
//! Also, because `doku::to_json()` returns a good-old `String`, it's easy to
//! e.g. create a test ensuring that docs stay in sync with the code:
//!
//! ```no_run
//! use std::fs;
//!
//! #[test]
//! fn docs() {
//!     let actual_docs = doku::to_json::<Config>();
//!     let current_docs = fs::read_to_string("config.example.json").unwrap();
//!
//!     if current_docs != actual_docs {
//!         fs::write("config.example.json.new", actual_docs);
//!         panic!("`config.example.json` is stale; please see: `config.example.json.new`");
//!     }
//! }
//! ```
//!
//! Let go & let the pipelines worry about your docs!
//!
//! # Plug and Play
//!
//! Doku has been made with the plug-and-play approach in mind - it understands
//! the most common Serde annotations and comes with a predefined formatting
//! settings, so adding `#[derive(Document)]` here and there should get you
//! started quickly & painlessly.
//!
//! At the same time, Doku is extensible - if the formatting settings don't
//! match your taste, there is a way to tune them; if the derive macro doesn't
//! work because you use custom `impl Serialize`, you can write `impl Document`
//! by hand, too.
//!
//! So - come join the doc side!
//!
//! # Limits
//!
//! ## Supported formats
//!
//! Currently Doku provides functions for generating JSON docs; more formats,
//! such as TOML, are on their way.
//!
//! If you wanted, you could even implement a pretty-printer for your own
//! format - all of the required types are public, so getting started is as
//! easy as:
//!
//! ```
//! fn to_my_own_format<T>() -> String
//! where
//!     T: doku::Document
//! {
//!    match T::ty().kind {
//!        doku::TypeKind::String => "got a string!".to_string(),
//!        doku::TypeKind::Struct { .. } => "got a struct!".to_string(),
//!        _ => todo!(),
//!    }
//! }
//!
//! println!("{}", to_my_own_format::<String>());
//! ```
//!
//! ## Supported Serde annotations
//!
//! Legend:
//!
//! - ❌ = not supported (the derive macro will return an error)
//! - ✅ = supported
//! - ✅ + no-op = supported, but doesn't affect the documentation
//!
//! `#[serde]` for [containers](https://serde.rs/container-attrs.html):
//!
//! - ❌ `#[serde(rename = "...")]`
//! - ❌ `#[serde(rename(serialize = "..."))]`
//! - ❌ `#[serde(rename(deserialize = "..."))]`
//! - ❌ `#[serde(rename(serialize = "...", deserialize = "..."))]`
//! - ❌ `#[serde(rename_all = "...")] `
//! - ❌ `#[serde(rename_all(serialize = "..."))]`
//! - ❌ `#[serde(rename_all(deserialize = "..."))]`
//! - ❌ `#[serde(rename_all(serialize = "...", deserialize = "..."))]`
//! - ✅ `#[serde(deny_unknown_fields)]` (no-op)
//! - ✅ `#[serde(tag = "...")]`
//! - ✅ `#[serde(tag = "...", content = "...")]`
//! - ✅ `#[serde(untagged)]`
//! - ❌ `#[serde(bound = "...")]`
//! - ❌ `#[serde(bound(serialize = "..."))]`
//! - ❌ `#[serde(bound(deserialize = "..."))]`
//! - ❌ `#[serde(bound(serialize = "...", deserialize = "..."))]`
//! - ✅ `#[serde(default)]` (no-op)
//! - ✅ `#[serde(default = "...")]` (no-op)
//! - ❌ `#[serde(remote = "...")]`
//! - ✅ `#[serde(transparent)]`
//! - ❌ `#[serde(from = "...")]`
//! - ❌ `#[serde(try_from = "...")]`
//! - ❌ `#[serde(into = "...")]`
//! - ✅ `#[serde(crate = "...")]` (no-op)
//!
//! `#[serde]` for [variants](https://serde.rs/variant-attrs.html):
//!
//! - ✅ `#[serde(rename = "...")]`
//! - ❌ `#[serde(rename(serialize = "..."))]`
//! - ❌ `#[serde(rename(deserialize = "..."))]`
//! - ❌ `#[serde(rename(serialize = "...", deserialize = "..."))]`
//! - ✅ `#[serde(alias = "...")]` (no-op)
//! - ❌ `#[serde(rename_all = "...")]`
//! - ✅ `#[serde(skip)]`
//! - ✅ `#[serde(skip_serializing)]`
//! - ✅ `#[serde(skip_deserializing)]`
//! - ✅ `#[serde(serialize_with = "...")]` (no-op)
//! - ✅ `#[serde(deserialize_with = "...")]` (no-op)
//! - ✅ `#[serde(with = "...")]` (no-op)
//! - ❌ `#[serde(bound = "...")]`
//! - ❌ `#[serde(borrow)]`
//! - ❌ `#[serde(borrow = "...")]`
//! - ✅ `#[serde(other)]` (no-op)
//!
//! `#[serde]` for [fields](https://serde.rs/field-attrs.html):
//!
//! - ✅ `#[serde(rename = "...")]`
//! - ❌ `#[serde(rename(serialize = "..."))]`
//! - ❌ `#[serde(rename(deserialize = "..."))]`
//! - ❌ `#[serde(rename(serialize = "...", deserialize = "..."))]`
//! - ✅ `#[serde(alias = "...")]` (no-op)
//! - ✅ `#[serde(default)]` (no-op)
//! - ✅ `#[serde(default = "...'")]` (no-op)
//! - ✅ `#[serde(skip)]`
//! - ✅ `#[serde(skip_serializing)]`
//! - ✅ `#[serde(skip_deserializing)]`
//! - ✅ `#[serde(skip_serializing_if = "...")]` (no-op)
//! - ✅ `#[serde(serialize_with = "...")]` (no-op)
//! - ✅ `#[serde(deserialize_with = "...")]` (no-op)
//! - ✅ `#[serde(with = "...")]` (no-op)
//! - ❌ `#[serde(borrow)]` (no-op)
//! - ❌ `#[serde(borrow = "...")]` (no-op)
//! - ❌ `#[serde(getter = "...")]`
//!
//! ## Supported language features
//!
//! - ❌ generic types (<https://github.com/anixe/doku/issues/3>)
//! - ❌ recursive types (<https://github.com/anixe/doku/issues/10>)
//!
//! # How does it work?
//!
//! When you wrap a type with `#[derive(Document)]`:
//!
//! ```
//! # use doku::Document;
//! #
//! #[derive(Document)]
//! struct User {
//!     /// Who? Who?
//!     #[doku(example = "alan.turing")]
//!     login: String,
//! }
//! ```
//!
//! ... this derive macro generates an `impl doku::Document`:
//!
//! ```
//! # struct User;
//! #
//! impl doku::Document for User {
//!     fn ty() -> doku::Type {
//!         let login = doku::Field {
//!             ty: doku::Type {
//!                 comment: Some("Who? Who?"),
//!                 example: Some(doku::Example::Simple("alan.turing")),
//!                 ..String::ty()
//!             },
//!             flattened: false,
//!         };
//!
//!         doku::Type::from(doku::TypeKind::Struct {
//!             fields: doku::Fields::Named {
//!                 fields: vec![
//!                     ("login", login)
//!                 ],
//!             },
//!             transparent: false,
//!         })
//!     }
//! }
//! ```
//!
//! ... and later, when you invoke `doku::to_json<...>()`, it just calls this
//!`fn ty()` method:
//!
//! ```rust,no_build,no_run
//! fn to_json<T>() -> String
//! where
//!     T: doku::Document
//! {
//!     match T::ty().kind {
//!         doku::TypeKind::String => print_string(/* ... */),
//!         doku::TypeKind::Struct { .. } => print_struct(/* ... */),
//!         /* ... */
//!     }
//! }
//! ```
//!
//! There's no magic, no [RTTI](https://en.wikipedia.org/wiki/Run-time_type_information)
//! hacks, no unsafety - it's all just Rust.

/// Macros facilitating working on Doku
mod macros;

/// Doku's data model
mod objects;

/// Doku's pretty-printers
mod printers;

pub use self::{objects::*, printers::*};
pub use doku_derive::*;

use serde::{Deserialize, Serialize};

/// Generates a JSON documentation for specified type.
///
/// # Example
///
/// ```
/// use doku::Document;
///
/// #[derive(Document)]
/// struct Config {
///     /// Database's host
///     db_host: String,
/// }
///
/// let doc = doku::to_json::<Config>();
///
/// doku::assert_doc!(r#"
///   {
///     // Database's host
///     "db_host": "string"
///   }
/// "#, doc);
/// ```
///
/// For more control over the output format, please see: [`to_json_fmt()`].
pub fn to_json<T>() -> String
where
    T: Document,
{
    json::Printer::default().print(&T::ty())
}

/// Generates a JSON documentation for specified type using custom formatting
/// settings.
///
/// # Example
///
/// ```
/// use doku::Document;
///
/// #[derive(Document)]
/// struct Config {
///     /// Database's host
///     db_host: String,
/// }
///
/// let fmt = doku::json::Formatting {
///     layout: doku::json::Layout::TwoColumns {
///         align: true,
///         spacing: 1,
///     },
///     ..Default::default()
/// };
///
/// let doc = doku::to_json_fmt::<Config>(&fmt);
///
/// doku::assert_doc!(r#"
///   {
///     "db_host": "string" // Database's host
///   }
/// "#, doc);
/// ```
///
/// For more details, please see: [`json::Formatting`].
pub fn to_json_fmt<T>(fmt: &json::Formatting) -> String
where
    T: Document,
{
    json::Printer::default()
        .with_formatting(fmt)
        .print(&T::ty())
}

/// Generates a JSON documentation for specified type, extracting example values
/// from given serializable object.
///
/// This is useful e.g. if you've got a configuration with predefined values
/// that you'd like to show your users.
///
/// # Example
///
/// ```
/// use doku::Document;
/// use serde::Serialize;
///
/// #[derive(Serialize, Document)]
/// struct Config {
///     /// Database's host
///     db_host: String,
/// }
///
/// impl Default for Config {
///     fn default() -> Self {
///         Self {
///             db_host: "localhost".to_string(),
///         }
///     }
/// }
///
/// let doc = doku::to_json_val(&Config::default());
///
/// doku::assert_doc!(r#"
///   {
///     // Database's host
///     "db_host": "localhost"
///   }
/// "#, doc);
/// ```
///
/// For more control over the output format, please see: [`to_json_fmt_val()`].
pub fn to_json_val<T>(val: &T) -> String
where
    T: Document + Serialize,
{
    json::Printer::default()
        .with_value(&Value::from(val))
        .print(&T::ty())
}

/// Generates a JSON documentation for specified type using custom formatting
/// settings, and extracting example values from given serializable object.
///
/// # Example
///
/// ```
/// use doku::Document;
/// use serde::Serialize;
///
/// #[derive(Serialize, Document)]
/// struct Config {
///     /// Database's host
///     db_host: String,
/// }
///
/// impl Default for Config {
///     fn default() -> Self {
///         Self {
///             db_host: "localhost".to_string(),
///         }
///     }
/// }
///
/// let fmt = doku::json::Formatting {
///     layout: doku::json::Layout::TwoColumns {
///         align: true,
///         spacing: 1,
///     },
///     ..Default::default()
/// };
///
/// let doc = doku::to_json_fmt_val(&fmt, &Config::default());
///
/// doku::assert_doc!(r#"
///   {
///     "db_host": "localhost" // Database's host
///   }
/// "#, doc);
/// ```
pub fn to_json_fmt_val<T>(fmt: &json::Formatting, val: &T) -> String
where
    T: Document + Serialize,
{
    json::Printer::default()
        .with_formatting(fmt)
        .with_value(&Value::from(val))
        .print(&T::ty())
}
