use crate::regex::CaptureMatches;
use serde::de;
use serde::de::value::Error;
use serde::de::SeqAccess;

use crate::single_capture::SingleCaptureDeserializer;
use crate::RegexTree;

pub struct MultiCaptureSeqAccess<'r, 't> {
    regex_tree: &'r RegexTree,
    captures: CaptureMatches<'r, 't>,
}

impl<'r, 't> MultiCaptureSeqAccess<'r, 't> {
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

impl<'de, 'r: 'de> SeqAccess<'de> for MultiCaptureSeqAccess<'r, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        self.captures
            .next()
            .map(|capture| {
                seed.deserialize(
                    SingleCaptureDeserializer::from_regex_tree_and_single_capture(
                        self.regex_tree,
                        capture.iter(),
                    ),
                )
            })
            .transpose()
    }
}
