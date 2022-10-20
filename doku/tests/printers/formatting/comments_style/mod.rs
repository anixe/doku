use crate::prelude::*;

#[derive(Document)]
struct Person {
    /// First name
    name: String,
}

printer_test! {
    "output.default_separator_one_column.json" => to_json_fmt(Person, {
    }),

    "output.default_separator_two_columns.json" => to_json_fmt(Person, {
        "layout": { "TwoColumns": { "align": false, "spacing": 2 }}
    }),

    "output.custom_separator_one_column.json" => to_json_fmt(Person, {
        "comments_style": { "separator": "#" },
   }),

   "output.custom_separator_two_columns.json" => to_json_fmt(Person, {
        "comments_style": { "separator": "#" },
        "layout": { "TwoColumns": { "align": false, "spacing": 2 }}
   }),

   "output.default_separator_one_column.toml" => to_toml_fmt(Person, {
    }),

    "output.default_separator_two_columns.toml" => to_toml_fmt(Person, {
        "layout": { "TwoColumns": { "align": false, "spacing": 2 }}
    }),

    "output.custom_separator_one_column.toml" => to_toml_fmt(Person, {
        "comments_style": { "separator": "//" },
    }),

    "output.custom_separator_two_columns.toml" => to_toml_fmt(Person, {
        "comments_style": { "separator": "//" },
        "layout": { "TwoColumns": { "align": false, "spacing": 2 }}
    }),
}
