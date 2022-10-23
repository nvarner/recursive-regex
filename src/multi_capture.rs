use regex::CaptureMatches;
use serde::de;
use serde::de::value::Error;
use serde::de::SeqAccess;
use serde::Deserializer;

use crate::single_capture::SingleCaptureDeserializer;
use crate::RegexTree;

pub struct MultiCaptureDeserializer<'r, 't> {
    regex_tree: &'r RegexTree,
    captures: CaptureMatches<'r, 't>,
}

impl<'r, 't> MultiCaptureDeserializer<'r, 't> {
    pub fn from_regex_tree_and_captures(
        regex_tree: &'r RegexTree,
        captures: CaptureMatches<'r, 't>,
    ) -> Self {
        Self {
            regex_tree,
            captures,
        }
    }
}

impl<'de, 'r, 't> Deserializer<'de> for MultiCaptureDeserializer<'r, 't> {
    type Error = Error;

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }
}

impl<'de, 'r, 't> SeqAccess<'de> for MultiCaptureDeserializer<'r, 't> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        self.captures
            .next()
            .map(|capture| {
                SingleCaptureDeserializer::from_regex_tree_and_single_capture(
                    self.regex_tree,
                    capture.iter(),
                )
            })
            .map(|deserializer| seed.deserialize(deserializer))
            .transpose()
    }
}
