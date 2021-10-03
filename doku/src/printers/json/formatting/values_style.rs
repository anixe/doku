use crate::*;

/// Determines how values should get displayed.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ValuesStyle {
    /// Displays values _outside_ of objects, as comments:
    ///
    /// ```
    /// use doku::Document;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize, Document)]
    /// struct Person {
    ///     /// First name
    ///     name: String,
    /// }
    ///
    /// let fmt = doku::json::Formatting {
    ///     values_style: doku::json::ValuesStyle::Comment(
    ///         "Default value: ".to_string(),
    ///     ),
    ///     ..Default::default()
    /// };
    ///
    /// let val = Person {
    ///     name: "Janet".to_string(),
    /// };
    ///
    /// let doc = doku::to_json_fmt_val(&fmt, &val);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     // First name
    ///     // Default value: "Janet"
    ///     "name": "string"
    ///   }
    /// "#, doc);
    /// ```
    Comment(String),

    /// Displays values _inside_ of objects, as fields:
    ///
    /// ```
    /// use doku::Document;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize, Document)]
    /// struct Person {
    ///     /// First name
    ///     name: String,
    /// }
    ///
    /// let fmt = doku::json::Formatting {
    ///     values_style: doku::json::ValuesStyle::Field,
    ///     ..Default::default()
    /// };
    ///
    /// let val = Person {
    ///     name: "Janet".to_string(),
    /// };
    ///
    /// let doc = doku::to_json_fmt_val(&fmt, &val);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     // First name
    ///     "name": "Janet"
    ///   }
    /// "#, doc);
    /// ```
    Field,
}

impl Default for ValuesStyle {
    fn default() -> Self {
        Self::Field
    }
}
