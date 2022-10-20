use super::*;

pub trait Table {
    /// Whether a field should be displayed as a table or a simple attribute.
    ///
    /// For example, in the code below, the `name` field is displayed as a simple attribute,
    /// while the `address` field is displayed as a table.
    ///
    /// ```
    /// use doku::Document;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     name: String,
    ///     address: Address,
    /// }
    ///
    /// #[derive(Document)]
    /// struct Address {
    ///     street: String,
    ///     number: u32
    /// }
    ///
    ///
    /// let doc = doku::to_toml::<Person>();
    ///
    /// doku::assert_doc!(r#"
    ///   name = "string"
    ///
    ///   [address]
    ///   street = "string"
    ///   number = 123
    /// "#, doc);
    /// ```
    fn is_table(&self) -> bool;

    /// Whether we should write the name / title of the current table field.
    ///
    /// There are currently 3 cases where the field is a table but we don't want to write its title:
    ///
    /// 1. For optionals, since the table title will be written by the child type.
    /// Writing the table title here would result in having the same title displayed twice.
    ///
    /// 2. For arrays, similarly as above.
    ///
    /// 3. For structs, when they have no simple fields, so every child field is a table.
    /// In this case writing the table title is not wrong, but it's unneccessary since every child
    /// field will write their own table.
    fn should_write_table_name(&self) -> bool;
}

impl<'ty> Table for Ctxt<'_, 'ty, '_> {
    fn is_table(&self) -> bool {
        self.ty.is_table()
    }

    fn should_write_table_name(&self) -> bool {
        self.has_array_parent() || self.ty.should_write_table_name()
    }
}

impl Table for Fields {
    fn is_table(&self) -> bool {
        match self {
            Fields::Named { .. } => true,
            Fields::Unnamed { fields } => fields.iter().any(|f| f.is_table()),
            Fields::Unit => false,
        }
    }

    fn should_write_table_name(&self) -> bool {
        match self {
            Fields::Named { fields } => {
                !fields.iter().all(|(_, f)| f.is_table())
            }
            _ => true,
        }
    }
}

impl Table for Field {
    fn is_table(&self) -> bool {
        !self.flattened && self.ty.is_table()
    }

    fn should_write_table_name(&self) -> bool {
        self.ty.should_write_table_name()
    }
}

impl Table for Type {
    fn is_table(&self) -> bool {
        match &self.kind {
            TypeKind::Array { ty, .. } => ty.is_table(),
            TypeKind::Enum {
                tag: Tag::External,
                variants,
                ..
            } => variants.iter().any(|v| !matches!(v.fields, Fields::Unit)),
            TypeKind::Enum {
                tag: Tag::None,
                variants,
                ..
            } => variants.iter().any(|v| v.fields.is_table()),
            TypeKind::Enum { .. } => true,
            TypeKind::Struct {
                fields,
                transparent: false,
            } => fields.is_table(),
            TypeKind::Struct {
                fields: Fields::Named { fields },
                transparent: true,
            } => fields.first().map(|(_, f)| f.is_table()).unwrap_or(false),
            TypeKind::Struct {
                fields: Fields::Unnamed { fields },
                transparent: true,
            } => fields.first().map(|f| f.is_table()).unwrap_or(false),
            TypeKind::Tuple { fields } => fields.iter().any(|t| t.is_table()),
            TypeKind::Optional { ty } => ty.is_table(),
            TypeKind::Map { .. } => true,
            _ => false,
        }
    }

    fn should_write_table_name(&self) -> bool {
        match &self.kind {
            TypeKind::Struct {
                fields,
                transparent: false,
            } => fields.should_write_table_name(),
            TypeKind::Optional { .. } | TypeKind::Array { .. } => false,
            _ => true,
        }
    }
}
