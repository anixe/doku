use crate::*;

/// Determines the objects style.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectsStyle {
    /// Whether to use quotes to surround keys
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     name: String,
    /// }
    ///
    /// let fmt_no_quotes = doku::json::Formatting {
    ///     objects_style: doku::json::ObjectsStyle { surround_keys_with_quotes: false, ..Default::default() },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_json_fmt::<Person>(&fmt_no_quotes);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     name: "string"
    ///   }
    /// "#, doc);
    ///
    /// let fmt_quotes = doku::json::Formatting {
    ///     objects_style: doku::json::ObjectsStyle { surround_keys_with_quotes: true, ..Default::default() },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_json_fmt::<Person>(&fmt_quotes);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     "name": "string"
    ///   }
    /// "#, doc);
    /// ```
    pub surround_keys_with_quotes: bool,

    /// Whether to use comma to separate properties
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     name: String,
    ///     address: String,
    /// }
    ///
    /// let fmt_no_quotes = doku::json::Formatting {
    ///     objects_style: doku::json::ObjectsStyle { use_comma_as_separator: false, ..Default::default() },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_json_fmt::<Person>(&fmt_no_quotes);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     "name": "string"
    ///     "address": "string"
    ///   }
    /// "#, doc);
    ///
    /// let fmt_quotes = doku::json::Formatting {
    ///     objects_style: doku::json::ObjectsStyle { use_comma_as_separator: true, ..Default::default() },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_json_fmt::<Person>(&fmt_quotes);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     "name": "string",
    ///     "address": "string"
    ///   }
    /// "#, doc);
    /// ```
    pub use_comma_as_separator: bool,
}

impl Default for ObjectsStyle {
    fn default() -> Self {
        Self {
            surround_keys_with_quotes: true,
            use_comma_as_separator: true,
        }
    }
}
