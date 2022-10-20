use crate::*;

/// Determines the indenting style.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndentStyle {
    /// How many spaces to use to indicate that objects are nested:
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     friends: [String; 3],
    /// }
    ///
    /// let fmt = doku::toml::Formatting {
    ///     indent_style: doku::toml::IndentStyle {
    ///         size: 8,
    ///         ..Default::default()
    ///     },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_toml_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   ## Must contain exactly 3 elements
    ///   friends = [
    ///           "string",
    ///           ## ...
    ///   ]
    /// "#, doc);
    /// ```
    pub size: usize,

    /// Whether table fields should be indented:
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
    ///     indent_style: doku::toml::IndentStyle {
    ///         indent_table_fields: true,
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
    ///   [address]
    ///     street = "string"
    ///     number = 123
    /// "#, doc);
    /// ```
    pub indent_table_fields: bool,
}

impl Default for IndentStyle {
    fn default() -> Self {
        Self {
            size: 2,
            indent_table_fields: false,
        }
    }
}
