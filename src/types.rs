//! Small value types shared by many resources.

use std::{borrow::Borrow, fmt, ops::Deref};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self as serde_de},
};

/// Declares a string-backed enum that survives new values added by the API.
///
/// Unknown strings deserialize into `Other(String)` instead of failing, so a new
/// subscriber status or webhook event never breaks decoding of an otherwise valid response.
/// Known values deserialize without allocating.
macro_rules! string_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $( $(#[$vmeta:meta])* $variant:ident => $value:literal, )+
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $( $(#[$vmeta])* $variant, )+
            /// A value this version of the SDK does not know about yet.
            Other(String),
        }

        impl $name {
            /// The wire representation of this value.
            #[must_use]
            pub fn as_str(&self) -> &str {
                match self {
                    $( Self::$variant => $value, )+
                    Self::Other(value) => value,
                }
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                match value {
                    $( $value => Self::$variant, )+
                    other => Self::Other(other.to_owned()),
                }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct Visitor;

                impl ::serde::de::Visitor<'_> for Visitor {
                    type Value = $name;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(concat!("a ", stringify!($name), " string"))
                    }

                    fn visit_str<E: ::serde::de::Error>(self, value: &str) -> Result<$name, E> {
                        Ok($name::from(value))
                    }
                }

                deserializer.deserialize_str(Visitor)
            }
        }
    };
}

pub(crate) use string_enum;

/// A MailerLite resource identifier.
///
/// The API documents ids as strings (`"31986843064993537"`) but some endpoints return them
/// as JSON numbers. They can exceed `2^53`, so they are kept as text to stay lossless.
/// Both forms deserialize into `Id`; it always serializes as a string.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Id(String);

impl Id {
    /// Wraps an id string.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// The id as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Unwraps into the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl serde_de::Visitor<'_> for Visitor {
            type Value = Id;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an id as a string or integer")
            }

            fn visit_str<E: serde_de::Error>(self, value: &str) -> Result<Id, E> {
                Ok(Id(value.to_owned()))
            }

            fn visit_string<E: serde_de::Error>(self, value: String) -> Result<Id, E> {
                Ok(Id(value))
            }

            fn visit_u64<E: serde_de::Error>(self, value: u64) -> Result<Id, E> {
                Ok(Id(value.to_string()))
            }

            fn visit_i64<E: serde_de::Error>(self, value: i64) -> Result<Id, E> {
                Ok(Id(value.to_string()))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl Deref for Id {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Id {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for Id {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for Id {
    fn from(id: &str) -> Self {
        Self(id.to_owned())
    }
}

impl From<u64> for Id {
    fn from(id: u64) -> Self {
        Self(id.to_string())
    }
}

impl PartialEq<str> for Id {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for Id {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

/// A timestamp exactly as MailerLite sent it.
///
/// The API mixes `2022-02-18 08:09:58` (UTC, no offset) and ISO-8601
/// (`2022-02-18T08:09:58.000000Z`) depending on the endpoint, so the SDK keeps the raw text
/// and leaves parsing to the date library of your choice.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(String);

impl Timestamp {
    /// Wraps a timestamp string.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// The raw timestamp text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Timestamp {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A percentage as MailerLite reports it: both as a number and preformatted.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Rate {
    /// Numeric value, e.g. `0.25` or `25.0` depending on the endpoint.
    #[serde(default, deserialize_with = "de::number")]
    pub float: f64,
    /// Display value, e.g. `"25%"`.
    #[serde(default)]
    pub string: String,
}

/// Sort order for list endpoints that accept `sort=field` / `sort=-field`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sort<F> {
    /// Field to sort by.
    pub field: F,
    /// Sort descending instead of ascending.
    pub descending: bool,
}

impl<F> Sort<F> {
    /// Ascending by `field`.
    pub fn asc(field: F) -> Self {
        Self {
            field,
            descending: false,
        }
    }

    /// Descending by `field`.
    pub fn desc(field: F) -> Self {
        Self {
            field,
            descending: true,
        }
    }
}

impl<F: AsRef<str>> Serialize for Sort<F> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.descending {
            serializer.collect_str(&format_args!("-{}", self.field.as_ref()))
        } else {
            serializer.serialize_str(self.field.as_ref())
        }
    }
}

/// A custom field value: text, number, or `null` to clear it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum FieldValue {
    /// `null`.
    Null,
    /// A numeric value.
    Number(serde_json::Number),
    /// A text (or date) value.
    Text(String),
    /// Anything else the API may return.
    Other(serde_json::Value),
}

impl FieldValue {
    /// The value as text, if it is text.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    /// The value as a float, if it is numeric.
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(number) => number.as_f64(),
            _ => None,
        }
    }

    /// `true` for `null`.
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

impl From<&str> for FieldValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<String> for FieldValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<i64> for FieldValue {
    fn from(value: i64) -> Self {
        Self::Number(value.into())
    }
}

impl From<i32> for FieldValue {
    fn from(value: i32) -> Self {
        Self::Number(value.into())
    }
}

impl From<u64> for FieldValue {
    fn from(value: u64) -> Self {
        Self::Number(value.into())
    }
}

impl From<f64> for FieldValue {
    /// Non-finite floats (NaN, ±∞) are not representable in JSON and become `null`.
    fn from(value: f64) -> Self {
        serde_json::Number::from_f64(value).map_or(Self::Null, Self::Number)
    }
}

impl<T: Into<FieldValue>> From<Option<T>> for FieldValue {
    fn from(value: Option<T>) -> Self {
        value.map_or(Self::Null, Into::into)
    }
}

/// `{"name": "..."}` request body shared by several rename endpoints.
#[derive(Serialize)]
pub(crate) struct NameBody<'a> {
    pub(crate) name: &'a str,
}

