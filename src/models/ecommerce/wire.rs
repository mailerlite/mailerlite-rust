//! Internal e-commerce identifier, decimal, and scalar decoding.
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serializer};
use std::str::FromStr;
pub(crate) fn id<'de, D: Deserializer<'de>, T: From<String>>(d: D) -> Result<T, D::Error> {
    let value = serde_json::Value::deserialize(d)?;
    let text = match value {
        serde_json::Value::String(s) => s,
        serde_json::Value::Number(n) => {
            let s = n.to_string();
            if !s.bytes().all(|b| b.is_ascii_digit()) {
                return Err(serde::de::Error::custom(
                    "identifier must be a string or unsigned integer",
                ));
            }
            s
        }
        _ => {
            return Err(serde::de::Error::custom(
                "identifier must be a string or unsigned integer",
            ));
        }
    };
    Ok(T::from(text))
}
pub(crate) fn decimal<'de, D: Deserializer<'de>>(d: D) -> Result<Decimal, D::Error> {
    let v = serde_json::Value::deserialize(d)?;
    let s = match v {
        serde_json::Value::String(s) => s,
        serde_json::Value::Number(n) => n.to_string(),
        _ => {
            return Err(serde::de::Error::custom(
                "expected decimal number or string",
            ));
        }
    };
    // `from_scientific` parses its mantissa using the rounding parser. Check it
    // independently with the exact parser before allowing exponent handling.
    if let Some((mantissa, _)) = s.split_once(['e', 'E']) {
        Decimal::from_str_exact(mantissa).map_err(serde::de::Error::custom)?;
        Decimal::from_scientific(&s).map_err(serde::de::Error::custom)
    } else {
        Decimal::from_str_exact(&s).map_err(serde::de::Error::custom)
    }
}
pub(crate) fn optional_decimal<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<Decimal>, D::Error> {
    let v = Option::<serde_json::Value>::deserialize(d)?;
    v.map(|v| decimal(v).map_err(serde::de::Error::custom))
        .transpose()
}
pub(crate) fn optional_id<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    Option::<serde_json::Value>::deserialize(d)?
        .map(|value| id(value).map_err(serde::de::Error::custom))
        .transpose()
}
pub(crate) fn unsigned<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
    let value: String = id(d)?;
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(serde::de::Error::custom(
            "expected unsigned integer or digit string",
        ));
    }
    value.parse().map_err(serde::de::Error::custom)
}
pub(crate) fn optional_unsigned<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u64>, D::Error> {
    Option::<serde_json::Value>::deserialize(d)?
        .map(|value| unsigned(value).map_err(serde::de::Error::custom))
        .transpose()
}
pub(crate) fn optional_bool<'de, D: Deserializer<'de>>(d: D) -> Result<Option<bool>, D::Error> {
    let value = Option::<serde_json::Value>::deserialize(d)?;
    value
        .map(|value| match value {
            serde_json::Value::Bool(value) => Ok(value),
            serde_json::Value::Number(value) if value.as_u64() == Some(0) => Ok(false),
            serde_json::Value::Number(value) if value.as_u64() == Some(1) => Ok(true),
            serde_json::Value::String(value) if value == "0" => Ok(false),
            serde_json::Value::String(value) if value == "1" => Ok(true),
            _ => Err(serde::de::Error::custom(
                "expected boolean or Laravel boolean representation (0 or 1)",
            )),
        })
        .transpose()
}
pub(crate) fn serialize_decimal<S: Serializer>(v: &Decimal, s: S) -> Result<S::Ok, S::Error> {
    let number = serde_json::Number::from_str(&v.to_string()).map_err(serde::ser::Error::custom)?;
    serde::Serialize::serialize(&number, s)
}
pub(crate) fn serialize_optional_decimal<S: Serializer>(
    v: &Option<Decimal>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match v {
        Some(v) => serialize_decimal(v, s),
        None => s.serialize_none(),
    }
}
