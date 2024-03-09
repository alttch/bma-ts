use crate::{Monotonic, Timestamp};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef},
    sqlite::{SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef},
    Decode, Encode, Postgres, Sqlite, Type,
};

// Timestamp

impl Type<Sqlite> for Timestamp {
    fn type_info() -> SqliteTypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
    fn compatible(ty: &SqliteTypeInfo) -> bool {
        *ty == <i64 as Type<Sqlite>>::type_info()
            || *ty == <i32 as Type<Sqlite>>::type_info()
            || *ty == <i16 as Type<Sqlite>>::type_info()
            || *ty == <i8 as Type<Sqlite>>::type_info()
    }
}
impl<'q> Encode<'q, Sqlite> for Timestamp {
    fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
        args.push(SqliteArgumentValue::Int64(
            (*self).try_into().expect("timestamp too large"),
        ));
        IsNull::No
    }
}
impl<'r> Decode<'r, Sqlite> for Timestamp {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let value = <i64 as Decode<Sqlite>>::decode(value)?;
        Ok(value.try_into()?)
    }
}

const J2000_EPOCH_US: i64 = 946_684_800_000_000;

impl Type<Postgres> for Timestamp {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("TIMESTAMPTZ")
    }
    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == PgTypeInfo::with_name("TIMESTAMPTZ") || *ty == PgTypeInfo::with_name("TIMESTAMP")
    }
}

impl PgHasArrayType for Timestamp {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_TIMESTAMPTZ")
    }

    fn array_compatible(ty: &PgTypeInfo) -> bool {
        *ty == PgTypeInfo::with_name("_TIMESTAMPTZ") || *ty == PgTypeInfo::with_name("_TIMESTAMP")
    }
}

impl Encode<'_, Postgres> for Timestamp {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        let us = i64::try_from(self.as_micros()).expect("timestamp too large") - J2000_EPOCH_US;
        Encode::<Postgres>::encode(us, buf)
    }
    fn size_hint(&self) -> usize {
        std::mem::size_of::<i64>()
    }
}
impl<'r> Decode<'r, Postgres> for Timestamp {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let us: i64 = Decode::<Postgres>::decode(value)?;
        Ok(Timestamp::from_micros((us + J2000_EPOCH_US).try_into()?))
    }
}

// Monotonic

impl Type<Sqlite> for Monotonic {
    fn type_info() -> SqliteTypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
    fn compatible(ty: &SqliteTypeInfo) -> bool {
        *ty == <i64 as Type<Sqlite>>::type_info()
            || *ty == <i32 as Type<Sqlite>>::type_info()
            || *ty == <i16 as Type<Sqlite>>::type_info()
            || *ty == <i8 as Type<Sqlite>>::type_info()
    }
}
impl<'q> Encode<'q, Sqlite> for Monotonic {
    fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
        args.push(SqliteArgumentValue::Int64(
            (*self).try_into().expect("timestamp too large"),
        ));
        IsNull::No
    }
}
impl<'r> Decode<'r, Sqlite> for Monotonic {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let value = <i64 as Decode<Sqlite>>::decode(value)?;
        Ok(value.try_into()?)
    }
}

impl Type<Postgres> for Monotonic {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("INT8")
    }
}

impl PgHasArrayType for Monotonic {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_INT8")
    }
}

impl Encode<'_, Postgres> for Monotonic {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        let us = i64::try_from(self.as_nanos()).expect("timestamp too large");
        Encode::<Postgres>::encode(us, buf)
    }
    fn size_hint(&self) -> usize {
        std::mem::size_of::<i64>()
    }
}

impl<'r> Decode<'r, Postgres> for Monotonic {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let ns: i64 = Decode::<Postgres>::decode(value)?;
        Ok(Monotonic::from_nanos(ns.try_into()?))
    }
}