/// `{"total": n}`: the response to `?limit=0` count requests.
#[derive(Deserialize)]
pub(crate) struct Total {
    pub(crate) total: u64,
}

/// `?limit=0`, used to ask list endpoints for a count only.
#[derive(Serialize)]
pub(crate) struct CountQuery {
    pub(crate) limit: u8,
}

pub(crate) const COUNT_QUERY: CountQuery = CountQuery { limit: 0 };

/// Lenient deserializers for fields the API is inconsistent about.
pub(crate) mod de {
    use std::fmt;

    use serde::{
        Deserialize, Deserializer,
        de::{self, IgnoredAny, MapAccess, SeqAccess, Visitor},
    };
    use serde_json::{Map, Value};

    /// A number that may arrive as a JSON number, a numeric string (`"123.4"`) or `null`
    /// (decoded as `0.0`).
    pub(crate) fn number<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
        Ok(optional_number(deserializer)?.unwrap_or_default())
    }

    /// Like [`number`] but keeps `null` / empty strings as `None`.
    pub(crate) fn optional_number<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<f64>, D::Error> {
        struct NumberVisitor;

        impl<'de> Visitor<'de> for NumberVisitor {
            type Value = Option<f64>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a number or numeric string")
            }

            fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
                Ok(Some(value))
            }

            #[allow(clippy::cast_precision_loss)]
            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(Some(value as f64))
            }

            #[allow(clippy::cast_precision_loss)]
            fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
                Ok(Some(value as f64))
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                let value = value.trim().trim_end_matches('%');
                if value.is_empty() {
                    return Ok(None);
                }
                value.parse().map(Some).map_err(E::custom)
            }

            fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
                d.deserialize_any(self)
            }
        }

        deserializer.deserialize_any(NumberVisitor)
    }

    /// A JSON object that PHP serializes as `[]` when empty. `null` becomes an empty map.
    pub(crate) fn object<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Map<String, Value>, D::Error> {
        struct ObjectVisitor;

        impl<'de> Visitor<'de> for ObjectVisitor {
            type Value = Map<String, Value>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an object, an empty array or null")
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                Map::deserialize(de::value::MapAccessDeserializer::new(map))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                while seq.next_element::<IgnoredAny>()?.is_some() {}
                Ok(Map::new())
            }

            fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(Map::new())
            }

            fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(Map::new())
            }

            fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
                d.deserialize_any(self)
            }
        }

        deserializer.deserialize_any(ObjectVisitor)
    }

    /// `null` decodes as `T::default()` (serde's `default` only covers a *missing* key).
    pub(crate) fn null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de> + Default,
    {
        Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[allow(unreachable_pub)]
    mod colour {
        string_enum! {
            /// Test enum.
            pub enum Colour {
                /// Red.
                Red => "red",
                /// Dark blue.
                DarkBlue => "dark_blue",
            }
        }
    }
    use colour::Colour;

    #[test]
    fn id_accepts_strings_and_numbers() {
        let ids: Vec<Id> =
            serde_json::from_value(json!(["31986843064993537", 46_919_150_796_276_752_u64, 7]))
                .unwrap();

        assert_eq!(
            ids,
            ["31986843064993537", "46919150796276752", "7"].map(Id::from)
        );
        assert_eq!(
            serde_json::to_value(&ids[1]).unwrap(),
            json!("46919150796276752")
        );
    }

    #[test]
    fn string_enum_round_trips_and_keeps_unknown_values() {
        let values: Vec<Colour> =
            serde_json::from_value(json!(["red", "dark_blue", "neon"])).unwrap();

        assert_eq!(
            values,
            [Colour::Red, Colour::DarkBlue, Colour::Other("neon".into())]
        );
        assert_eq!(
            serde_json::to_value(&values).unwrap(),
            json!(["red", "dark_blue", "neon"])
        );
        assert_eq!(Colour::DarkBlue.to_string(), "dark_blue");
    }

    #[test]
    fn sort_serializes_with_direction_prefix() {
        assert_eq!(
            serde_json::to_value(Sort::asc(Colour::Red)).unwrap(),
            json!("red")
        );
        assert_eq!(
            serde_json::to_value(Sort::desc(Colour::DarkBlue)).unwrap(),
            json!("-dark_blue")
        );
    }

    #[test]
    fn rate_and_lenient_numbers() {
        #[derive(Deserialize)]
        struct Money {
            #[serde(deserialize_with = "de::number")]
            a: f64,
            #[serde(deserialize_with = "de::number")]
            b: f64,
            #[serde(deserialize_with = "de::number")]
            c: f64,
            #[serde(default, deserialize_with = "de::optional_number")]
            d: Option<f64>,
        }

        let money: Money =
            serde_json::from_value(json!({"a": "123.4", "b": 7, "c": null, "d": ""})).unwrap();
        assert!((money.a - 123.4).abs() < f64::EPSILON);
        assert!((money.b - 7.0).abs() < f64::EPSILON);
        assert!(money.c.abs() < f64::EPSILON);
        assert_eq!(money.d, None);

        let rate: Rate = serde_json::from_value(json!({"float": 0.25, "string": "25%"})).unwrap();
        assert_eq!(rate.string, "25%");
    }

    #[test]
    fn object_accepts_php_empty_array() {
        #[derive(Deserialize)]
        struct Holder {
            #[serde(deserialize_with = "de::object")]
            properties: serde_json::Map<String, serde_json::Value>,
        }

        let empty: Holder = serde_json::from_value(json!({"properties": []})).unwrap();
        assert!(empty.properties.is_empty());

        let full: Holder = serde_json::from_value(json!({"properties": {"a": 1}})).unwrap();
        assert_eq!(full.properties["a"], 1);
    }

    #[test]
    fn field_value_conversions() {
        let values: Vec<FieldValue> =
            serde_json::from_value(json!(["Ann", 3, null, true])).unwrap();

        assert_eq!(values[0].as_str(), Some("Ann"));
        assert_eq!(values[1].as_f64(), Some(3.0));
        assert!(values[2].is_null());
        assert_eq!(values[3], FieldValue::Other(json!(true)));
        assert_eq!(FieldValue::from(f64::NAN), FieldValue::Null);
        assert_eq!(FieldValue::from(None::<&str>), FieldValue::Null);
    }
}
