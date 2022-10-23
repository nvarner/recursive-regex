use std::fmt::Display;
use std::iter::Zip;
use std::str::FromStr;

use regex::{CaptureNames, Match, SubCaptureMatches};
use serde::de::value::{BorrowedStrDeserializer, Error};
use serde::de::{Error as ErrorTrait, MapAccess, SeqAccess};
use serde::Deserializer;
use serde::{de, serde_if_integer128};

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

    fn parse_bool(&mut self) -> Result<bool, Error> {
        match self.whole_match().to_lowercase().as_str() {
            "false" | "f" | "no" | "n" | "0" => Ok(false),
            "true" | "t" | "yes" | "y" | "1" => Ok(true),
            whole_match => Err(Error::custom(format!(
                "got {whole_match:?} but expecting a bool"
            ))),
        }
    }

    fn parse<T: FromStr>(&mut self) -> Result<T, Error>
    where
        T::Err: Display,
    {
        self.whole_match()
            .parse::<T>()
            .map_err(|err| Error::custom(format!("parsing error: {err}")))
    }

    fn parse_char(&mut self) -> Result<char, Error> {
        let whole_match = self.whole_match();
        let mut chars = whole_match.chars();
        let first_char = chars.next();
        match first_char {
            Some(first_char) if chars.next().is_none() => Ok(first_char),
            _ => Err(Error::custom(format!(
                "got {whole_match:?} but expecting a single char",
            ))),
        }
    }

    /// Only valid if called before `self.capture` is modified, so that capture
    /// group 0 is still the first entry in the iterator
    fn whole_match(&mut self) -> &'t str {
        self.capture.next().unwrap().unwrap().as_str()
    }
}

impl<'de, 'r, 'c> Deserializer<'de> for SingleCaptureDeserializer<'r, 'c, 'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
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
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
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

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse()?)
    }

    serde_if_integer128! {
        fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            visitor.visit_i128(self.parse()?)
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse()?)
    }

    serde_if_integer128! {
        fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            visitor.visit_u128(self.parse()?)
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.parse()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.parse()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_char(self.parse_char()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.whole_match())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.whole_match().as_bytes())
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
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
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

impl<'de, 'r, 'c> MapAccess<'de> for SingleCaptureMapAccess<'r, 'c, 'de> {
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

impl<'de, 'r, 'c> SeqAccess<'de> for SingleCaptureSeqAccess<'r, 'c, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        let next = self
            .next()
            .map(|(key, value)| (key.map(|key| self.regex_tree.child(key)).flatten(), value));
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
