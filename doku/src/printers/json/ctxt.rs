use super::*;

pub struct Ctxt<'p, 'ty, 'out> {
    pub ty: &'ty Type,
    pub val: Option<&'ty Value>,
    pub vis: Visibility,
    pub fmt: &'p Formatting,
    pub out: &'out mut Output<'p>,

    /// Parent of `ty`.
    ///
    /// # Example
    ///
    /// When processing `usize` of `Option<usize>`, this `Option<usize>` would
    /// be the parent.
    pub parent: Option<&'ty Type>,

    /// When present, overrides `ty.example`; used to propagate examples through
    /// optional values.
    pub example: Option<Example>,

    /// When enabled, `ty` will be printed as flat / transparent.
    ///
    /// # Example
    ///
    /// Without `flat`:
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
    /// With `flat`:
    ///
    /// ```json
    /// {
    ///   "a": "string",
    ///   "b": "string",
    ///   "c": "string",
    /// }
    /// ```
    pub flat: bool,

    /// When enabled, `ty` will be printed in one line.
    ///
    /// # Example
    ///
    /// Without `inline`:
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
    /// With `inline`:
    ///
    /// ```json
    /// { "a": "string", "b": "string" }
    /// ```
    pub inline: bool,

    /// Incremented each time `Ctxt::nested()` is called; used to detect
    /// recursion.
    pub depth: u8,
}

impl<'p, 'ty, 'out> Ctxt<'p, 'ty, 'out> {
    /// Clones `self`, but preserves the same `self.out` (hence the returned
    /// object has to have a shorter output borrow's lifetime - `out2`).
    pub fn nested<'out2>(&'out2 mut self) -> Ctxt<'p, 'ty, 'out2> {
        Ctxt {
            ty: self.ty,
            val: self.val,
            vis: self.vis,
            fmt: self.fmt,
            out: self.out,
            parent: self.parent,
            example: self.example,
            flat: self.flat,
            inline: self.inline,
            depth: self.depth.checked_add(1).expect(
                "Seems like the printer got stuck; this might indicate a bug \
                 in Doku or a recursive type in your code-base",
            ),
        }
    }

    pub fn with_ty(mut self, ty: &'ty Type) -> Self {
        self.parent = Some(self.ty);

        // When we have a struct with a flattened field, all the inner fields of
        // _that_ type are not subject for flattening anymore:
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
        //   f1: ..., // <- doesn't get flattened
        //   f2: ..., // <- doesn't get flattened
        // }
        //
        // ```
        //
        // There's one edge-case here though - when we flatten a transparent
        // field, the `flat` flag has to be carried further, because otherwise
        // we'd essentially ignore the transparency requirement.
        let keep_flat = matches!(
            self.ty.kind,
            TypeKind::Struct {
                transparent: true,
                fields: _,
            }
        );

        self.ty = ty;
        self.flat = self.flat && keep_flat;
        self.example = None;
        self
    }

    pub fn with_val(mut self, val: Option<&'ty Value>) -> Self {
        self.val = val;
        self
    }

    pub fn with_example(mut self, example: Option<impl Into<Example>>) -> Self {
        self.example = example.map(Into::into);
        self
    }

    pub fn with_flat(mut self) -> Self {
        self.flat = true;
        self
    }

    pub fn example(&self) -> Option<Example> {
        self.example.or(self.ty.example)
    }

    pub fn first_example(&self) -> Option<&'static str> {
        self.example().and_then(Example::first)
    }

    pub fn literal_example(&self) -> Option<&'static str> {
        self.example().and_then(|example| {
            if let Example::Literal(example) = example {
                Some(example)
            } else {
                None
            }
        })
    }

    pub fn print(mut self) {
        use super::*;

        if !self
            .vis
            .allows(self.ty.serializable, self.ty.deserializable)
        {
            return;
        }

        self.print_comment();

        if !self.inline {
            if let Some(example) = self.literal_example() {
                self.out.write(example);
                return;
            }
        }

        match &self.ty.kind {
            TypeKind::Bool => self.print_bool(),
            TypeKind::Float => self.print_float(),
            TypeKind::Integer => self.print_integer(),
            TypeKind::String => self.print_string(),
            TypeKind::Array { ty, size } => self.print_array(ty, *size),
            TypeKind::Enum { tag, variants } => self.print_enum(*tag, variants),
            TypeKind::Struct {
                fields,
                transparent,
            } => self.print_struct(fields, *transparent, None),
            TypeKind::Tuple { fields } => self.print_tuple(fields),
            TypeKind::Map { key, value } => self.print_map(key, value),
            TypeKind::Optional { ty } => self.print_optional(ty),
        }
    }
}
