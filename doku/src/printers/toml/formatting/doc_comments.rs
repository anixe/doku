use crate::*;

/// Determines if doc-comments should get displayed.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum DocComments {
    /// Shows doc-comments:
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     /// First name
    ///     /// (aka forename)
    ///     name: String,
    /// }
    ///
    /// let fmt = doku::toml::Formatting {
    ///     doc_comments: doku::toml::DocComments::Visible,
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_toml_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   ## First name
    ///   ## (aka forename)
    ///   name = "string"
    /// "#, doc);
    /// ```
    ///
    /// Please note that doc-comments are only the ones starting with _three_
    /// slashes.
    Visible,

    /// Hides doc-comments:
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     /// First name
    ///     /// (aka forename)
    ///     name: String,
    /// }
    ///
    /// let fmt = doku::toml::Formatting {
    ///     doc_comments: doku::toml::DocComments::Hidden,
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_toml_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   name = "string"
    /// "#, doc);
    /// ```
    Hidden,
}

impl Default for DocComments {
    fn default() -> Self {
        Self::Visible
    }
}
