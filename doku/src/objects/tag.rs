/// Defines the way enums are represented (<https://serde.rs/enum-representations.html>)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tag {
    /// E.g.: `#[serde(tag = "t", content = "c")]`
    Adjacent {
        tag: &'static str,
        content: &'static str,
    },

    /// E.g.: `#[serde(tag = "t")]`
    Internal { tag: &'static str },

    /// The default enum's representation
    External,

    /// E.g.: `#[serde(untagged)]`
    None,
}
