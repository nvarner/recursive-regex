use std::iter::Zip;

use regex::{CaptureNames, Match, SubCaptureMatches};
use serde::de::value::{BorrowedStrDeserializer, Error};
use serde::de::{MapAccess, SeqAccess};
use serde::Deserializer;
use serde::{de, serde_if_integer128};

use crate::just_string::JustStrDeserializer;
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

    fn whole_match(mut self) -> &'t str {
        // capture group 0 is the whole match
        self.capture.next().unwrap().unwrap().as_str()
    }

    fn just_str(self) -> JustStrDeserializer<'t> {
        JustStrDeserializer::from_string(self.whole_match())
    }
}

impl<'de, 'r: 'de, 'c> Deserializer<'de> for SingleCaptureDeserializer<'r, 'c, 'de> {
    type Error = Error;

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

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

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let seq_access =
            SingleCaptureSeqAccess::from_regex_tree_and_captures(self.regex_tree, self.capture);
        visitor.visit_seq(seq_access)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_i8(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_i16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_i32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_i64(visitor)
    }

    serde_if_integer128! {
        fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            self.just_str().deserialize_i128(visitor)
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_u8(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_u16(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_u32(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_u64(visitor)
    }

    serde_if_integer128! {
        fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            self.just_str().deserialize_u128(visitor)
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_f32(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_f64(visitor)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_char(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_identifier(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_string(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_str(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_byte_buf(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.just_str().deserialize_bytes(visitor)
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

impl<'de, 'r: 'de, 'c> MapAccess<'de> for SingleCaptureMapAccess<'r, 'c, 'de> {
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
            Some(regex_tree) => seed.deserialize(StrDeserializer::from_regex_tree_and_str(
                regex_tree,
                value.as_str(),
            )),
            None => seed.deserialize(BorrowedStrDeserializer::new(value.as_str())),
        }
    }
}

pub struct SingleCaptureSeqAccess<'r, 'c, 't> {
    regex_tree: &'r RegexTree,
    named_captures: Zip<CaptureNames<'r>, SubCaptureMatches<'c, 't>>,
}

impl<'r, 'c, 't> SingleCaptureSeqAccess<'r, 'c, 't> {
    pub fn from_regex_tree_and_captures(
        regex_tree: &'r RegexTree,
        captures: SubCaptureMatches<'c, 't>,
    ) -> Self {
        let names = regex_tree.names();
        let named_captures = names.zip(captures);
        Self {
            regex_tree,
            named_captures,
        }
    }

    fn next(&mut self) -> Option<(Option<&'r str>, Match<'t>)> {
        self.named_captures
            .find_map(|(name, re_match)| re_match.map(|re_match| (name, re_match)))
    }
}

impl<'de, 'r: 'de, 'c> SeqAccess<'de> for SingleCaptureSeqAccess<'r, 'c, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        let next = self
            .next()
            .map(|(key, value)| (key.and_then(|key| self.regex_tree.child(key)), value));
        match next {
            Some((Some(regex_tree), value)) => seed
                .deserialize(StrDeserializer::from_regex_tree_and_str(
                    regex_tree,
                    value.as_str(),
                ))
                .map(Some),
            Some((None, value)) => seed
                .deserialize(BorrowedStrDeserializer::new(value.as_str()))
                .map(Some),
            None => Ok(None),
        }
    }
}
