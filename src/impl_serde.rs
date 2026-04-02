use core::fmt;
use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Monotonic, Timestamp};

#[cfg(not(feature = "as-float-secs"))]
impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.as_nanos().try_into().unwrap())
    }
}

#[cfg(feature = "as-float-secs")]
impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.as_secs_f64())
    }
}

impl Serialize for Monotonic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.as_nanos().try_into().unwrap())
    }
}

struct TimestampVisitor;

impl<'de> serde::de::Visitor<'de> for TimestampVisitor {
    type Value = Timestamp;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string, float, a signed or unsigned integer, or a 2-element array")
    }

    #[cfg(not(feature = "as-float-secs"))]
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.into())
    }

    #[cfg(not(feature = "as-float-secs"))]
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Timestamp::try_from(value).map_err(serde::de::Error::custom)
    }

    #[cfg(feature = "as-float-secs")]
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Timestamp::from_secs(value))
    }

    #[cfg(feature = "as-float-secs")]
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let secs = u64::try_from(value).map_err(|_| {
            serde::de::Error::custom("signed seconds must be non-negative for Timestamp")
        })?;
        Ok(Timestamp::from_secs(secs))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.into())
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.into())
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        let s: u64 = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let ns: u32 = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        Ok(Duration::new(s, ns).into())
    }
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        value
            .parse()
            .map_err(|_| serde::de::Error::custom("invalid time string"))
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        value
            .parse()
            .map_err(|_| serde::de::Error::custom("invalid time string"))
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Timestamp, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TimestampVisitor)
    }
}

struct MonotonicVisitor;

impl<'de> serde::de::Visitor<'de> for MonotonicVisitor {
    type Value = Monotonic;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string, float, a signed or unsigned integer, or a 2-element array")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.into())
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Monotonic::try_from(value).map_err(serde::de::Error::custom)
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.into())
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.into())
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        let s: u64 = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let ns: u32 = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        Ok(Duration::new(s, ns).into())
    }
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        value
            .parse()
            .map_err(|_| serde::de::Error::custom("invalid time string"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        value
            .parse()
            .map_err(|_| serde::de::Error::custom("invalid time string"))
    }
}

impl<'de> Deserialize<'de> for Monotonic {
    fn deserialize<D>(deserializer: D) -> Result<Monotonic, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(MonotonicVisitor)
    }
}

#[cfg(test)]
mod i64_roundtrip_tests {
    use serde_json::json;

    use crate::Monotonic;

    #[cfg(not(feature = "as-float-secs"))]
    #[test]
    fn timestamp_deserializes_json_integer_as_i64_nanos() {
        let v = json!(1_700_000_000_000_000_000_i64);
        let ts: crate::Timestamp = serde_json::from_value(v).unwrap();
        assert_eq!(ts.as_nanos(), 1_700_000_000_000_000_000_u128);
    }

    #[test]
    fn monotonic_deserializes_json_integer_as_i64_nanos() {
        let v = json!(42_i64);
        let m: Monotonic = serde_json::from_value(v).unwrap();
        assert_eq!(m.as_nanos(), 42_u128);
    }
}
