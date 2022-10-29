#![doc = include_str!("../README.md")]

use std::collections::HashMap;

use regex::{CaptureMatches, CaptureNames, Captures, Regex};
use serde::de::value::Error;
use serde::Deserialize;
use string::StrDeserializer;

mod just_string;
mod multi_capture;
mod single_capture;
mod string;

pub fn from_regex_tree_and_str<'t, 'r: 't, T: Deserialize<'t>>(
    regex_tree: &'r RegexTree,
    text: &'t str,
) -> Result<T, Error> {
    let deserializer = StrDeserializer::from_regex_tree_and_str(regex_tree, text);
    T::deserialize(deserializer)
}

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
