//! Todo: Documentations

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{name} must be in the range {min}..{max} given {given}")]
    InvalidRange {
        name: &'static str,
        given: i32,
        min: i32,
        max: i32,
    },

    #[error("can not parse {0}, invalid token `{1}`")]
    InvalidVariant(&'static str, String),

    #[error("conversion faild")]
    DateConversion(#[from] time::error::ComponentRange),

    #[error("invalid {0} date given")]
    InvalidDate(String),
}

pub fn is_in_range(value: i32, min: i32, max: i32, name: &'static str) -> Result<(), Error> {
    if value >= min && value <= max {
        Ok(())
    } else {
        Err(Error::InvalidRange {
            name,
            given: value,
            min,
            max,
        })
    }
}
