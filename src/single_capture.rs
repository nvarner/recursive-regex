use regex::SubCaptureMatches;
use serde::de;
use serde::de::value::Error;
use serde::Deserializer;

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

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}
