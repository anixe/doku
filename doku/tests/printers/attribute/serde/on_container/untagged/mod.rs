use crate::prelude::*;

#[derive(Serialize, Document)]
struct WrappedEnum {
    value: Enum,
}

#[derive(Serialize, Document)]
struct WrappedFlattenedEnum {
    #[serde(flatten)]
    value: Enum,
}

#[derive(Serialize, Document)]
#[serde(untagged)]
enum Enum {
    A { one: String },
    B { two: String, three: String },
    C(EnumC),
}

#[derive(Serialize, Document)]
struct EnumC {
    four: String,
    five: String,
}

printer_test! {
    "output.enum.json" => to_json(Enum),
    "output.enum.toml" => to_toml(Enum),
    "output.wrapped.json" => to_json(WrappedEnum),
    "output.wrapped.toml" => to_toml(WrappedEnum),
    "output.flatten.json" => to_json(WrappedFlattenedEnum),
    "output.flatten.toml" => to_toml(WrappedFlattenedEnum),
}
