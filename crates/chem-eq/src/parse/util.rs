use std::fmt;

use itertools::Itertools;
use nom::{
    error::{
        ContextError as NomContexError, Error as NomError, ErrorKind as NomErrorKind,
        FromExternalError, ParseError as NomParseError,
    },
    IResult,
};
use thiserror::Error;

use crate::error::ElementError;

pub type Input<'a> = &'a str;
pub type Result<'a, T> = IResult<Input<'a>, T, Error<Input<'a>>>;

/// Error type for parsing equations
#[derive(Error, Clone, PartialEq, Eq)]
pub struct Error<I> {
    pub errors: Vec<(I, ErrorKind)>,
}

impl<I> fmt::Display for Error<I>
where
    I: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (input, kind) in self.errors.iter().rev() {
            writeln!(f, "  {}: {:?}", kind, input)?;
        }
        Ok(())
    }
}

impl<I> fmt::Debug for Error<I>
where
    I: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Custom error kind for parsing equations
#[derive(Clone, Error, PartialEq, Eq)]
pub enum ErrorKind {
    #[error("parse error: {0:#?}")]
    Nom(NomErrorKind),
    #[error("... while getting {0}")]
    Context(&'static str),
    #[error("invalid element \"{0}\"")]
    InvalidElement(String),
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl From<NomErrorKind> for ErrorKind {
    fn from(e: NomErrorKind) -> Self {
        Self::Nom(e)
    }
}

impl<I> From<NomError<I>> for Error<I> {
    fn from(e: NomError<I>) -> Self {
        Self {
            errors: vec![(e.input, e.code.into())],
        }
    }
}

impl<I> NomParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: NomErrorKind) -> Self {
        Self {
            errors: vec![(input, ErrorKind::Nom(kind))],
        }
    }

    fn append(input: I, kind: NomErrorKind, mut other: Self) -> Self {
        other.errors.push((input, ErrorKind::Nom(kind)));
        other
    }

    fn or(mut self, other: Self) -> Self {
        self.errors.extend(other.errors);
        self
    }
}

impl<I> NomContexError<I> for Error<I>
where
    I: Clone,
{
    fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
        other.errors.push((input, ErrorKind::Context(ctx)));
        other
    }
}

impl<I> FromExternalError<I, ElementError> for Error<I> {
    fn from_external_error(input: I, _kind: NomErrorKind, e: ElementError) -> Self {
        Self {
            errors: vec![(input, ErrorKind::InvalidElement(e.0))],
        }
    }
}

impl From<Error<&str>> for Error<String> {
    fn from(e: Error<&str>) -> Self {
        Self {
            errors: e
                .errors
                .into_iter()
                .map(|(s, k)| (s.to_string(), k))
                .collect_vec(),
        }
    }
}
