type Papers = Vec<Paper>;

struct Paper {
    width:  usize,
    height: usize,
}

/// An equivalent of:
///
/// ```
/// struct Paper {
///     size: String,
/// }
/// ```
impl doku::ty::Provider for Paper {
    fn ty() -> doku::ty::Type {
        doku::ty::Type::from_def(doku::ty::Def::Struct {
            fields:      doku::ty::Fields::Named {
                fields: vec![("size", doku::ty::Field {
                    ty:        String::ty(),
                    flattened: false,
                })],
            },
            transparent: false,
        })
    }
}

impl serde::Serialize for Paper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("Paper", 1)?;

        let size = match (self.width, self.height) {
            (594, 841) => "A1",
            (420, 594) => "A2",
            _ => panic!("paper machine does brrr"),
        };

        s.serialize_field("size", size)?;
        s.end()
    }
}

fn main() {
    println!("{}", doku::to_json::<Papers>());
}
