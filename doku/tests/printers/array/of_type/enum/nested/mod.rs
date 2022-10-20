use crate::prelude::*;

type Ty = Vec<Inner>;

#[derive(Document)]
#[doku(tag = "t")]
enum Inner {
    /// This is `A` (Untagged)
    A(Untagged),

    /// This is `B` (AdjacentlyTagged)
    B(AdjacentlyTagged),

    /// This is `C` (InternallyTagged)
    C(InternallyTagged),

    /// This is `D` (ExternallyTagged)
    D(ExternallyTagged),

    /// This is `E` (Complex)
    E(Complex),
}

#[derive(Document)]
#[doku(untagged)]
enum Untagged {
    A1 {
        /// Some comment
        something: usize,

        /// Another comment
        something_else: usize,
    },

    A2 {
        /// Some comment
        a: String,

        /// Another comment
        b: String,
    },
}

#[derive(Document)]
#[doku(tag = "t2", content = "c")]
enum AdjacentlyTagged {
    B1 {
        /// Some comment
        something: usize,

        /// Another comment
        something_else: usize,
    },

    B2 {
        /// Some comment
        a: String,

        /// Another comment
        b: usize,
    },
}

#[derive(Document)]
#[doku(tag = "t2")]
enum InternallyTagged {
    C1 {
        /// Some comment
        something: usize,

        /// Another comment
        something_else: usize,
    },

    C2 {
        /// Some comment
        a: String,

        /// Another comment
        b: usize,
    },
}

#[derive(Document)]
enum ExternallyTagged {
    D1 {
        /// Some comment
        something: usize,

        /// Another comment
        something_else: usize,
    },

    D2 {
        /// Some comment
        a: String,

        /// Another comment
        b: usize,
    },
}

#[derive(Document)]
struct Complex {
    name: String,

    /// This is `b` (AdjacentlyTagged)
    b: Option<AdjacentlyTagged>,

    /// This is `c` (InternallyTagged)
    c: Option<InternallyTagged>,
}

printer_test! {
    "output.json" => to_json(Ty),
    "output.toml" => to_toml(TomlWrapper<Ty>),
}
