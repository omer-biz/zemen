use std::fmt;
use std::error;

#[derive(Debug)]
pub struct Error {
    pub msg: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
            self.msg
        )
    }
}

impl From<InvalidDateRangeError> for Error {
    fn from(value: InvalidDateRangeError) -> Self {
        Error { msg: format!("{}", value) }
    }
}

impl From<InvalidVariant> for Error {
    fn from(value: InvalidVariant) -> Self {
        Error { msg: format!("{}", value) }
    }
}

impl From<time::error::ComponentRange> for Error {
    fn from(value: time::error::ComponentRange) -> Self {
        Error { msg: format!("{}", value) }
    }
}

impl error::Error for Error {}

#[derive(Debug)]
pub struct InvalidDateRangeError {
    pub name: &'static str,
    pub given: i32,
    pub max: i32,
    pub min: i32,
}

impl fmt::Display for InvalidDateRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} must be in the range {}..{} given {}",
            self.name, self.min, self.max, self.given
        )
    }
}

impl error::Error for InvalidDateRangeError {}

#[derive(Debug)]
pub struct InvalidVariant {
    pub name: &'static str,
}

impl fmt::Display for InvalidVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error setting: {}",
            self.name
        )
    }
}

impl error::Error for InvalidVariant {}

