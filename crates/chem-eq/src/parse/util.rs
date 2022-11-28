use std::fmt;

use itertools::Itertools;
use nom::{
    error::{
        ContextError as NomContexError, Error as NomError, ErrorKind as NomErrorKind,
        ParseError as NomParseError,
    },
    IResult,
};
use thiserror::Error;

pub type Input<'a> = &'a str;
pub type Result<'a, T> = IResult<Input<'a>, T, Error<Input<'a>>>;

/// Error type for parsing equations
#[derive(Error, Clone, PartialEq, Eq)]
pub struct Error<I> {
    pub errors: Vec<(I, ErrorKind)>,
}

impl<I> fmt::Display for Error<I> 
where
    I: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for (input, kind) in self.errors.iter() {
            writeln!(f, "- {}: {}", kind, input)?;
        }
        writeln!(f)
    }
}

impl<I> fmt::Debug for Error<I> 
where
    I: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Custom error kind for parsing equations
#[derive(Clone, Copy, Error, PartialEq, Eq)]
pub enum ErrorKind {
    #[error("Parse error: {0:#?}")]
    Nom(NomErrorKind),
    #[error("... in {0}")]
    Context(&'static str),
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

impl<I> NomContexError<I> for Error<I> {
    fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
        other.errors.push((input, ErrorKind::Context(ctx)));
        other
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
