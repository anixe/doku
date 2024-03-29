use crate::*;

/// Determines whether the document should contain one or two columns.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Layout {
    /// Prints types and comments inline, in a single chunk of text:
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     /// First name
    ///     /// (aka forename)
    ///     first_name: String,
    ///
    ///     /// Last name
    ///     /// (aka surname)
    ///     last_name: String,
    /// }
    ///
    /// let fmt = doku::toml::Formatting {
    ///     layout: doku::toml::Layout::OneColumn,
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_toml_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   ## First name
    ///   ## (aka forename)
    ///   first_name = "string"
    ///
    ///   ## Last name
    ///   ## (aka surname)
    ///   last_name = "string"
    /// "#, doc);
    /// ```
    OneColumn,

    /// Prints types and comments in two separate columns.
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     /// First name
    ///     /// (aka forename)
    ///     first_name: String,
    ///
    ///     /// Last name
    ///     /// (aka surname)
    ///     last_name: String,
    /// }
    ///
    /// let fmt = doku::toml::Formatting {
    ///     layout: doku::toml::Layout::TwoColumns {
    ///         align: true,
    ///         spacing: 1,
    ///     },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_toml_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   first_name = "string" # First name
    ///                         ## (aka forename)
    ///
    ///   last_name = "string"  # Last name
    ///                         ## (aka surname)
    /// "#, doc);
    /// ```
    TwoColumns {
        /// Whether the two columns should be aligned or not:
        ///
        /// ```
        /// use doku::Document;
        ///
        /// #[derive(Document)]
        /// struct Person {
        ///     /// First name
        ///     /// (aka forename)
        ///     first_name: String,
        ///
        ///     /// Last name
        ///     /// (aka surname)
        ///     last_name: String,
        /// }
        ///
        /// let fmt = doku::toml::Formatting {
        ///     layout: doku::toml::Layout::TwoColumns {
        ///         align: false,
        ///         spacing: 1,
        ///     },
        ///     ..Default::default()
        /// };
        ///
        /// let doc = doku::to_toml_fmt::<Person>(&fmt);
        ///
        /// doku::assert_doc!(r#"
        ///   first_name = "string" # First name
        ///                         ## (aka forename)
        ///
        ///   last_name = "string" # Last name
        ///                        ## (aka surname)
        /// "#, doc);
        /// ```
        align: bool,

        /// Size of the horizontal spacing between both columns:
        ///
        /// ```
        /// use doku::Document;
        ///
        /// #[derive(Document)]
        /// struct Person {
        ///     /// First name
        ///     /// (aka forename)
        ///     first_name: String,
        ///
        ///     /// Last name
        ///     /// (aka surname)
        ///     last_name: String,
        /// }
        ///
        /// let fmt = doku::toml::Formatting {
        ///     layout: doku::toml::Layout::TwoColumns {
        ///         align: true,
        ///         spacing: 5,
        ///     },
        ///     ..Default::default()
        /// };
        ///
        /// let doc = doku::to_toml_fmt::<Person>(&fmt);
        ///
        /// doku::assert_doc!(r#"
        ///   first_name = "string"     # First name
        ///                             ## (aka forename)
        ///
        ///   last_name = "string"      # Last name
        ///                             ## (aka surname)
        /// "#, doc);
        /// ```
        spacing: usize,
    },
}

impl Default for Layout {
    fn default() -> Self {
        Self::OneColumn
    }
}
