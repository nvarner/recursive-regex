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
//! (.*)'s favorite numbers? (?:is|are) (.*)
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
//! To parse the
//! numbers, we design the regular expression
//! ```regex
//! \d+
//! ```
//! and call it `number`. After running this on each of the second capture
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
//!     }
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
use std::iter::Zip;

use regex::{CaptureMatches, CaptureNames, Captures, Match, Regex, SubCaptureMatches};
use serde::de;
use serde::de::value::{BorrowedStrDeserializer, Error};
use serde::de::MapAccess;
use string::StrDeserializer;

mod multi_capture;
mod single_capture;
mod string;

pub struct RegexTree {
    regex: Regex,
    children: HashMap<String, RegexTree>,
}

impl RegexTree {
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

struct CapturesMapAccess<'r, 'c, 't> {
    regex_tree: &'r RegexTree,
    named_captures: Zip<CaptureNames<'r>, SubCaptureMatches<'c, 't>>,
    /// Stores the last returned key with its associated value
    last_key_value: Option<(&'r str, Match<'t>)>,
}

impl<'r, 'c, 't> CapturesMapAccess<'r, 'c, 't> {
    pub fn new(
        regex_tree: &'r RegexTree,
        named_captures: Zip<CaptureNames<'r>, SubCaptureMatches<'c, 't>>,
    ) -> Self {
        Self {
            regex_tree,
            named_captures,
            last_key_value: None,
        }
    }

    fn last(&mut self) -> Option<(&'r str, Match<'t>)> {
        self.last_key_value.take()
    }

    fn next_key(&mut self) -> Option<&'r str> {
        self.next().map(|(key, _value)| key)
    }

    fn next(&mut self) -> Option<(&'r str, Match<'t>)> {
        let next = self
            .named_captures
            .find_map(|(name, re_match)| name.zip(re_match));
        self.last_key_value = next;
        next
    }
}

impl<'de, 'r, 'c, 't> MapAccess<'de> for CapturesMapAccess<'r, 'c, 't> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        self.next_key()
            .map(BorrowedStrDeserializer::new)
            .map(|deserializer| seed.deserialize(deserializer))
            .transpose()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let (key, value) = self
            .last()
            .expect("invalid calling order; cannot get next value if there was no next key");
        match self.regex_tree.child(key) {
            Some(regex_tree) => seed.deserialize(&mut StrDeserializer::from_regex_tree_and_str(
                regex_tree,
                value.as_str(),
            )),
            None => seed.deserialize(BorrowedStrDeserializer::new(value.as_str())),
        }
    }
}
