use crate::prelude::*;

#[derive(Document)]
#[doku(tag = "t", content = "c")]
enum WithoutUserComments {
    Foo(String),
    Bar(String, usize),
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
    Foo(String),

    /// This is `Bar`
    Bar(String, usize),
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
}
