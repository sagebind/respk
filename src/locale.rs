use std::fmt;


/// A specific locale that can be assigned to a resource.
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

/// Filters what locales to use to resolve a resource.
#[derive(Clone)]
pub enum LocaleFilter {
    /// Only match resources without a locale. This is the default filter.
    None,

    /// Allow all locales. No specific resolution is guaranteed and the order of locales used should not be relied upon.
    All,

    /// Only match the given locale.
    Only(Locale),

    /// Match any of the locales. Locales are resolved in the order given. If a resource is missing in the first locale,
    /// the second locale is tried, and so on, until a resource is found or the end of the list is reached.
    Any(Vec<Locale>),
}

impl Default for LocaleFilter {
    fn default() -> LocaleFilter {
        LocaleFilter::None
    }
}
