use doku::{Document, Fields, TypeKind};

#[test]
fn serde_aliases_are_collected_by_document() {
    #[derive(serde::Deserialize, Document)]
    #[allow(dead_code)]
    struct Thing {
        #[serde(alias = "foos")]
        foo: String,

        #[serde(alias = "bat")]
        #[serde(alias = "baz")]
        bar: i32,
    }

    if let doku::Type {
        kind:
            TypeKind::Struct {
                fields: Fields::Named { fields },
                ..
            },
        ..
    } = Thing::ty()
    {
        let foo_aliases = fields
            .iter()
            .find(|(name, _)| *name == "foo")
            .unwrap()
            .1
            .aliases;
        let bar_aliases = fields
            .iter()
            .find(|(name, _)| *name == "bar")
            .unwrap()
            .1
            .aliases;
        assert_eq!(foo_aliases, ["foos"]);
        assert_eq!(bar_aliases, ["bat", "baz"]);
    } else {
        unreachable!();
    }
}
