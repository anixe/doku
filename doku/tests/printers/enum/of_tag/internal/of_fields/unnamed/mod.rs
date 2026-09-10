use crate::prelude::*;

#[derive(Document)]
#[doku(tag = "t")]
enum WithoutUserComments {
    Foo(WithoutUserCommentsFoo),
    Bar(WithoutUserCommentsBar),
    Zar(WithoutUserCommentsZar),
}

#[derive(Document)]
struct WithoutUserCommentsFoo {
    a: String,
}

#[derive(Document)]
struct WithoutUserCommentsBar {
    a: String,
    b: usize,
}

#[derive(Document)]
struct WithoutUserCommentsZar;

#[derive(Document)]
struct WithoutUserCommentsWrapped {
    value: WithoutUserComments,
}

// ---

#[derive(Document)]
#[doku(tag = "t")]
enum WithUserComments {
    /// This is `Foo`
    Foo(WithUserCommentsFoo),

    /// This is `Bar`
    Bar(WithUserCommentsBar),

    /// This is `Zar`
    Zar(WithUserCommentsZar),
}

#[derive(Document)]
struct WithUserCommentsFoo {
    /// Some comment
    a: String,
}

#[derive(Document)]
struct WithUserCommentsBar {
    /// Some comment
    a: String,

    /// Some comment
    b: usize,
}

#[derive(Document)]
struct WithUserCommentsZar;

#[derive(Document)]
struct WithUserCommentsWrapped {
    value: WithUserComments,
}

// ---

printer_test! {
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

    // ---

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
}
