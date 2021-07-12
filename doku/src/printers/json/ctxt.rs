use super::*;
use std::iter::once;

/// This struct gathers all the information required for the `print_*`
/// functions; it's easier to pass it around than a bunch of loose variables.
pub struct Ctxt<'ty, 'out> {
    /// Whether we should print serializable / deserializable types
    pub mode: TypePrinterMode,

    /// Object containing everything we've printed so far
    pub out: &'out mut Paragraph,

    /// Type we're currently printing
    pub ty: &'ty Type,

    /// Parent(s) of the type we're currently printing.
    ///
    /// E.g. for `Array<Option<T>>` we could have:
    ///
    /// - `parents[0] = Array<Option<T>>`
    /// - `parents[1] = Option<T>`
    /// - `ty = T`
    pub parents: Vec<&'ty Type>,

    /// When enabled, `ty` will be printed "flat".
    ///
    /// Currently we utilize this flag for flattening key-value pairs of structs
    /// and maps which have been covered with the `#[serde(flatten)]` attribute.
    ///
    /// # Example
    ///
    /// For instance, was `inner-struct` to be flattened here:
    ///
    /// ```json
    /// {
    ///   "a": "string",
    ///
    ///   "inner-struct": {
    ///     "b": "string",
    ///     "c": "string",
    ///   }
    /// }
    /// ```
    ///
    /// ... we'd end up with:
    ///
    /// ```json
    /// {
    ///   "a": "string",
    ///   "b": "string",
    ///   "c": "string",
    /// }
    /// ```
    ///
    /// # Caveats
    ///
    /// Setting this flag for types that cannot be flattened (e.g. scalars) is a
    /// no-op.
    pub flat: bool,

    /// When enabled, `ty` will be printed in one line.
    ///
    /// Currently we utilize this flag for printing comments for complex
    /// enumeration types.
    ///
    /// # Example
    ///
    /// For instance, instead of:
    ///
    /// ```json
    /// {
    ///   // Some field
    ///   "a": "string",
    ///
    ///   // Some other field
    ///   "b": "string",
    /// }
    /// ```
    ///
    /// ... you'd end up with:
    ///
    /// ```json
    /// { "a": "string", "b": "string" }
    /// ```
    ///
    /// # Caveats
    ///
    /// Enabling this flag disables printing comments (because, as you might
    /// have noticed, there's no way to render them properly when everything
    /// is just a one big pile-line of JSON).
    pub inline: bool,
}

impl<'ty, 'out> Ctxt<'ty, 'out> {
    pub fn with_ty<'out2>(&'out2 mut self, ty: &'ty Type) -> Ctxt<'ty, 'out2> {
        let parents = {
            let mut parents = self.parents.clone();
            parents.push(self.ty);
            parents
        };

        // When we have a struct with a flattened field, all the *inner* fields
        // of _that_ type are not subject to flattening anymore:
        //
        // ```
        // struct T1 {
        //   #[doku(flatten)]
        //   f: T2,
        // }
        //
        // struct T2 {
        //   f: T3, // <- gets flattened
        // }
        //
        // struct T3 {
        //   f1: T4, // <- doesn't get flattened
        //   f2: T4, // <- doesn't get flattened
        // }
        //
        // struct T4 {
        //   something: String, // <- doesn't get flattened
        // }
        // ```
        //
        // Here's comparison:
        //
        // ```
        // # With keep_flat always being true:
        //
        // {
        //   something: "string",
        // }
        // ```
        //
        // ```
        // # With keep_flat adjusting to the context:
        //
        // {
        //   "f1": {
        //     "something": "string"
        //   },
        //   "f2": {
        //     "something": "string"
        //   }
        // }
        // ```
        //
        // There's one edge-case here though - when we flatten a transparent
        // field, the `flat` flag has to be carried further, because otherwise
        // we'd essentially ignore the transparency requirement.
        //
        // To avoid making this comment overly long, if you want to see this
        // condition in action, please comment it out and run the tests :-)
        let keep_flat = matches!(self.ty.def, TypeDef::Struct { transparent: true, .. });

        Ctxt {
            out: self.out,
            ty,
            parents,
            mode: self.mode,
            flat: self.flat && keep_flat,
            inline: self.inline,
        }
    }

    pub fn with_flat(mut self) -> Self {
        self.flat = true;
        self
    }

    pub fn example(&self) -> Option<&'static str> {
        once(&self.ty)
            .chain(self.parents.iter().rev())
            .find_map(|ty| ty.example)
    }

    pub fn print(mut self) {
        use super::*;

        if !self.mode.allows(self.ty.serializable, self.ty.deserializable) {
            return;
        }

        self.print_comment();

        match &self.ty.def {
            TypeDef::Bool => self.print_bool(),
            TypeDef::Float => self.print_float(),
            TypeDef::Integer => self.print_integer(),
            TypeDef::String => self.print_string(),
            TypeDef::Array { ty, .. } => self.print_array(ty),
            TypeDef::Enum { tag, variants } => self.print_enum(*tag, variants),
            TypeDef::Struct { fields, transparent } => self.print_struct(fields, *transparent, None),
            TypeDef::Tuple { fields } => self.print_tuple(fields),
            TypeDef::Map { key, value } => self.print_map(key, value),
            TypeDef::Optional { ty } => self.print_optional(ty),
        }
    }
}
