use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct Meta {
    key: &'static str,
    value: &'static str,
}

impl Meta {
    pub fn new(key: &'static str, value: &'static str) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &'static str {
        self.key
    }

    pub fn value(&self) -> &'static str {
        self.value
    }
}

#[derive(Clone, Debug, Default)]
pub struct Metas {
    metas: BTreeMap<&'static str, &'static str>,
}

impl Metas {
    pub fn add(&mut self, key: &'static str, value: &'static str) {
        self.metas.insert(key, value);
    }

    pub fn with(mut self, key: &'static str, value: &'static str) -> Self {
        self.add(key, value);
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = Meta> + '_ {
        self.metas.iter().map(|(&k, &v)| Meta::new(k, v))
    }

    pub fn get(&self, key: &'static str) -> Option<&'static str> {
        self.metas.get(key).cloned()
    }
}
