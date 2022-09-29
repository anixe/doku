use crate::*;

#[derive(Clone, Debug)]
pub struct Type {
    pub comment: Option<&'static str>,
    pub example: Option<Example>,
    pub metas: Metas,

    /// When we have an adjacently-tagged enum, this field contains name of the
    /// field that should represent that enum's tag.
    pub tag: Option<&'static str>,

    /// Whether this type is serializable or not (think
    /// `#[serde(skip_serializing)]`).
    pub serializable: bool,

    /// Whether this type is deserializable or not (think
    /// `#[serde(skip_deserializing)]`).
    pub deserializable: bool,

    // Keeping the kind last improves legibility of debug-printing
    pub kind: TypeKind,
}

impl From<TypeKind> for Type {
    fn from(kind: TypeKind) -> Self {
        Self {
            comment: None,
            example: None,
            metas: Metas::default(),
            tag: None,
            serializable: true,
            deserializable: true,
            kind,
        }
    }
}

impl From<Fields> for Type {
    fn from(fields: Fields) -> Self {
        let kind: TypeKind = fields.into();
        kind.into()
    }
}

impl Type {
    /// Recursively walks through the current type, and when an enum is found,
    /// all its variants are converted to equivalent structs.
    pub(crate) fn enum_to_structs(&self) -> Option<Vec<Self>> {
        match &self.kind {
            TypeKind::Enum {
                tag: Tag::Adjacent { tag, content },
                variants,
            } => Some(
                variants
                    .iter()
                    .map(|variant| {
                        self.to_owned()
                            .adjacent_variant_to_struct(variant, tag, content)
                    })
                    .collect(),
            ),
            TypeKind::Enum {
                tag: Tag::Internal { tag },
                variants,
            } => Some(
                variants
                    .iter()
                    .map(|variant| {
                        self.to_owned().internal_variant_to_struct(variant, tag)
                    })
                    .collect(),
            ),
            TypeKind::Enum {
                tag: Tag::External,
                variants,
            } => Some(
                variants
                    .iter()
                    .map(|variant| {
                        self.to_owned().external_variant_to_struct(variant)
                    })
                    .collect(),
            ),
            TypeKind::Enum {
                tag: Tag::None,
                variants,
            } => Some(
                variants
                    .iter()
                    .map(|variant| {
                        self.to_owned().untagged_variant_to_struct(variant)
                    })
                    .collect(),
            ),
            TypeKind::Array { ty, size } => {
                ty.enum_to_structs().map(|mut types| {
                    types
                        .drain(..)
                        .map(|ty| {
                            self.with_kind(TypeKind::Array {
                                ty: Box::new(ty),
                                size: size.to_owned(),
                            })
                        })
                        .collect()
                })
            }
            TypeKind::Map { key, value } => {
                value.enum_to_structs().map(|mut types| {
                    types
                        .drain(..)
                        .map(|ty| {
                            self.with_kind(TypeKind::Map {
                                key: key.to_owned(),
                                value: Box::new(ty),
                            })
                        })
                        .collect()
                })
            }
            TypeKind::Optional { ty } => {
                ty.enum_to_structs().map(|mut types| {
                    types
                        .drain(..)
                        .map(|ty| {
                            self.with_kind(TypeKind::Optional {
                                ty: Box::new(ty),
                            })
                        })
                        .collect()
                })
            }
            TypeKind::Struct {
                fields: Fields::Named { fields },
                ..
            } => fields.iter().enumerate().find_map(|(i, (name, f))| {
                f.ty.enum_to_structs().map(|mut types| {
                    types
                        .drain(..)
                        .map(|ty| {
                            self.to_owned().replace_named_field_of_struct(
                                (name, ty.into()),
                                i,
                            )
                        })
                        .collect()
                })
            }),
            TypeKind::Struct {
                fields: Fields::Unnamed { fields },
                ..
            } => fields.iter().enumerate().find_map(|(i, f)| {
                f.ty.enum_to_structs().map(|mut types| {
                    types
                        .drain(..)
                        .map(|ty| {
                            self.to_owned()
                                .replace_unnamed_field_of_struct(ty.into(), i)
                        })
                        .collect()
                })
            }),
            TypeKind::Tuple { fields } => {
                fields.iter().enumerate().find_map(|(i, ty)| {
                    ty.enum_to_structs().map(|mut types| {
                        types
                            .drain(..)
                            .map(|ty| {
                                self.to_owned().replace_field_of_tuple(ty, i)
                            })
                            .collect()
                    })
                })
            }
            _ => None,
        }
    }

