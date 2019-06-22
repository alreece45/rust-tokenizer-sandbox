use std::borrow::Cow;
use std::collections::HashMap;

pub struct Session<'a> {
    options: HashMap<Cow<'a, str>, Cow<'a, str>>,
    definitions: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl<'a> Session<'a> {
    pub fn new() -> Self {
        Self {
            options: HashMap::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn define<K, V>(&mut self, symbol: K, value: V)
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.definitions.insert(symbol.into(), value.into());
    }

    pub fn definitions(&self) -> impl Iterator<Item = (&Cow<'a, str>, &Cow<'a, str>)> {
        self.definitions.iter()
    }

    pub fn set_option<K, V>(&mut self, config: K, value: V)
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.options.insert(config.into(), value.into());
    }

    pub fn remove_definition<K>(&mut self, symbol: K) -> Option<Cow<'a, str>>
    where
        K: AsRef<str>,
    {
        self.definitions.remove(symbol.as_ref())
    }

    pub fn remove_option<K>(&mut self, option: K) -> Option<Cow<'a, str>>
    where
        K: AsRef<str>,
    {
        self.options.remove(option.as_ref())
    }
}
