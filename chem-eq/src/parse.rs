use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_until1, take_while},
    character::complete::{anychar, digit0},
    combinator::{map_opt, map_res, opt},
    error::{Error, ErrorKind},
    multi::many1,
    sequence::delimited,
    IResult,
};

use crate::{Compound, Element, Equation, State};

/// Parse an [`Equation`] from a str
pub fn parse_equation(i: &str) -> IResult<&str, Equation> {
    let (i, lhs) = take_until1("->")(i)?;
    let (rhs, _) = tag("->")(i)?;
    let (_, left_cmp) = parse_side(lhs)?;
    let (_, right_cmp) = parse_side(rhs)?;

    Ok((
        "",
        Equation {
            left: left_cmp,
            right: right_cmp,
        },
    ))
}

/// Parse one side of the equation into [`Compound`]
fn parse_side(i: &str) -> IResult<&str, Vec<Compound>> {
    // let (i, )
    todo!()
}

/// Parse an [`Element`]
fn parse_element(i: &str) -> IResult<&str, Element> {
    let (i, c) = anychar(i)?;
    if c.is_lowercase() || !c.is_alphabetic() {
        return Err(nom::Err::Error(Error::new(
            "Invalid element name.",
            ErrorKind::Char,
        )));
    }

    let alpha_lower = |i: char| i.is_alphabetic() && i.is_lowercase();
    let (i, name) = take_while(alpha_lower)(i)?;

    let mut c = c.to_string();
    c.push_str(name);

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
    let opt_num = opt(digit0);
    let (i, num) = map_opt(opt_num, |s: Option<&str>| s.map(|n| n.parse::<usize>()))(i)?;

    let (i, elements) = many1(parse_element)(i)?;
    dbg!("after elms", i, &elements);
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
            dbg!(&e);
            match e {
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
fn bracketed_elements(i: &str) -> IResult<&str, Vec<Element>> {
    todo!()
}

/// Parse a compound and an optional "+"
fn compound_and_plus(i: &str) -> IResult<&str, Compound> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn element_without_number_one_letter() {
        let el = Element {
            name: "O".to_owned(),
            count: 1,
        };
        assert_eq!(parse_element("O"), Ok(("", el)));
    }

    #[test]
    fn element_with_number_one_letter() {
        let el = Element {
            name: "O".to_owned(),
            count: 1,
        };
        assert_eq!(parse_element("O1"), Ok(("", el)));
    }

    #[test]
    fn element_one_letter_multi_number() {
        let el = Element {
            name: "O".to_owned(),
            count: 12,
        };
        assert_eq!(parse_element("O12"), Ok(("", el)));
    }

    #[test]
    fn element_one_letter_from_compound() {
        let el = Element {
            name: "K".to_owned(),
            count: 3,
        };
        assert_eq!(parse_element("K3S"), Ok(("S", el)));
    }

    #[test]
    fn element_two_letter_no_number() {
        let el = Element {
            name: "Fe".to_owned(),
            count: 1,
        };
        assert_eq!(parse_element("Fe"), Ok(("", el)));
    }

    #[test]
    fn element_two_letter_with_number() {
        let el = Element {
            name: "Fe".to_owned(),
            count: 2,
        };
        assert_eq!(parse_element("Fe2"), Ok(("", el)));
    }

    #[test]
    fn element_in_compound_with_capital_no_number() {
        let el = Element {
            name: "Fee".to_owned(),
            count: 1,
        };
        assert_eq!(parse_element("FeeK2"), Ok(("K2", el)));
    }

    #[test]
    fn element_in_compound_with_capital_number() {
        let el = Element {
            name: "Fee".to_owned(),
            count: 3,
        };
        assert_eq!(parse_element("Fee3K2"), Ok(("K2", el)));
    }

    #[test]
    fn element_not_letters() {
        assert_eq!(
            parse_element("+2"),
            Err(nom::Err::Error(Error::new(
                "Invalid element name.",
                ErrorKind::Char
            )))
        );
    }

    #[test]
    fn element_not_letters_whitespace() {
        assert_eq!(
            parse_element(" "),
            Err(nom::Err::Error(Error::new(
                "Invalid element name.",
                ErrorKind::Char
            )))
        );
    }

    #[test]
    fn compound_one_element_no_coef() {
        let cmp = Compound {
            elements: vec![Element {
                name: "O".to_owned(),
                count: 2,
            }],
            coefficient: 1,
            state: None,
        };
        assert_eq!(parse_compound("O2"), Ok(("", cmp)));
    }

    #[test]
    fn compound_one_element_coef() {
        let cmp = Compound {
            elements: vec![Element {
                name: "O".to_owned(),
                count: 2,
            }],
            coefficient: 1,
            state: None,
        };
        assert_eq!(parse_compound("1O2"), Ok(("", cmp)));
    }

    #[test]
    fn compound_one_element_coef_big() {
        let cmp = Compound {
            elements: vec![Element {
                name: "O".to_owned(),
                count: 2,
            }],
            coefficient: 2,
            state: None,
        };
        assert_eq!(parse_compound("2O2"), Ok(("", cmp)));
    }

    #[test]
    fn compound_one_element_coef_long() {
        let cmp = Compound {
            elements: vec![Element {
                name: "O".to_owned(),
                count: 2,
            }],
            coefficient: 13,
            state: None,
        };
        assert_eq!(parse_compound("13O2"), Ok(("", cmp)));
    }

    #[test]
    fn compound_two_elements_no_coef() {
        let cmp = Compound {
            elements: vec![
                Element {
                    name: "Fe".to_owned(),
                    count: 2,
                },
                Element {
                    name: "O".to_owned(),
                    count: 3,
                },
            ],
            coefficient: 1,
            state: None,
        };
        assert_eq!(parse_compound("Fe2O3"), Ok(("", cmp)));
    }

    #[test]
    fn compound_two_elements_coef() {
        let cmp = Compound {
            elements: vec![
                Element {
                    name: "Fe".to_owned(),
                    count: 2,
                },
                Element {
                    name: "O".to_owned(),
                    count: 3,
                },
            ],
            coefficient: 2,
            state: None,
        };
        assert_eq!(parse_compound("2Fe2O3"), Ok(("", cmp)));
    }

    #[test]
    fn compound_three_elements_coef() {
        // I know this is not a regular compound
        let cmp = Compound {
            elements: vec![
                Element {
                    name: "Fe".to_owned(),
                    count: 2,
                },
                Element {
                    name: "O".to_owned(),
                    count: 3,
                },
                Element {
                    name: "Pb".to_owned(),
                    count: 1,
                },
            ],
            coefficient: 2,
            state: None,
        };
        assert_eq!(parse_compound("2Fe2O3Pb"), Ok(("", cmp)));
    }

    #[test]
    fn compound_two_elements_coef_no_num() {
        // I know this is not a regular compound
        let cmp = Compound {
            elements: vec![
                Element {
                    name: "O".to_owned(),
                    count: 1,
                },
                Element {
                    name: "H".to_owned(),
                    count: 1,
                },
            ],
            coefficient: 33,
            state: None,
        };
        assert_eq!(parse_compound("33OH"), Ok(("", cmp)));
    }

    #[test]
    fn two_compounds_coef() {
        let cmp = Compound {
            elements: vec![
                Element {
                    name: "O".to_owned(),
                    count: 1,
                },
                Element {
                    name: "H".to_owned(),
                    count: 1,
                },
            ],
            coefficient: 3,
            state: None,
        };
        assert_eq!(parse_compound("3OH + Fe"), Ok((" + Fe", cmp)));
    }

    #[test]
    fn compound_and_state() {
        let cmp = Compound {
            elements: vec![
                Element {
                    name: "O".to_owned(),
                    count: 1,
                },
                Element {
                    name: "H".to_owned(),
                    count: 1,
                },
            ],
            coefficient: 3,
            state: Some(State::Aqueous),
        };
        assert_eq!(parse_compound("3OH(aq) + Fe"), Ok((" + Fe", cmp)));
    }

    #[test]
    fn compound_and_state_solid() {
        let cmp = Compound {
            elements: vec![
                Element {
                    name: "O".to_owned(),
                    count: 1,
                },
                Element {
                    name: "H".to_owned(),
                    count: 1,
                },
            ],
            coefficient: 3,
            state: Some(State::Solid),
        };
        assert_eq!(parse_compound("3OH(s) + Fe"), Ok((" + Fe", cmp)));
    }

    #[test]
    fn compound_and_state_invalid() {
        assert_eq!(
            parse_compound("3OH(f) + Fe"),
            Err(nom::Err::Error(Error::new("f) + Fe", ErrorKind::MapRes)))
        );
    }

    #[test]
    fn compound_and_brackets() {
        let cmp = Compound {
            elements: vec![
                Element {
                    name: "Ca".to_owned(),
                    count: 1,
                },
                Element {
                    name: "O".to_owned(),
                    count: 2,
                },
                Element {
                    name: "H".to_owned(),
                    count: 2,
                },
            ],
            coefficient: 3,
            state: None,
        };
        assert_eq!(parse_compound("Ca(OH)2"), Ok(("", cmp)));
    }

    // #[test]
    // fn simple_eq() {
    //     let eq = Equation {
    //         left: vec![
    //             Compound {
    //                 elements: vec![Element {
    //                     name: "O".to_owned(),
    //                     count: 2,
    //                 }],
    //                 coefficient: 3,
    //                 state: None,
    //             },
    //             Compound {
    //                 elements: vec![Element {
    //                     name: "Fe".to_owned(),
    //                     count: 4,
    //                 }],
    //                 coefficient: 1,
    //                 state: None,
    //             },
    //         ],
    //         right: vec![Compound {
    //             elements: vec![
    //                 Element {
    //                     name: "Fe".to_owned(),
    //                     count: 2,
    //                 },
    //                 Element {
    //                     name: "O".to_owned(),
    //                     count: 3,
    //                 },
    //             ],
    //             coefficient: 2,
    //             state: None,
    //         }],
    //     };
    //     let other = parse_equation("3O2 + 4Fe -> 2Fe2O3").unwrap();
    //
    //     assert_eq!(eq, other);
    // }
}
