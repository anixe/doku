use crate::*;

/// Determines how enums should get displayed.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum EnumsStyle {
    /// Displays enum variants as comments:
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     favourite_color: Color,
    /// }
    ///
    /// #[derive(Document)]
    /// enum Color {
    ///     Red,
    ///     Green,
    ///     Blue,
    /// }
    ///
    /// let fmt = doku::json::Formatting {
    ///     enums_style: doku::json::EnumsStyle::Commented,
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_json_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     // Possible variants:
    ///     // - "Red"
    ///     // - "Green"
    ///     // - "Blue"
    ///     "favourite_color": "Red"
    ///   }
    /// "#, doc);
    /// ```
    Commented,

    /// Displays enum variants as separated with a pipe:
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     favourite_color: Color,
    /// }
    ///
    /// #[derive(Document)]
    /// enum Color {
    ///     Red,
    ///     Green,
    ///     Blue,
    /// }
    ///
    /// let fmt = doku::json::Formatting {
    ///     enums_style: doku::json::EnumsStyle::Separated,
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_json_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     "favourite_color": "Red" | "Green" | "Blue"
    ///   }
    /// "#, doc);
    /// ```
    Separated,
}

impl Default for EnumsStyle {
    fn default() -> Self {
        Self::Separated
    }
}
