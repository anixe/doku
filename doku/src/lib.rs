//! `fn(Code) -> Docs`
//!
//! # Overview
//!
//! Doku is a framework for documenting Rust data structures - it allows to
//! generate aesthetic, human-friendly descriptions of configuration types,
//! requests / responses, and so on.
//!
//! Say goodbye to stale, hand-written documentation - with Doku, _code_ is the
//! documentation!
//!
//! # Example
//!
//! Say, you're writing an application that requires some TOML configuration to
//! work:
//!
//! ```
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     db_engine: DbEngine,
//!     db_host: String,
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
//! Usually you'll want to create a `config.example.toml`, describing the
//! configuration's format for your users to copy-paste and adjust:
//!
//! ```toml
//! db_engine = "pgsql" # or mysql
//! db_host = "localhost"
//! db_port = 5432
//! ```
//!
//! But writing such `config.example.toml` by hand is both tedious to maintain
//! and error-prone, since there's no guarantee that e.g. someone won't rename a
//! field, forgetting to update the documentation.
//!
//! Now, with Doku, all you need to do is add a few `#[derive(Document)]`:
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
//! ... and call `doku::to_json()` / `doku::to_toml()`, which will generate the
//! docs for you!
//!
//! ```
//! # use doku::Document;
//! # use serde::Deserialize;
//! #
//! # #[derive(Deserialize, Document)]
//! # struct Config {
//! #     db_engine: DbEngine,
//! #     db_host: String,
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
//! println!("{}", doku::to_toml::<Config>());
//!
//! /*
//! # doku::assert_doc!(r#"
//!   db_engine = "pgsql" | "mysql"
//!   db_host = "string"
//!   db_port = 123
//! # "#, doku::to_toml::<Config>());
//! */
//! ```
//!
//! This automatically-generated documentation can be then fine-tuned e.g. by
//! providing examples:
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
//! #
//! # #[derive(Deserialize, Document)]
//! # enum DbEngine {
//! #     #[serde(rename = "pgsql")]
//! #     PostgreSQL,
//! #
//! #     #[serde(rename = "mysql")]
//! #     MySQL,
//! # }
//!
//! println!("{}", doku::to_toml::<Config>());
//!
//! /*
//! # doku::assert_doc!(r#"
//!   ## Database's engine
//!   db_engine = "pgsql" | "mysql"
//!
//!   ## Database's host
//!   db_host = "localhost"
//!
//!   ## Database's port
//!   db_port = 5432
//! # "#, doku::to_toml::<Config>());
//! */
//! ```
//!
//! And voilà, ready to deploy!
//!
//! What's more -- because `doku::to_json()` returns a good-old `String`, it's
//! possible to create a test to make sure your docs always stay up-to-date:
//!
//! ```no_run
//! use std::fs;
//!
//! #[test]
//! fn docs() {
//!     let expected_docs = doku::to_toml::<Config>();
//!     let actual_docs = fs::read_to_string("config.example.toml").unwrap();
//!
//!     if actual_docs != expected_docs {
//!         fs::write("config.example.toml.new", actual_docs);
//!         panic!("`config.example.toml` is stale");
//!     }
//! }
//! ```
//!
//! Let go & let the pipelines worry about your docs!
//!
//! # Plug and Play
//!
//! Doku has been made with plug-and-play approach in mind - it understands most
//! of the Serde's annotations and comes with a predefined, curated formatting
//! settings, so that just adding `#[derive(Document)]` should get you started
//! quickly & painlessly.
//!
//! At the same time, Doku is extensible - if the formatting settings don't
//! match your taste, you can tune them; if the derive macro doesn't work
//! because you've got a custom `impl Serialize`, you can write `impl Document`
//! by hand as well.
//!
//! So - **come join the doc side**!
//!
//! # Limitations
//!
//! ## Formats
//!
//! At the moment Doku provides functions for rendering JSON-like and TOML-like
//! documents.
//!
//! All models used by Doku are public though, so if you wanted, you could very
//! easily roll your own pretty-printer, for you own custom format:
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
//! ## Annotations
//!
//! Doku understands most of Serde's annotations, so e.g. the following will
//! work as expected:
//!
//! ```
//! # use doku::Document;
//! # use serde::Serialize;
//! #
//! #[derive(Serialize, Document)]
//! struct Something {
//!     #[serde(rename = "foo")]
//!     bar: String,
//! }
//! ```
//!
//! If you're not using Serde, but you'd like to pass Serde-like attributes for
//! Doku to understand, there's also:
//!
//! ```
//! # use doku::Document;
//! #
//! #[derive(Document)]
//! struct Something {
//!     #[doku(rename = "foo")] // (note the attribute name here)
//!     bar: String,
//! }
//! ```
//!
//! ## Language features
//!
//! Doku supports most of Rust language's & standard library's features (such as
//! strings, vectors, maps or generic types); the only exceptions are recursive
//! types (which will cause the pretty-printers to panic, since they don't
//! support those).
//!
//! Some external crates (such as `chrono` or `url`) are supported behind
//! feature-flags.
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
//! ... the macro will generate an `impl doku::Document`:
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
//!             aliases: &[],
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
//! ... that will be invoked later, when you call `doku::to_*()`:
//!
//! ```
//! fn to_json<T>() -> String
//! where
//!     T: doku::Document
//! {
//!     match T::ty().kind {
//!         doku::TypeKind::String => print_string(/* ... */),
//!         doku::TypeKind::Struct { .. } => print_struct(/* ... */),
//!         /* ... */
//!         # _ => todo!(),
//!     }
//! }
//! #
//! # fn print_string() -> String { todo!() }
//! # fn print_struct() -> String { todo!() }
//! ```
//!
//! There's no magic, no [RTTI](https://en.wikipedia.org/wiki/Run-time_type_information)
//! hacks, no unsafety - it's all just-Rust.

