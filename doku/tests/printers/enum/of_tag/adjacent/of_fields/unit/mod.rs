use crate::prelude::*;

#[derive(Document)]
#[doku(tag = "t", content = "c")]
enum WithoutUserComments {
    Foo,
    Bar,
}

#[derive(Document)]
struct WithoutUserCommentsWrapped {
    value: WithoutUserComments,
}

// ---

#[derive(Document)]
#[doku(tag = "t", content = "c")]
enum WithUserComments {
    /// This is `Foo`
    Foo,

    /// This is `Bar`
    Bar,
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

    "output.without-key-quotes.with-user-comments-wrapped.commented.json" => to_json_without_key_quotes(WithUserCommentsWrapped, {
          "enums_style": "Commented",
     }),

     "output.without-key-quotes.with-user-comments-wrapped.separated.json" => to_json_without_key_quotes(WithUserCommentsWrapped, {
          "enums_style": "Separated",
     }),

     // --- TOML ---

     "output.without-user-comments.commented.toml" => to_toml_fmt(WithoutUserComments, {
          "enums_style": "Commented",
     }),

     "output.without-user-comments.separated.toml" => to_toml_fmt(WithoutUserComments, {
          "enums_style": "Separated",
     }),

     "output.without-user-comments-wrapped.commented.toml" => to_toml_fmt(WithoutUserCommentsWrapped, {
          "enums_style": "Commented",
     }),

     "output.without-user-comments-wrapped.separated.toml" => to_toml_fmt(WithoutUserCommentsWrapped, {
          "enums_style": "Separated",
     }),

     "output.with-user-comments.commented.toml" => to_toml_fmt(WithUserComments, {
          "enums_style": "Commented",
     }),

     "output.with-user-comments.separated.toml" => to_toml_fmt(WithUserComments, {
          "enums_style": "Separated",
     }),

     "output.with-user-comments-wrapped.commented.toml" => to_toml_fmt(WithUserCommentsWrapped, {
          "enums_style": "Commented",
     }),

     "output.with-user-comments-wrapped.separated.toml" => to_toml_fmt(WithUserCommentsWrapped, {
          "enums_style": "Separated",
     }),
}
