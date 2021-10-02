#[derive(Clone, Copy, Debug)]
pub enum Example {
    Simple(&'static str),
    Compound(&'static [&'static str]),
}

impl Example {
    pub fn first(self) -> Option<&'static str> {
        match self {
            Self::Simple(example) => Some(example),
            Self::Compound(examples) => examples.first().copied(),
        }
    }

    pub fn iter(self) -> impl Iterator<Item = &'static str> {
        let (example, examples) = match self {
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
