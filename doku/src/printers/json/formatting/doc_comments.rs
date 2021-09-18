use crate::*;

/// Determines if doc-comments should get displayed.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum DocComments {
    /// Shows doc-comments:
    ///
    /// ```
    /// use doku::prelude::*;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     /// First name
    ///     /// (aka forename)
    ///     name: String,
    /// }
    ///
    /// let fmt = doku::json::Formatting {
    ///     doc_comments: doku::json::DocComments::Visible,
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_json_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     // First name
    ///     // (aka forename)
    ///     "name": "string"
    ///   }
    /// "#, doc);
    /// ```
    ///
    /// Please note that doc-comments are only the ones starting with _three_
    /// slashes.
    Visible,

    /// Hides doc-comments:
    ///
    /// ```
    /// use doku::prelude::*;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     /// First name
    ///     /// (aka forename)
    ///     name: String,
    /// }
    ///
    /// let fmt = doku::json::Formatting {
    ///     doc_comments: doku::json::DocComments::Hidden,
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_json_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     "name": "string"
    ///   }
    /// "#, doc);
    /// ```
    Hidden,
}

impl Default for DocComments {
    fn default() -> Self {
        Self::Visible
    }
}
