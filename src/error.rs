use charts_rs::CanvasError;
use hmac::digest::InvalidLength;
use std::env::VarError;
use std::num::{ParseFloatError, ParseIntError};
use std::time::SystemTimeError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Var(VarError),
    Fetch(reqwest::Error),
    Canvas(CanvasError),
    Io(std::io::Error),
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
    Time(SystemTimeError),
    MinMax,
    Crypto(InvalidLength),
    Request(String),
}

//region      --- From

impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        Error::Var(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Fetch(value)
    }
}

impl From<CanvasError> for Error {
    fn from(value: CanvasError) -> Self {
        Error::Canvas(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Error::ParseFloat(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseInt(value)
    }
}

impl From<SystemTimeError> for Error {
    fn from(value: SystemTimeError) -> Self {
        Error::Time(value)
    }
}

impl From<InvalidLength> for Error {
    fn from(value: InvalidLength) -> Self {
        Error::Crypto(value)
    }
}
//endregion   --- From

//region      --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
//endregion   --- Error Boilerplate
