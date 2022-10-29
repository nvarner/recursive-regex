use std::collections::HashMap;

use crate::regex::{CaptureMatches, CaptureNames, Captures, Regex};

pub struct RegexTree {
    regex: Regex,
    children: HashMap<String, RegexTree>,
}

impl RegexTree {
    pub fn new(regex: Regex, children: HashMap<String, RegexTree>) -> Self {
        Self { regex, children }
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
