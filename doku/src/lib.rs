//! # Doku
//!
//! Doku is a framework for building aesthetic, human-readable documentation
//! straight from the code; it allows you to effortlessly generate docs for
//! configuration files, HTTP endpoints, and so on.
//!
//! Say goodbye to stale, hand-written documentation - with Doku, code _is_ the
//! documentation.
//!
//! ## Example
//!
//! Say, you're writing a CLI application that requires some JSON configuration
//! to work:
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
//! Now, how would you document this configuration for your users?
//!
//! You could show them to the Rust code, but that wouldn't be helpful for
//! people who don't understand Rust.
//!
//! You could write an example `config.json` by hand, but it might get stale
//! over time.
//!
//! Here comes Document - just add `#[derive(Document)]`:
//!
//! ```
//! # use serde::Deserialize;
//! use doku::prelude::*;
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
//! ... and voilà:
//!
//! ```
//! # use doku::prelude::*;
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
//! // println!("{}", doc); would say:
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
//! If you keep your documentation in the filesystem, you can even create a
//! test just to ensure the docs are in sync with code:
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
//! ## Supported formats
//!
//! At the moment there's out-of-box support for JSON; more formats, such as
//! TOML, are on their way!

mod macros;
mod objects;
mod printers;

pub mod prelude {
    pub use crate::objects::Document;
    pub use doku_derive::*;
}

pub use self::{objects::*, printers::*};

use serde::{Deserialize, Serialize};

/// Generates a JSON documentation for specified type.
///
/// # Example
///
/// ```
/// use doku::prelude::*;
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
pub fn to_json<T: Document>() -> String {
    json::Printer::default().print(&T::ty())
}

/// Generates a JSON documentation for specified type using custom formatting
/// settings.
///
/// # Example
///
/// ```
/// use doku::prelude::*;
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
pub fn to_json_fmt<T: Document>(fmt: &json::Formatting) -> String {
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
/// use doku::prelude::*;
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
pub fn to_json_val<T: Document + Serialize>(val: &T) -> String {
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
/// use doku::prelude::*;
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
pub fn to_json_fmt_val<T: Document + Serialize>(
    fmt: &json::Formatting,
    val: &T,
) -> String {
    json::Printer::default()
        .with_formatting(fmt)
        .with_value(&Value::from(val))
        .print(&T::ty())
}
