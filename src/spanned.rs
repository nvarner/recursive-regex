// Inspired by toml-rs's Spanned
// https://github.com/toml-rs/toml/blob/main/crates/toml/src/spanned.rs

use std::fmt::Debug;
use std::marker::PhantomData;
use std::{fmt, mem};

use serde::de::value::BorrowedStrDeserializer;
use serde::de::IntoDeserializer;
use serde::{de, ser, Deserializer};

pub(crate) const SPANNED_NAME: &str = "  __SPANNED";
pub(crate) const SPANNED_BEGIN: &str = "  __SPANNED_BEGIN";
pub(crate) const SPANNED_END: &str = "  __SPANNED_END";
pub(crate) const SPANNED_VALUE: &str = "  __SPANNED_VALUE";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Spanned<T> {
    begin: usize,
    end: usize,
    value: T,
}

impl<T> Spanned<T> {
    pub fn into_inner(self) -> T {
        self.value
    }

    pub fn new_raw(value: T, begin: usize, end: usize) -> Self {
        Self { begin, end, value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn begin(&self) -> usize {
        self.begin
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

impl<'de, T: de::Deserialize<'de>> de::Deserialize<'de> for Spanned<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = SpannedVisitor(PhantomData);
        deserializer.deserialize_struct(
            SPANNED_NAME,
            &[SPANNED_BEGIN, SPANNED_END, SPANNED_VALUE],
            visitor,
        )
    }
}

struct SpannedVisitor<T>(PhantomData<T>);

impl<'de, T: de::Deserialize<'de>> de::Visitor<'de> for SpannedVisitor<T> {
    type Value = Spanned<T>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a spanned regex match")
    }

    fn visit_map<A>(self, mut visitor: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        fn check_key<'de, A>(visitor: &mut A, expected: &str, name: &str)
        where
            A: de::MapAccess<'de>,
        {
            let key_valid = match visitor.next_key::<&str>() {
                Ok(Some(key)) => std::ptr::eq(key, expected),
                _ => false,
            };
            if !key_valid {
                panic!("`Spanned` {name} key not found");
            }
        }

        check_key(&mut visitor, SPANNED_BEGIN, "begin");
        let begin: usize = visitor.next_value()?;

        check_key(&mut visitor, SPANNED_END, "end");
        let end: usize = visitor.next_value()?;

        check_key(&mut visitor, SPANNED_VALUE, "value");
        let value: T = visitor.next_value()?;

        Ok(Spanned { begin, end, value })
    }
}

pub(crate) enum SpannedDeserializer<T, E> {
    Start(PhantomData<E>, T, usize, usize),
    End(T, usize),
    Value(T),
    None,
}

impl<T, E> SpannedDeserializer<T, E> {
    pub fn new(start: usize, end: usize, value: T) -> Self {
        Self::Start(PhantomData, value, end, start)
    }
}

impl<'de, T, E> de::MapAccess<'de> for SpannedDeserializer<T, E>
where
    T: Deserializer<'de, Error = E>,
    E: de::Error,
{
    type Error = E;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        let key = match self {
            Self::Start(..) => Some(SPANNED_BEGIN),
            Self::End(..) => Some(SPANNED_END),
            Self::Value(..) => Some(SPANNED_VALUE),
            Self::None => None,
        };

        key.map(BorrowedStrDeserializer::new)
            .map(|key_de| seed.deserialize(key_de))
            .transpose()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let data = mem::replace(self, Self::None);

        let (result, data) = match data {
            Self::Start(_, value, end, start) => (
                seed.deserialize(start.into_deserializer()),
                Self::End(value, end),
            ),
            Self::End(value, end) => (
                seed.deserialize(end.into_deserializer()),
                Self::Value(value),
            ),
            Self::Value(value) => (seed.deserialize(value), Self::None),
            Self::None => (
                Err(Self::Error::custom("no more values for `Spanned`")),
                Self::None,
            ),
        };
        let _ = mem::replace(self, data);

        result
    }
}

impl<T: ser::Serialize> ser::Serialize for Spanned<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}
