//! # Recursive Regex
//!
//! The recursive regex algorithm is designed to be a simple, customizable
//! parser for basic data files. It matches a regular expression to text as many
//! times as possible, extracting data via capture groups. On each capture
//! group, it may recurse with a new regular expression to further parse the
//! results.
//!
//! ## Example
//! The following data file is being maintained by hand, but we want it in a
//! more structured format. We need to extract names and a list of the favorite
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
//! and call it `line`. After running this on the file, we have a list of
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
//! replace each of the second capture group entries with a list of numbers.
//!
//! We now have structured data extracted from the file. In a JSON
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

use regex::{Captures, Regex};

pub struct RegexTree {
    regex: Regex,
    capture_group_indices: HashMap<usize, Box<RegexTree>>,
    capture_group_names: HashMap<String, Box<RegexTree>>,
}

impl RegexTree {
    pub fn leaf(regex: Regex) -> Self {
        Self {
            regex,
            capture_group_indices: HashMap::new(),
            capture_group_names: HashMap::new(),
        }
    }

    pub fn captures<'r, 't: 'r>(
        &'r self,
        text: &'t str,
    ) -> impl Iterator<Item = Captures<'t>> + 'r {
        self.regex.captures_iter(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_recursive() {
        let text = "Hello, world! Goodbye, universe!";
        let regex = Regex::new("([A-Z][a-z]+), (.*?)!").unwrap();
        let regex_tree = RegexTree::leaf(regex);
        let results: Vec<Vec<_>> = regex_tree
            .captures(text)
            .map(|captures| {
                captures
                    .iter()
                    .skip(1) // capture group 0 is the entire string
                    .map(|group| group.map(|inner| inner.as_str()).unwrap_or(""))
                    .collect()
            })
            .collect();
        assert_eq!(
            vec![vec!["Hello", "world"], vec!["Goodbye", "universe"]],
            results
        );
    }
}
