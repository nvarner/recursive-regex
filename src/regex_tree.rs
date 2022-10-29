use std::collections::HashMap;

#[cfg(feature = "deserialize-regex-tree")]
use serde::Deserialize;

use crate::regex::{CaptureMatches, CaptureNames, Captures, Regex};

/// A regex tree is a recursive regular expression. Once the root regex of a
/// tree matches a string, if any of its named capture groups match the name of
/// one of its children, it recurses, and the child regex tree will be run on
/// the capture group.
///
/// ## Construction
/// For a regex tree with no children, use [`leaf`](RegexTree::leaf).
///
///
/// For a regex tree with children, use [`root`](RegexTree::root), which returns
/// a [`Builder`](Builder). Add children to the builder with
/// [`with_child`](Builder::with_child`), then finish with
/// [`build`](Builder::build).
///
/// ## Example
/// Consider the following regex tree:
/// ```text
///      (?P<name>[A-Z][a-z]) (?P<opinion_list>(?:true|false ?)*) (?P<favorite_number_list>[-0-9 ]*)
///                                                   |
///                                  -----------------------------------
///                                  |                                 |
///                        child: opinion_list            child: favorite_number_list
///                             true|false                            -?\d+
/// ```
///
/// It takes in a line, parses out an ASCII name, list of boolean opinions, and
/// a list of favorite integers. Then, its children split the space-separated
/// list of opinions into a logical list of "true" and "false", and the
/// space-separated list of numbers into a logical list of numbers.
///
/// So, for the following text:
/// ```text
/// Ying true false false false true false -24 42 123987
/// ```
///
/// The regex tree would begin matching with the root:
/// ```text
/// (name: Ying) (opinion_list: true false false false true false) (favorite_number_list: -24 42 123987)
/// ```
///
/// It would then recurse and match with one of its children:
/// ```text
/// (name: Ying) (opinion_list) (favorite_number_list: -24 42 123987)
///                     |
///  (true) (false) (false) (false) (true) (false)
/// ```
///
/// And the other:
/// ```text
/// (name: Ying) (opinion_list) (favorite_number_list)
///                     |                           |
///  (true) (false) (false) (false) (true) (false)  |
///                                                 |
///                                        (-24) (42) (123987)
/// ```
///
/// In general, the tree can be arbitrarily deep. For instance, if the entries
/// in the opinion list were pairs (boolean number), perhaps indicating
/// belief and strength of belief, the opinion_list could have another child
/// to break up each space-separated pair into a logical tuple.
#[cfg_attr(feature = "deserialize-regex-tree", derive(Deserialize))]
pub struct RegexTree {
    #[cfg_attr(feature = "deserialize-regex-tree", serde(with = "serde_regex"))]
    regex: Regex,
    #[cfg_attr(feature = "deserialize-regex-tree", serde(default))]
    children: HashMap<String, RegexTree>,
}

impl RegexTree {
    /// Begin construction of a regex tree with children. See
    /// [`Builder`](Builder).
    pub fn root(regex: impl ToRegex) -> Builder {
        Builder::new(regex.to_regex())
    }

    /// Construct a regex tree with no children.
    pub fn leaf(regex: impl ToRegex) -> Self {
        Self {
            regex: regex.to_regex(),
            children: HashMap::new(),
        }
    }

    pub(crate) fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
        self.regex.captures(text)
    }

    pub(crate) fn captures_iter<'r, 't>(&'r self, text: &'t str) -> CaptureMatches<'r, 't> {
        self.regex.captures_iter(text)
    }

    pub(crate) fn names(&self) -> CaptureNames {
        self.regex.capture_names()
    }

    pub(crate) fn child(&self, name: &str) -> Option<&RegexTree> {
        self.children.get(name)
    }
}

pub struct Builder {
    regex: Regex,
    children: HashMap<String, RegexTree>,
}

impl Builder {
    fn new(regex: Regex) -> Self {
        Self {
            regex,
            children: HashMap::new(),
        }
    }

    /// Add a child with the given name to the regex tree under construction
    pub fn with_child(mut self, name: impl Into<String>, child: RegexTree) -> Self {
        self.children.insert(name.into(), child);
        self
    }

    /// Finish construction and create the regex tree
    pub fn build(self) -> RegexTree {
        RegexTree {
            regex: self.regex,
            children: self.children,
        }
    }
}

pub trait ToRegex {
    /// Convert to regex. Expected to panic upon failure.
    fn to_regex(self) -> Regex;
}

impl<'a> ToRegex for &'a str {
    fn to_regex(self) -> Regex {
        Regex::new(self).unwrap()
    }
}

impl ToRegex for String {
    fn to_regex(self) -> Regex {
        Regex::new(&self).unwrap()
    }
}

impl ToRegex for Regex {
    fn to_regex(self) -> Regex {
        self
    }
}
