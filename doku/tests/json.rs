use self::{framework::*, prelude::*};

// Specifying path is a no-op, but rustfmt fails to notice the file otherwise
#[path = "framework.rs"]
#[macro_use]
mod framework;

crate::test_suite!(json, [
    attribute/serde/on_container/crate
    attribute/serde/on_container/default
    attribute/serde/on_container/deny_unknown_fields
    attribute/serde/on_container/tag
    attribute/serde/on_container/tag_and_content
    attribute/serde/on_container/transparent
    attribute/serde/on_container/untagged
    attribute/serde/on_variant/deserialize_with
    attribute/serde/on_variant/other
    attribute/serde/on_variant/rename
    attribute/serde/on_variant/serialize_with
    attribute/serde/on_variant/skip
    attribute/serde/on_variant/skip_deserializing
    attribute/serde/on_variant/skip_serializing
    attribute/serde/on_variant/with
    attribute/serde/on_field/rename
    attribute/serde/on_field/default
    attribute/serde/on_field/skip
    attribute/serde/on_field/skip_serializing
    attribute/serde/on_field/skip_serializing_if
    attribute/serde/on_field/skip_deserializing
    attribute/serde/on_field/serialize_with
    attribute/serde/on_field/deserialize_with
    attribute/serde/on_field/with

    array/of_constant_size
    array/of_type/enum/of_tag/adjacent/of_fields/named
    array/of_type/enum/of_tag/adjacent/of_fields/unit
    array/of_type/enum/of_tag/adjacent/of_fields/unnamed
    array/of_type/enum/of_tag/external/of_fields/named
    array/of_type/enum/of_tag/external/of_fields/unit
    array/of_type/enum/of_tag/external/of_fields/unnamed
    array/of_type/enum/of_tag/none/of_fields/named
    array/of_type/number
    array/of_type/struct
    array/with_examples
    enum/of_tag/adjacent/of_fields/named
    enum/of_tag/adjacent/of_fields/unit
    enum/of_tag/adjacent/of_fields/unnamed
    enum/of_tag/external/of_fields/named
    enum/of_tag/external/of_fields/unit
    enum/of_tag/external/of_fields/unnamed
    enum/of_tag/internal/of_fields/named
    enum/of_tag/internal/of_fields/unit
    enum/of_tag/none/of_fields/named
    enum/of_tag/none/of_fields/unit
    enum/of_tag/none/of_fields/unnamed
    enum/with_comments/flat
    enum/with_comments/nested
    map/of_key/primitive/of_value/primitive
    map/of_key/primitive/of_value/primitive_as_comment
    optional/of_type/enum
    optional/of_type/optional
    optional/of_type/scalar
    struct/of_fields/named
    struct/of_fields/unit
    struct/of_fields/unnamed
    struct/of_transparent
    struct/with_comments
    struct/with_examples
    struct/with_flattened_field
    struct/with_flattened_transparent_field
    struct/with_optional_field
    tuple/of_arity/_1
    tuple/of_arity/_2
    tuple/of_arity/_3
]);

pub fn test_case<T: doku::TypeProvider>(dir: &Path) -> TestCase {
    TestCase {
        expected_path:     dir.join("expected.json"),
        expected_new_path: dir.join("expected.json.new"),
        actual:            doku::to_json::<T>(),
    }
}

mod prelude {
    pub use doku::prelude::*;
    pub use serde::*;
    pub use std::fs;
    pub use std::path::Path;
}
