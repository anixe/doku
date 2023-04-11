type Papers = Vec<Paper>;

struct Paper {
    width: usize,
    height: usize,
}

/// An equivalent of:
///
/// ```
/// struct Paper {
///     size: String,
/// }
/// ```
impl doku::Document for Paper {
    fn ty() -> doku::Type {
        doku::Type::from(doku::TypeKind::Struct {
            fields: doku::Fields::Named {
                fields: vec![(
                    "size",
                    doku::Field {
                        ty: String::ty(),
                        flattened: false,
                        aliases: &[],
                    },
                )],
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
    println!("```json");
    println!("{}", doku::to_json::<Papers>());
    println!("```");
    println!();
    println!("```toml");
    println!("{}", doku::to_toml::<Papers>());
    println!("```");
}
