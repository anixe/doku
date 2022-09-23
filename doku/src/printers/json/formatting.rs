mod auto_comments;
mod comments_style;
mod doc_comments;
mod enums_style;
mod indent_style;
mod layout;
mod objects_style;
mod values_style;

pub use self::{
    auto_comments::*, comments_style::*, doc_comments::*, enums_style::*,
    indent_style::*, layout::*, objects_style::*, values_style::*,
};

use crate::*;
use std::any;

/// Determines the look & feel of the documentation.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Formatting {
    /// Determines which auto-comments - _hints_, so to say - should get
    /// displayed.
    pub auto_comments: AutoComments,

    /// Determines how comments should get displayed.
    pub comments_style: CommentsStyle,

    /// Determines if doc-comments should get displayed.
    pub doc_comments: DocComments,

    /// Determines how enums should get displayed.
    pub enums_style: EnumsStyle,

    /// Determines the indenting style.
    pub indent_style: IndentStyle,

    /// Determines whether the document should contain one or two columns.
    pub layout: Layout,

    /// Determines how objects should get displayed.
    pub objects_style: ObjectsStyle,

    /// Determines how values should get displayed.
    pub values_style: ValuesStyle,
}

impl Formatting {
    pub(crate) fn customize(&self, metas: impl Iterator<Item = Meta>) -> Self {
        let mut this = serde_json::to_value(self).unwrap();

        for meta in metas {
            if meta.key() == "fmt" {
                this =
                    serde_json::from_str(meta.value()).unwrap_or_else(|err| {
                        panic!(
                            "Not a valid {}: {}",
                            any::type_name::<Self>(),
                            err
                        )
                    });
            } else if let Some(key) = meta.key().strip_prefix("fmt") {
                let key = key.replace('.', "/");

                let this = this.pointer_mut(&key).unwrap_or_else(|| {
                    panic!(
                        "Tried to overwrite a non-existing formatting option: {}",
                        key
                    );
                });

                *this = serde_json::from_str(meta.value())
                    .unwrap_or_else(|err| panic!("Not a valid JSON: {}", err));
            }
        }

        serde_json::from_value(this).unwrap()
    }
}
