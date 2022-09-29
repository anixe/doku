use crate::*;

/// Determines the comments style.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommentsStyle {
    /// String to use as comment separator
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     /// First name
    ///     name: String,
    /// }
    ///
    /// let fmt = doku::toml::Formatting {
    ///     comments_style: doku::toml::CommentsStyle { separator: "//".to_owned() },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_toml_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   // First name
    ///   name = "string"
    /// "#, doc);
    /// ```
    pub separator: String,
}

impl Default for CommentsStyle {
    fn default() -> Self {
        Self {
            separator: "#".to_owned(),
        }
    }
}
