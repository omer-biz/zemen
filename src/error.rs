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

    #[error("can not parse {0}")]
    InvalidVariant(&'static str),

    #[error("conversion faild")]
    DateConversion(#[from] time::error::ComponentRange),
}
