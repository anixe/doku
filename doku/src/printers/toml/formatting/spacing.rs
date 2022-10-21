use crate::*;

/// Determines the spacing between fields.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spacing {
    /// How many lines to use to split scalar fields:
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     /// Person's name
    ///     name: String,
    ///     surname: String,
    ///
    ///     /// Person's address
    ///     address: String,
    /// }
    ///
    /// let fmt = doku::toml::Formatting {
    ///     spacing: doku::toml::Spacing {
    ///         lines_between_scalar_fields: 1,
    ///         ..Default::default()
    ///     },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_toml_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   ## Person's name
    ///   name = "string"
    ///
    ///   surname = "string"
    ///
    ///   ## Person's address
    ///   address = "string"
    /// "#, doc);
    /// ```
    pub lines_between_scalar_fields: usize,

    /// How many lines to use before a scalar field's comment:
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     /// Person's name
    ///     name: String,
    ///     surname: String,
    ///
    ///     /// Person's address
    ///     address: String,
    /// }
    ///
    /// let fmt = doku::toml::Formatting {
    ///     spacing: doku::toml::Spacing {
    ///         lines_between_scalar_field_comments: 1,
    ///         ..Default::default()
    ///     },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_toml_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   ## Person's name
    ///   name = "string"
    ///   surname = "string"
    ///
    ///   ## Person's address
    ///   address = "string"
    /// "#, doc);
    /// ```
    pub lines_between_scalar_field_comments: usize,

    /// How many lines to use to split tables:
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     name: String,
    ///     address: Address,
    /// }
    ///
    /// #[derive(Document)]
    /// struct Address {
    ///     street: String,
    ///     number: u32
    /// }
    ///
    /// let fmt = doku::toml::Formatting {
    ///     spacing: doku::toml::Spacing {
    ///         lines_between_tables: 2,
    ///         ..Default::default()
    ///     },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_toml_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   name = "string"
    ///
    ///
    ///   [address]
    ///   street = "string"
    ///   number = 123
    /// "#, doc);
    /// ```
    pub lines_between_tables: usize,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            lines_between_scalar_fields: 0,
            lines_between_scalar_field_comments: 1,
            lines_between_tables: 1,
        }
    }
}
