//! Error types for the calculator engine.

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("empty input")]
    Empty,

    #[error("invalid number: {0}")]
    InvalidNumber(String),

    #[error("invalid fraction: {0}")]
    InvalidFraction(String),

    #[error("invalid unit: {0}")]
    InvalidUnit(String),

    #[error("malformed length expression: {0}")]
    Malformed(String),

    #[error("denominator must be a power of two between 2 and 16, got {0}")]
    InvalidDenominator(u32),

    #[error("decimal precision must be 0..=12, got {0}")]
    InvalidPrecision(u8),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CalcError {
    #[error("division by zero")]
    DivByZero,

    #[error("type mismatch: cannot apply operation to these value types")]
    TypeMismatch,

    #[error("domain error: {0}")]
    Domain(String),

    #[error("not enough information: {0}")]
    Underdetermined(String),

    #[error("invalid input: {0}")]
    Invalid(String),

    #[error("parse error: {0}")]
    Parse(#[from] ParseError),
}
