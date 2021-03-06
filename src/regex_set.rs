//! A type that represents the union of a set of regular expressions.

use regex::RegexSet as RxSet;

/// A dynamic set of regular expressions.
#[derive(Debug)]
pub struct RegexSet {
    items: Vec<String>,
    set: Option<RxSet>,
}

impl RegexSet {
    /// Is this set empty?
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Insert a new regex into this set.
    pub fn insert<S>(&mut self, string: S)
    where
        S: AsRef<str>,
    {
        self.items.push(format!("^{}$", string.as_ref()));
        self.set = None;
    }

    /// Returns slice of String from its field 'items'
    pub fn get_items(&self) -> &[String] {
        &self.items[..]
    }

    /// Construct a RegexSet from the set of entries we've accumulated.
    ///
    /// Must be called before calling `matches()`, or it will always return
    /// false.
    pub fn build(&mut self) {
        self.set = match RxSet::new(&self.items) {
            Ok(x) => Some(x),
            Err(e) => {
                error!("Invalid regex in {:?}: {:?}", self.items, e);
                None
            }
        }
    }

    /// Does the given `string` match any of the regexes in this set?
    pub fn matches<S>(&self, string: S) -> bool
    where
        S: AsRef<str>,
    {
        let s = string.as_ref();
        self.set.as_ref().map(|set| set.is_match(s)).unwrap_or(
            false,
        )
    }
}

impl Default for RegexSet {
    fn default() -> Self {
        RegexSet {
            items: vec![],
            set: None,
        }
    }
}