#[doc(hidden)]
#[macro_use]
mod macros;

mod objects;
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

/// Generates a TOML documentation for specified type.
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
/// let doc = doku::to_toml::<Config>();
///
/// doku::assert_doc!(r#"
///   ## Database's host
///   db_host = "string"
/// "#, doc);
/// ```
///
/// For more control over the output format, please see: [`to_toml_fmt()`].
pub fn to_toml<T>() -> String
where
    T: Document,
{
    toml::Printer::default().print(&T::ty())
}

/// Generates a TOML documentation for specified type using custom formatting
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
/// let fmt = doku::toml::Formatting {
///     layout: doku::toml::Layout::TwoColumns {
///         align: true,
///         spacing: 1,
///     },
///     ..Default::default()
/// };
///
/// let doc = doku::to_toml_fmt::<Config>(&fmt);
///
/// doku::assert_doc!(r#"
///   db_host = "string" # Database's host
/// "#, doc);
/// ```
///
/// For more details, please see: [`toml::Formatting`].
pub fn to_toml_fmt<T>(fmt: &toml::Formatting) -> String
where
    T: Document,
{
    toml::Printer::default()
        .with_formatting(fmt)
        .print(&T::ty())
}

/// Generates a TOML documentation for specified type, extracting example values
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
/// let doc = doku::to_toml_val(&Config::default());
///
/// doku::assert_doc!(r#"
///   ## Database's host
///   db_host = "localhost"
/// "#, doc);
/// ```
///
/// For more control over the output format, please see: [`to_toml_fmt_val()`].
pub fn to_toml_val<T>(val: &T) -> String
where
    T: Document + Serialize,
{
    toml::Printer::default()
        .with_value(&Value::from(val))
        .print(&T::ty())
}

/// Generates a TOML documentation for specified type using custom formatting
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
/// let fmt = doku::toml::Formatting {
///     layout: doku::toml::Layout::TwoColumns {
///         align: true,
///         spacing: 1,
///     },
///     ..Default::default()
/// };
///
/// let doc = doku::to_toml_fmt_val(&fmt, &Config::default());
///
/// doku::assert_doc!(r#"
///   db_host = "localhost" # Database's host
/// "#, doc);
/// ```
pub fn to_toml_fmt_val<T>(fmt: &toml::Formatting, val: &T) -> String
where
    T: Document + Serialize,
{
    toml::Printer::default()
        .with_formatting(fmt)
        .with_value(&Value::from(val))
        .print(&T::ty())
}
