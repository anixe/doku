#[derive(Clone, Copy, Debug)]
pub enum Example {
    /// `#[doku(literal_example = "foo")]`
    Literal(&'static str),

    /// `#[doku(example = "foo")]`
    Simple(&'static str),

    /// `#[doku(example = "one", example = "two")]`
    Compound(&'static [&'static str]),
}

impl Example {
    pub fn first(self) -> Option<&'static str> {
        match self {
            Example::Literal(example) => Some(example),
            Example::Simple(example) => Some(example),
            Example::Compound(examples) => examples.first().copied(),
        }
    }

    pub fn iter(self) -> impl Iterator<Item = &'static str> {
        let (example, examples) = match self {
            Example::Literal(example) => (Some(example), None),
            Example::Simple(example) => (Some(example), None),
            Example::Compound(examples) => (None, Some(examples)),
        };

        example
            .into_iter()
            .chain(examples.into_iter().flatten().copied())
    }
}

impl From<&'static str> for Example {
    fn from(example: &'static str) -> Self {
        Self::Simple(example)
    }
}

impl From<&'static [&'static str]> for Example {
    fn from(examples: &'static [&'static str]) -> Self {
        if examples.len() == 1 {
            Self::Simple(examples[0])
        } else {
            Self::Compound(examples)
        }
    }
}
