use crate::prelude::*;

#[derive(Document)]
#[doku(tag = "t")]
enum WithoutUserComments {
    Foo { a: String },
    Bar { a: String, b: usize },
}

#[derive(Document)]
struct WithoutUserCommentsWrapped {
    value: WithoutUserComments,
}

// ---

#[derive(Document)]
#[doku(tag = "t")]
enum WithUserComments {
    /// This is `Foo`
    Foo {
        /// Some comment
        a: String,
    },

    /// This is `Bar`
    Bar {
        /// Some comment
        a: String,

        /// Some comment
        b: usize,
    },
}

#[derive(Document)]
struct WithUserCommentsWrapped {
    value: WithUserComments,
}

// ---

printer_test! {
    "output.without-user-comments.commented.json" => to_json_fmt(WithoutUserComments, {
         "enums_style": "Commented",
    }),

    "output.without-user-comments.separated.json" => to_json_fmt(WithoutUserComments, {
         "enums_style": "Separated",
    }),

    "output.without-user-comments-wrapped.commented.json" => to_json_fmt(WithoutUserCommentsWrapped, {
         "enums_style": "Commented",
    }),

    "output.without-user-comments-wrapped.separated.json" => to_json_fmt(WithoutUserCommentsWrapped, {
         "enums_style": "Separated",
    }),

    // ---

    "output.with-user-comments.commented.json" => to_json_fmt(WithUserComments, {
         "enums_style": "Commented",
    }),

    "output.with-user-comments.separated.json" => to_json_fmt(WithUserComments, {
         "enums_style": "Separated",
    }),

    "output.with-user-comments-wrapped.commented.json" => to_json_fmt(WithUserCommentsWrapped, {
         "enums_style": "Commented",
    }),

    "output.with-user-comments-wrapped.separated.json" => to_json_fmt(WithUserCommentsWrapped, {
         "enums_style": "Separated",
    }),

    "output.without_key_quotes.with-user-comments-wrapped.commented.json" => to_json_fmt(WithUserCommentsWrapped, {
          "objects_style": { "surround_keys_with_quotes": false },
          "enums_style": "Commented",
     }),

     "output.without_key_quotes.with-user-comments-wrapped.separated.json" => to_json_fmt(WithUserCommentsWrapped, {
          "objects_style": { "surround_keys_with_quotes": false },
          "enums_style": "Separated",
     }),
}
