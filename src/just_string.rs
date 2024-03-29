use regex::Match;
use std::fmt::Display;
use std::str::FromStr;

use crate::spanned::{
    SpannedDeserializer, SPANNED_BEGIN, SPANNED_END, SPANNED_NAME, SPANNED_VALUE,
};
use serde::de::value::Error;
use serde::de::Error as ErrorTrait;
use serde::{de, serde_if_integer128};

/// Deserialize just a string, in the sense that regular expressions are no
/// longer needed to complete parsing. This should be invoked near the end of
/// (nearly) all deserialization to take the final capture groups and turn them
/// into numbers, `bool`s, `&str`s, or whatever other type was requested.
pub struct JustStrDeserializer<'t> {
    text: &'t str,
    /// Byte offset of the start of `text` within the originally parsed string
    start: usize,
}

impl<'t> JustStrDeserializer<'t> {
    pub fn new(text: &'t str, start: usize) -> Self {
        Self { text, start }
    }

    /// Create a new deserializer from a `Match`
    pub fn from_match(re_match: Match<'t>, start: usize) -> Self {
        Self {
            text: re_match.as_str(),
            start,
        }
    }

    fn parse_bool(self) -> Result<bool, Error> {
        match self.text.to_lowercase().as_str() {
            "false" | "f" | "no" | "n" | "0" => Ok(false),
            "true" | "t" | "yes" | "y" | "1" => Ok(true),
            whole_match => Err(Error::custom(format!(
                "got {whole_match:?} but expecting a bool"
            ))),
        }
    }

    fn parse_char(self) -> Result<char, Error> {
        let mut chars = self.text.chars();
        let first_char = chars.next();
        match first_char {
            Some(first_char) if chars.next().is_none() => Ok(first_char),
            _ => Err(Error::custom(format!(
                "got {} but expecting a single char",
                self.text
            ))),
        }
    }

    fn parse<T: FromStr>(self) -> Result<T, Error>
    where
        T::Err: Display,
    {
        self.text
            .parse::<T>()
            .map_err(|err| Error::custom(format!("parsing error: {err}")))
    }
}

impl<'de> de::Deserializer<'de> for JustStrDeserializer<'de> {
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

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if name == SPANNED_NAME && fields == [SPANNED_BEGIN, SPANNED_END, SPANNED_VALUE] {
            let end = self.start + self.text.len();
            visitor.visit_map(SpannedDeserializer::new(self.start, end, self))
        } else {
            self.deserialize_map(visitor)
        }
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
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

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
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
        visitor.visit_borrowed_str(self.text)
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
        visitor.visit_borrowed_bytes(self.text.as_bytes())
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
}

#[cfg(test)]
mod test {
    use super::JustStrDeserializer;
    use serde::Deserialize;

    #[test]
    fn bool_success() {
        let true_strs = ["true", "tRuE", "T", "Yes", "y", "1"];
        for x in true_strs {
            let deserializer = JustStrDeserializer::new(x, 0);
            assert_eq!(deserializer.parse_bool(), Ok(true));
        }

        let false_strs = ["false", "FaLsE", "F", "No", "n", "0"];
        for x in false_strs {
            let deserializer = JustStrDeserializer::new(x, 0);
            assert_eq!(deserializer.parse_bool(), Ok(false));
        }
    }

    #[test]
    fn bool_fail() {
        let fail_strs = ["frue", "talse", "2", "sure", "maybe", "tr", "fal"];
        for x in fail_strs {
            let deserializer = JustStrDeserializer::new(x, 0);
            assert!(deserializer.parse_bool().is_err());
        }
    }

    #[test]
    fn char_success() {
        let strs_output = [("f", 'f'), (" ", ' '), ("H", 'H')];
        for (x, expected) in strs_output {
            let deserializer = JustStrDeserializer::new(x, 0);
            assert_eq!(deserializer.parse_char(), Ok(expected));
        }
    }

    #[test]
    fn char_fail() {
        let fail_strs = ["false", "Hello", ""];
        for x in fail_strs {
            let deserializer = JustStrDeserializer::new(x, 0);
            assert!(deserializer.parse_char().is_err());
        }
    }

    #[test]
    fn int_success() {
        let strs_output = [("123", 123), ("-432", -432)];
        for (x, expected) in strs_output {
            let deserializer = JustStrDeserializer::new(x, 0);
            assert_eq!(deserializer.parse(), Ok(expected));
        }
    }

    #[test]
    fn int_fail() {
        let fail_strs = ["123abc", "12.6"];
        for x in fail_strs {
            let deserializer = JustStrDeserializer::new(x, 0);
            assert!(deserializer.parse::<i32>().is_err());
        }
    }

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct Data<T>(T);

    macro_rules! test_type {
        ($t:ty, $name:ident, $data:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let data_str = $data;
                let data_struct = Data::<$t>::deserialize(JustStrDeserializer::new(data_str, 0));
                assert_eq!(data_struct, Ok(Data($expected)))
            }
        };
    }

    test_type!(bool, test_bool, "true", true);
    test_type!(i32, test_i32, "-0324", -324);
    test_type!(u32, test_u32, "52", 52);
    test_type!(f32, test_f32, "4235.2", 4235.2);
    test_type!(char, test_char, "d", 'd');
    test_type!(
        String,
        test_string,
        "hello world",
        "hello world".to_string()
    );
    test_type!(&str, test_str, "hello world", "hello world");
    test_type!(
        Option<&str>,
        test_option,
        "hello world",
        Some("hello world")
    );
    test_type!((), test_unit, "yf78iy f37y", ());
}
