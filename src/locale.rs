use std::fmt;


#[derive(Clone, Eq, PartialEq)]
pub struct Locale {
    name: String,
}

impl<S> From<S> for Locale where S: Into<String> {
    fn from(name: S) -> Locale {
        Locale {
            name: name.into(),
        }
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name)
    }
}

pub enum LocaleFilter {
    All,
    Only(Locale),
    Any(Vec<Locale>),
}
