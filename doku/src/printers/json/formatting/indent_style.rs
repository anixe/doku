use crate::*;

/// Determines the indenting style.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndentStyle {
    /// How many spaces use to indicate that objects are nested:
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Response {
    ///     people: Vec<Person>,
    /// }
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     name: String,
    /// }
    ///
    /// let fmt = doku::json::Formatting {
    ///     indent_style: doku::json::IndentStyle {
    ///         size: 8,
    ///     },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_json_fmt::<Response>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///           "people": [
    ///                   {
    ///                           "name": "string"
    ///                   },
    ///                   /* ... */
    ///           ]
    ///   }
    /// "#, doc);
    /// ```
    pub size: usize,
}

impl Default for IndentStyle {
    fn default() -> Self {
        Self { size: 2 }
    }
}
