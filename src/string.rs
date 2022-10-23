use serde::de;
use serde::de::value::Error;

use crate::multi_capture::MultiCaptureDeserializer;
use crate::{CapturesMapAccess, RegexTree};

pub struct StrDeserializer<'r, 't> {
    regex_tree: &'r RegexTree,
    text: &'t str,
}

impl<'r, 't> StrDeserializer<'r, 't> {
    pub fn from_regex_tree_and_str(regex_tree: &'r RegexTree, text: &'r str) -> Self {
        Self { regex_tree, text }
    }
}

impl<'de, 'r, 't> de::Deserializer<'de> for &mut StrDeserializer<'r, 't> {
    type Error = Error;

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
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
        // Deserialize from a single capture
        let names = self.regex_tree.names();
        let captures = self
            .regex_tree
            .captures(self.text)
            .ok_or_else(|| <Error as de::Error>::custom("regular expression does not match"))?;
        let named_captures = names.zip(captures.iter());
        let deserializer = CapturesMapAccess::new(self.regex_tree, named_captures);
        visitor.visit_map(deserializer)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // Deserialize from many captures
        let captures_iter = self.regex_tree.captures_iter(self.text);
        let deserializer =
            MultiCaptureDeserializer::from_regex_tree_and_captures(self.regex_tree, captures_iter);
        visitor.visit_seq(deserializer)
    }
}
