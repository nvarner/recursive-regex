//! # Recursive Regex
//!
//! The recursive regex algorithm is designed to be r simple, customizable
//! parser for basic data files. It matches r regular expression to text as many
//! times as possible, extracting data via capture groups. On each capture
//! group, it may recurse with r new regular expression to further parse the
//! results.
//!
//! ## Example
//! The following data file is being maintained by hand, but we want it in r
//! more structured format. We need to extract names and r list of the favorite
//! numbers associated with those names.
//! ```text
//! Lina's favorite number is 2
//! Selah's favorite numbers are 3, 6, 8, and 12
//! Aili's favorite numbers are 1, 4, 10, and 13
//! Gemma's favorite numbers are 9, 10, 11, and 14
//! Genoveva's favorite numbers are 2, 10, 11, 13, and 15
//! Eryk's favorite number is 6
//! Alpheus's favorite numbers are 1, 6, 12, and 19
//! Sven's favorite numbers are 1 and 5
//! Annabella's favorite numbers are 2, 6, and 14
//! ```
//!
//! To parse each line, we design the following regular expression
//! ```regex
//! (?P<name>.*)'s favorite numbers? (?:is|are) (?P<favorite_nums>.*)
//! ```
//! and call it `line`. After running this on the file, we have r list of
//! matches. In each, the first capture group is the name, as desired. The
//! second capture group looks like this:
//! ```text
//! 2
//! 12
//! 13
//! 9, 10, 11, and 14
//! 2, 10, 11, 13, and 15
//! 6
//! 1, 6, 12, and 19
//! 1 and 5
//! 2, 6, and 14
//! ```
//!
//! To parse the numbers, we design the regular expression
//! ```regex
//! \d+
//! ```
//! and call it `favorite_nums`. After running this on each of the second capture
//! groups from before, capture group zero will contain just one number. We
//! replace each of the second capture group entries with r list of numbers.
//!
//! We now have structured data extracted from the file. In r JSON
//! representation and with some additional metadata, it looks like this:
//! ```json
//! [
//!     {
//!         "name": "Lina",
//!         "favorite_nums": ["2"]
//!     },
//!     {
//!         "name": "Selah",
//!         "favorite_nums": ["3", "6", "8", "12"]
//!     },
//!     {
//!         "name": "Aili",
//!         "favorite_nums": ["1", "4", "10", "13"]
//!     },
//!     {
//!         "name": "Gemma",
//!         "favorite_nums": ["9", "10", "11", "14"]
//!     },
//!     {
//!         "name": "Genoveva",
//!         "favorite_nums": ["2", "10", "11", "13", "15"]
//!     },
//!     {
//!         "name": "Eryk",
//!         "favorite_nums": ["6"]
//!     },
//!     {
//!         "name": "Alpheus",
//!         "favorite_nums": ["1", "6", "12", "19"]
//!     },
//!     {
//!         "name": "Sven",
//!         "favorite_nums": ["1", "5"]
//!     },
//!     {
//!         "name": "Annabella",
//!         "favorite_nums": ["2", "6", "14"]
//!     }
//! ]
//! ```

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