    /// Converts an adjacently tagged variant of an enum to an equivalent struct.
    ///
    /// This is done by creating a struct with the `tag`, `content` fields.
    fn adjacent_variant_to_struct(
        mut self,
        variant: &Variant,
        tag: &'static str,
        content: &'static str,
    ) -> Self {
        let mut new_fields = Vec::new();
        let mut tag_type: Type = TypeKind::String.into();
        tag_type.example = Some(Example::Simple(variant.id));

        new_fields.push((tag, tag_type.into()));
        new_fields.push((content, variant.fields.clone().into()));

        self.kind = Fields::Named { fields: new_fields }.into();
        self
    }

    /// Converts an internally tagged variant of an enum to an equivalent struct.
    ///
    /// This is done by creating a struct with the `tag` field, and then
    /// appending the fields of the original variant to it.
    fn internal_variant_to_struct(
        mut self,
        variant: &Variant,
        tag: &'static str,
    ) -> Self {
        let mut new_fields = Vec::new();
        let mut tag_type: Type = TypeKind::String.into();
        tag_type.example = Some(Example::Simple(variant.id));
        new_fields.push((tag, tag_type.into()));

        match variant.fields.clone() {
            Fields::Named { mut fields } => new_fields.append(&mut fields),
            Fields::Unnamed { .. } => panic!(
                "Internally tagged unnamed variants are unsupported in TOML"
            ),
            Fields::Unit => {}
        };

        self.kind = Fields::Named { fields: new_fields }.into();
        self
    }

    /// Converts an externally tagged variant of an enum to an equivalent struct.
    ///
    /// This is done by creating a struct with just one field, whose name is the
    /// name of the variant and its type is another struct with the original
    /// fields of the variant.
    fn external_variant_to_struct(mut self, variant: &Variant) -> Self {
        let mut new_fields = Vec::new();
        let new_type: Type = variant.fields.clone().into();
        new_fields.push((variant.id, new_type.into()));

        self.kind = Fields::Named { fields: new_fields }.into();
        self
    }

    /// Converts an untagged variant of an enum to an equivalent struct.
    ///
    /// This is done by creating a struct that contains the fields of the
    /// original variant.
    fn untagged_variant_to_struct(mut self, variant: &Variant) -> Self {
        self.kind = variant.fields.clone().into();
        self
    }

    fn with_kind(&self, kind: TypeKind) -> Self {
        Self {
            comment: self.comment,
            example: self.example,
            metas: self.metas.clone(),
            tag: self.tag,
            serializable: self.serializable,
            deserializable: self.deserializable,
            kind,
        }
    }

    fn replace_named_field_of_struct(
        mut self,
        new_field: (&'static str, Field),
        i: usize,
    ) -> Self {
        if let TypeKind::Struct {
            fields: Fields::Named { fields },
            ..
        } = &mut self.kind
        {
            fields[i] = new_field;
        }
        self
    }

    fn replace_unnamed_field_of_struct(
        mut self,
        new_field: Field,
        i: usize,
    ) -> Self {
        if let TypeKind::Struct {
            fields: Fields::Unnamed { fields },
            ..
        } = &mut self.kind
        {
            fields[i] = new_field;
        }
        self
    }

    fn replace_field_of_tuple(mut self, new_field: Type, i: usize) -> Self {
        if let TypeKind::Tuple { fields } = &mut self.kind {
            fields[i] = new_field;
        }
        self
    }
}
