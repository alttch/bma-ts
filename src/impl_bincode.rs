use bincode::{Decode, Encode};

use crate::{Monotonic, Timestamp};

//
// Timestamp
//
#[cfg(not(feature = "as-float-secs"))]
impl Encode for Timestamp {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        let nanos: u64 = self.as_nanos().try_into().unwrap();
        nanos.encode(encoder)
    }
}

#[cfg(feature = "as-float-secs")]
impl Encode for Timestamp {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        let secs = self.as_secs_f64();
        secs.encode(encoder)
    }
}

#[cfg(not(feature = "as-float-secs"))]
impl<C> Decode<C> for Timestamp {
    fn decode<D: bincode::de::Decoder<Context = C>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let nanos = u64::decode(decoder)?;
        Ok(nanos.into())
    }
}

#[cfg(feature = "as-float-secs")]
impl<C> Decode<C> for Timestamp {
    fn decode<D: bincode::de::Decoder<Context = C>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let secs = f64::decode(decoder)?;
        Ok(Timestamp::from_secs_f64(secs))
    }
}

//
// Monotonic
//
impl Encode for Monotonic {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        let nanos: u64 = self.as_nanos().try_into().unwrap();
        nanos.encode(encoder)
    }
}

impl<C> Decode<C> for Monotonic {
    fn decode<D: bincode::de::Decoder<Context = C>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let nanos = u64::decode(decoder)?;
        Ok(nanos.into())
    }
}

bincode::impl_borrow_decode!(Timestamp);
bincode::impl_borrow_decode!(Monotonic);

#[cfg(test)]
mod tests {
    use super::{Monotonic, Timestamp};

    #[test]
    fn test_timestamp_bincode() {
        let ts = Timestamp::from_nanos(1_500_000_000);
        let encoded = bincode::encode_to_vec(ts, bincode::config::standard()).unwrap();
        let (decoded, _) =
            bincode::decode_from_slice(&encoded, bincode::config::standard()).unwrap();
        assert_eq!(ts, decoded);
    }

    #[test]
    fn test_monotonic_bincode() {
        let mono = Monotonic::from_nanos(2_500_000_000);
        let encoded = bincode::encode_to_vec(mono, bincode::config::standard()).unwrap();
        let (decoded, _) =
            bincode::decode_from_slice(&encoded, bincode::config::standard()).unwrap();
        assert_eq!(mono, decoded);
    }
}
