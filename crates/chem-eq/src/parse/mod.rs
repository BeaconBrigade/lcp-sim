#[cfg(test)]
mod tests;
pub mod util;

use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_till1, take_while, take_while1},
    character::complete::{anychar, digit0, multispace0},
    combinator::{map, map_opt, map_res, opt, peek, verify},
    error::{context, Error as NomError, ErrorKind as NomErrorKind},
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
};

use crate::{
    parse::util::{Error, Input, Result},
    Compound, Direction, Element, Equation, State,
};

/// Parse an [`Equation`] from a str
pub fn parse_equation(orig_i: Input) -> Result<Equation> {
    // get the left side of the equals
    let (i, lhs) = context(
        "splitting equation",
        take_till1(|c: char| c == '<' || c == '-'),
    )(orig_i)?;

    // get the direction of reaction
    let (rhs, tag) = context(
        "direction of equation",
        take_while1(|c: char| c == '<' || c == '-' || c == '>'),
    )(i)?;
    let direction = Direction::from_str(tag)
        .map_err(|_| nom::Err::Error(NomError::new(i, NomErrorKind::Verify).into()))?;

    // parse either side
    let (_, left_cmp) = context("left side", parse_side)(lhs)?;
    let (i, right_cmp) = context("right side", parse_side)(rhs)?;

    // clear trailing whitespace
    let mut orig_i = orig_i.to_string();
    orig_i.truncate(orig_i.trim_end().len());
    Ok((
        i,
        Equation {
            left: left_cmp,
            right: right_cmp,
            direction,
            equation: orig_i,
            delta_h: 0.0,
            temperature: 0.0,
        },
    ))
}

/// Parse one side of the equation into [`Compound`]
fn parse_side(i: Input) -> Result<Vec<Compound>> {
    // collect as many compounds as possible skipping leading whitespace
    preceded(multispace0, many1(compound_and_plus))(i)
}

/// Parse an [`Element`]
fn parse_element(orig_i: Input) -> Result<Element> {
    let (i, (c, name)) = context(
        "element name",
        tuple((
            context(
                "starting element letter",
                verify(anychar, |c| c.is_uppercase() && c.is_alphabetic()),
            ),
            context(
                "rest of element name",
                take_while(|i: char| i.is_alphabetic() && i.is_lowercase()),
            ),
        )),
    )(orig_i)?;

    let mut c = c.to_string();
    c.push_str(name);

    // capture the number at the end of the element
    map(
        map_opt(opt(digit0), |s| s.map(str::parse::<usize>)),
        move |num| Element {
            // map expects FnMut which theoretically can be called multiple times, so we can't move
            // out of c
            name: c.clone(),
            count: num.unwrap_or(1),
        },
    )(i)
}

/// Parse a [`Compound`] from an input
fn parse_compound(i: Input) -> Result<Compound> {
    // get prefix of compound and extra elements
    let (i, (num, elements)) = tuple((
        // optional coefficient
        context(
            "compound coefficient",
            map_opt(opt(digit0), |s: Option<&str>| s.map(str::parse::<usize>)),
        ),
        // get all the elements
        context(
            "optionally bracketed elements",
            map(many1(bracketed_elements), |v| {
                v.into_iter().flatten().collect::<Vec<_>>()
            }),
        ),
    ))(i)?;

    // get state of compound
    let (i, state) = match delimited(
        context(
            "leading bracket for compound state",
            tag::<_, _, NomError<&str>>("("),
        ),
        context(
            "compound state",
            map_res(take_while(char::is_alphabetic), State::from_str),
        ),
        context("closing bracket for compound state", tag(")")),
    )(i)
    {
        Ok((i, state)) => (i, Some(state)),
        Err(e) => {
            match e {
                // if state couldn't be parsed
                nom::Err::Error(inner) if inner.code == NomErrorKind::MapRes => {
                    return Err(nom::Err::Error(inner.into()))
                }
                _ => {}
            }
            (i, None)
        }
    };

    Ok((
        i,
        Compound {
            elements,
            coefficient: num.unwrap_or(1),
            state,
            concentration: 0.0,
        },
    ))
}

/// Parse elements that are bracketed with a coefficient on the end
fn bracketed_elements(orig_i: Input) -> Result<Vec<Element>> {
    // get bracket
    let (i, b) = peek(anychar)(orig_i)?;

    // this isn't an element, but a state
    if i.chars().next().unwrap_or('a').is_lowercase() {
        return Err(nom::Err::Error(
            NomError::new(orig_i, NomErrorKind::Verify).into(),
        ));
    }

    // keep track of if we're in brackets
    let (i, deep) = if b == '(' {
        // should be good considering we peeked it earlier
        let (i, _) = anychar::<_, Error<&str>>(i).unwrap();
        (i, true)
    } else {
        (i, false)
    };

    // get list of elements
    let (i, mut elements) = context("elements in compound", many1(parse_element))(i)?;

    // see if there's a second bracket, and if there is, get the coefficient
    //
    // also, check if we've run out of input, and if so, return what we've got so far
    let (i, b) = match peek(anychar::<_, NomError<&str>>)(i) {
        Ok((i, b)) => (i, b),
        Err(e) => match e {
            nom::Err::Error(e) if e.code == NomErrorKind::Eof => return Ok((i, elements)),
            nom::Err::Error(e) => return Err(nom::Err::Error(e.into())),
            nom::Err::Failure(e) => return Err(nom::Err::Failure(e.into())),
            // not using streaming parsers
            nom::Err::Incomplete(_) => unreachable!(),
        },
    };

    let (i, coef) = if b == ')' && deep {
        // same logic as above with peeking
        let (i, _) = anychar::<_, Error<&str>>(i).unwrap();
        let opt_num = opt(digit0);
        context(
            "coefficient for brackets",
            map_opt(opt_num, |s: Option<&str>| s.map(str::parse::<usize>)),
        )(i)?
    } else {
        (i, Ok(1))
    };

    // multiply each element's count by the coefficient
    for el in &mut elements {
        el.count *= coef.as_ref().unwrap_or(&1);
    }

    Ok((i, elements))
}

/// Parse a compound and an optional "+"
fn compound_and_plus(i: Input) -> Result<Compound> {
    terminated(
        context("compound", parse_compound),
        take_while(|c: char| c.is_whitespace() || c == '+'),
    )(i)
}
