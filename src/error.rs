use std::num::{ParseFloatError, ParseIntError, TryFromIntError};

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("timestamp parse failed {0}")]
    Parse(String),
    #[error("timestamp conversion failed")]
    ConvertChrono,
    #[error("timestamp number conversion failed: {0}")]
    Convert(String),
    #[error("time went backward")]
    TimeWentBackward,
}

macro_rules! impl_convert_err {
    ($t: ty) => {
        impl From<$t> for Error {
            fn from(err: $t) -> Self {
                Self::Convert(err.to_string())
            }
        }
    };
}

impl_convert_err!(TryFromIntError);
impl_convert_err!(ParseIntError);
impl_convert_err!(ParseFloatError);
