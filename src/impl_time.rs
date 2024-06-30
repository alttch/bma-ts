use time::OffsetDateTime;

use crate::{Error, Timestamp};

impl TryFrom<Timestamp> for OffsetDateTime {
    type Error = Error;

    fn try_from(ts: Timestamp) -> Result<Self, Self::Error> {
        OffsetDateTime::from_unix_timestamp_nanos(i128::try_from(ts.as_nanos())?)
            .map_err(Into::into)
    }
}

impl TryFrom<OffsetDateTime> for Timestamp {
    type Error = Error;

    fn try_from(dt: OffsetDateTime) -> Result<Self, Self::Error> {
        Ok(Timestamp::from_nanos(u64::try_from(
            dt.unix_timestamp_nanos(),
        )?))
    }
}
