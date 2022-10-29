#![doc = include_str!("../README.md")]

use serde::de::value::Error;
use serde::Deserialize;
use string::StrDeserializer;

mod just_string;
mod multi_capture;
mod regex_tree;
mod single_capture;
mod string;

pub use regex;

pub use crate::regex_tree::RegexTree;
pub use just_string::JustStrDeserializer;

/// Primary entry point to the library.
///
/// Takes [`&RegexTree`](crate::RegexTree) and `&str`, then deserializes the
/// text with the given regex tree.
///
/// ## Example
/// ```
/// # use recursive_regex::{RegexTree, from_regex_tree_and_str};
/// let text = "1 2 456";
/// let regex_tree = RegexTree::leaf(r"\d+");
/// let deserialized: Vec<u32> = from_regex_tree_and_str(&regex_tree, &text).unwrap();
/// assert_eq!(deserialized, vec![1, 2, 456]);
/// ```
pub fn from_regex_tree_and_str<'t, 'r: 't, T: Deserialize<'t>>(
    regex_tree: &'r RegexTree,
    text: &'t str,
) -> Result<T, Error> {
    let deserializer = StrDeserializer::from_regex_tree_and_str(regex_tree, text);
    T::deserialize(deserializer)
}
