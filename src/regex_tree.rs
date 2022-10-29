use std::collections::HashMap;

use crate::regex::{CaptureMatches, CaptureNames, Captures, Regex};

/// A regex tree is a recursive regular expression. Once the root regex of a
/// tree matches a string, if any of its named capture groups match the name of
/// one of its children, it recurses, and the child regex tree will be run on
/// the capture group.
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
pub struct RegexTree {
    regex: Regex,
    children: HashMap<String, RegexTree>,
}

impl RegexTree {
    pub fn root(regex: Regex) -> Builder {
        Builder::new(regex)
    }

    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
        self.regex.captures(text)
    }

    pub fn captures_iter<'r, 't>(&'r self, text: &'t str) -> CaptureMatches<'r, 't> {
        self.regex.captures_iter(text)
    }

    pub fn names(&self) -> CaptureNames {
        self.regex.capture_names()
    }

    pub fn child(&self, name: &str) -> Option<&RegexTree> {
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

    pub fn with_child(mut self, name: impl Into<String>, child: RegexTree) -> Self {
        self.children.insert(name.into(), child);
        self
    }

    pub fn build(self) -> RegexTree {
        RegexTree {
            regex: self.regex,
            children: self.children,
        }
    }
}
