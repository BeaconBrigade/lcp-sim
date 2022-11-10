#[cfg(test)]
mod tests;

use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_till1, take_while, take_while1},
    character::complete::{anychar, digit0},
    combinator::{map_opt, map_res, opt, peek},
    error::{Error, ErrorKind},
    multi::many1,
    sequence::delimited,
    IResult,
};

use crate::{Compound, Direction, Element, Equation, State};

/// Parse an [`Equation`] from a str
pub fn parse_equation(orig_i: &str) -> IResult<&str, Equation> {
    // get the left side of the equals
    let (i, lhs) = take_till1(|c: char| c == '<' || c == '-')(orig_i)?;

    // get the direction of reaction
    let (rhs, tag) = take_while1(|c: char| c == '<' || c == '-' || c == '>')(i)?;
    let direction =
        Direction::from_str(tag).map_err(|_| nom::Err::Error(Error::new(i, ErrorKind::Verify)))?;

    // parse either side
    let (_, left_cmp) = parse_side(lhs)?;
    let (i, right_cmp) = parse_side(rhs)?;

    // clear trailing whitespace
    let mut orig_i = orig_i.to_string();
    orig_i.truncate(orig_i.trim_end().len());
    Ok((
        i,
        Equation {
            left: left_cmp,
            right: right_cmp,
            direction,
            original_equation: orig_i,
        },
    ))
}

/// Parse one side of the equation into [`Compound`]
fn parse_side(i: &str) -> IResult<&str, Vec<Compound>> {
    let i = i.trim_start();
    // collect as many compounds as possible
    let (i, compounds) = many1(compound_and_plus)(i)?;
    Ok((i, compounds))
}

/// Parse an [`Element`]
fn parse_element(orig_i: &str) -> IResult<&str, Element> {
    // if character is lowercase that means this is not an element
    let (i, c) = anychar(orig_i)?;
    if c.is_lowercase() || !c.is_alphabetic() {
        return Err(nom::Err::Error(Error::new(orig_i, ErrorKind::Char)));
    }

    // take while the letters are lowercase
    let alpha_lower = |i: char| i.is_alphabetic() && i.is_lowercase();
    let (i, name) = take_while(alpha_lower)(i)?;

    let mut c = c.to_string();
    c.push_str(name);

    // capture the number at the end of the element
    let opt_num = opt(digit0);
    let (i, num) = map_opt(opt_num, |s: Option<&str>| s.map(|n| n.parse::<usize>()))(i)?;
    Ok((
        i,
        Element {
            name: c,
            count: num.unwrap_or(1),
        },
    ))
}

/// Parse a [`Compound`] from an input
fn parse_compound(i: &str) -> IResult<&str, Compound> {
    // get prefix of compound
    let opt_num = opt(digit0);
    let (i, num) = map_opt(opt_num, |s: Option<&str>| s.map(|n| n.parse::<usize>()))(i)?;

    // get each sub element
    let (i, extra_elements) = many1(bracketed_elements)(i)?;

    // combine `Vec<Vec<Element>>` into `Vec<Element>`
    let mut elements = vec![];
    extra_elements.into_iter().for_each(|v| elements.extend(v));

    // get state of compound
    let (i, state) = match delimited(
        tag::<_, _, Error<&str>>("("),
        map_res(take_while(|c: char| c.is_alphabetic()), |v: &str| {
            State::from_str(v)
        }),
        tag(")"),
    )(i)
    {
        Ok((i, state)) => (i, Some(state)),
        Err(e) => {
            match e {
                // if state couldn't be parsed
                nom::Err::Error(ref inner) if inner.code == ErrorKind::MapRes => return Err(e),
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
        },
    ))
}

/// Parse elements that are bracketed with a coefficient on the end
fn bracketed_elements(orig_i: &str) -> IResult<&str, Vec<Element>> {
    // get bracket
    let (i, b) = peek(anychar)(orig_i)?;

    // this isn't an element, but a state
    if i.chars().next().unwrap_or('a').is_lowercase() {
        return Err(nom::Err::Error(Error::new(orig_i, ErrorKind::Verify)));
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
    let (i, mut elements) = many1(parse_element)(i)?;

    // see if there's a second bracket, and if there is, get the coefficient
    //
    // also, check if we've run out of input, and if so, return what we've got so far
    let (i, b) = match peek(anychar::<_, Error<&str>>)(i) {
        Ok((i, b)) => (i, b),
        Err(e) => match &e {
            nom::Err::Error(e) if e.code == ErrorKind::Eof => return Ok((i, elements)),
            _ => return Err(e),
        },
    };

    let (i, coef) = if b == ')' && deep {
        // same logic as above with peeking
        let (i, _) = anychar::<_, Error<&str>>(i).unwrap();
        let opt_num = opt(digit0);
        map_opt(opt_num, |s: Option<&str>| s.map(|n| n.parse::<usize>()))(i)?
    } else {
        (i, Ok(1))
    };

    // multiply each element's count by the coefficient
    for el in elements.iter_mut() {
        el.count *= coef.as_ref().unwrap_or(&1);
    }

    Ok((i, elements))
}

/// Parse a compound and an optional "+"
fn compound_and_plus(i: &str) -> IResult<&str, Compound> {
    let (i, compound) = parse_compound(i)?;
    let (i, _) = take_while(|c: char| c.is_whitespace() || c == '+')(i)?;
    Ok((i, compound))
}
