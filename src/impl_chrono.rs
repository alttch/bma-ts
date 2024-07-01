use std::time::Duration;

use super::{Error, Timestamp};
use chrono::{DateTime, Local, NaiveDateTime, Utc};

impl TryFrom<Timestamp> for NaiveDateTime {
    type Error = Error;
    #[inline]
    fn try_from(t: Timestamp) -> Result<Self, Self::Error> {
        let d = DateTime::from_timestamp(i64::try_from(t.0.as_secs())?, t.0.subsec_nanos())
            .ok_or(Error::ConvertChrono)?;
        Ok(d.naive_utc())
    }
}
impl TryFrom<Timestamp> for DateTime<Utc> {
    type Error = Error;
    fn try_from(t: Timestamp) -> Result<Self, Self::Error> {
        let nt = NaiveDateTime::try_from(t)?;
        let dt_utc = DateTime::<Utc>::from_naive_utc_and_offset(nt, Utc);
        Ok(dt_utc)
    }
}
impl TryFrom<Timestamp> for DateTime<Local> {
    type Error = Error;
    fn try_from(t: Timestamp) -> Result<Self, Self::Error> {
        let dt_utc = DateTime::<Utc>::try_from(t)?;
        Ok(DateTime::from(dt_utc))
    }
}
impl TryFrom<NaiveDateTime> for Timestamp {
    type Error = Error;
    fn try_from(datetime: NaiveDateTime) -> Result<Self, Self::Error> {
        let duration_nanos = Duration::from_secs(u64::try_from(
            datetime
                .and_utc()
                .timestamp_nanos_opt()
                .ok_or(Error::ConvertChrono)?,
        )?);
        Ok(Self(duration_nanos))
    }
}
impl TryFrom<DateTime<Utc>> for Timestamp {
    type Error = Error;
    fn try_from(datetime: DateTime<Utc>) -> Result<Self, Self::Error> {
        let duration_sec = Duration::from_secs(u64::try_from(datetime.timestamp())?);
        let duration_nanos = Duration::from_nanos(u64::from(datetime.timestamp_subsec_nanos()));
        Ok(Self(duration_sec + duration_nanos))
    }
}
impl TryFrom<DateTime<Local>> for Timestamp {
    type Error = Error;
    fn try_from(datetime: DateTime<Local>) -> Result<Self, Self::Error> {
        let duration_sec = Duration::from_secs(u64::try_from(datetime.timestamp())?);
        let duration_nanos = Duration::from_nanos(u64::from(datetime.timestamp_subsec_nanos()));
        Ok(Self(duration_sec + duration_nanos))
    }
}
impl Timestamp {
    #[inline]
    pub fn try_into_naivedatetime(self) -> Result<NaiveDateTime, Error> {
        self.try_into()
    }
    #[inline]
    pub fn try_into_datetime_local(self) -> Result<DateTime<Local>, Error> {
        self.try_into()
    }
    #[inline]
    pub fn try_into_datetime_utc(self) -> Result<DateTime<Utc>, Error> {
        self.try_into()
    }
}
