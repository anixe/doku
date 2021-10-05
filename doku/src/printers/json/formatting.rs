mod auto_comments;
mod doc_comments;
mod enums_style;
mod indent_style;
mod layout;
mod values_style;

pub use self::{
    auto_comments::*, doc_comments::*, enums_style::*, indent_style::*,
    layout::*, values_style::*,
};

use crate::*;

/// Determines the look & feel of the documentation.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Formatting {
    /// Determines which auto-comments - _hints_, so to say - should get
    /// displayed.
    pub auto_comments: AutoComments,

    /// Determines if doc-comments should get displayed.
    pub doc_comments: DocComments,

    /// Determines how enums should get displayed.
    pub enums_style: EnumsStyle,

    /// Determines the indenting style.
    pub indent_style: IndentStyle,

    /// Determines whether the document should contain one or two columns.
    pub layout: Layout,

    /// Determines how values should get displayed.
    pub values_style: ValuesStyle,
}
