use std::iter::Zip;

use regex::{CaptureNames, Match, SubCaptureMatches};
use serde::de::value::{BorrowedStrDeserializer, Error};
use serde::de::{self, MapAccess};
use serde::Deserializer;

use crate::string::StrDeserializer;
use crate::RegexTree;

pub struct SingleCaptureDeserializer<'r, 'c, 't> {
    regex_tree: &'r RegexTree,
    capture: SubCaptureMatches<'c, 't>,
}

impl<'r, 'c, 't> SingleCaptureDeserializer<'r, 'c, 't> {
    pub fn from_regex_tree_and_single_capture(
        regex_tree: &'r RegexTree,
        capture: SubCaptureMatches<'c, 't>,
    ) -> Self {
        Self {
            regex_tree,
            capture,
        }
    }
}

impl<'de, 'r, 'c, 't> Deserializer<'de> for SingleCaptureDeserializer<'r, 'c, 't> {
    type Error = Error;

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let deserializer =
            SingleCaptureMapAccess::from_regex_tree_and_captures(self.regex_tree, self.capture);
        visitor.visit_map(deserializer)
    }
}

pub struct SingleCaptureMapAccess<'r, 'c, 't> {
    regex_tree: &'r RegexTree,
    named_captures: Zip<CaptureNames<'r>, SubCaptureMatches<'c, 't>>,
    /// Stores the last returned key with its associated value
    last_key_value: Option<(&'r str, Match<'t>)>,
}

impl<'r, 'c, 't> SingleCaptureMapAccess<'r, 'c, 't> {
    pub fn from_regex_tree_and_captures(
        regex_tree: &'r RegexTree,
        captures: SubCaptureMatches<'c, 't>,
    ) -> Self {
        let names = regex_tree.names();
        let named_captures = names.zip(captures);
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

impl<'de, 'r, 'c, 't> MapAccess<'de> for SingleCaptureMapAccess<'r, 'c, 't> {
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
