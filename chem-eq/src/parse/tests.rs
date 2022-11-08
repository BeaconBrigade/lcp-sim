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
        Err(nom::Err::Error(Error::new("+2", ErrorKind::Char)))
    );
}

#[test]
fn element_not_letters_whitespace() {
    assert_eq!(
        parse_element(" "),
        Err(nom::Err::Error(Error::new(" ", ErrorKind::Char)))
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
        coefficient: 1,
        state: None,
    };
    assert_eq!(parse_compound("Ca(OH)2"), Ok(("", cmp)));
}

#[test]
fn compound_and_brackets_front() {
    let cmp = Compound {
        elements: vec![
            Element {
                name: "C".to_owned(),
                count: 9,
            },
            Element {
                name: "H".to_owned(),
                count: 18,
            },
            Element {
                name: "C".to_owned(),
                count: 1,
            },
            Element {
                name: "H".to_owned(),
                count: 3,
            },
        ],
        coefficient: 1,
        state: None,
    };
    assert_eq!(parse_compound("(CH2)9CH3"), Ok(("", cmp)));
}

#[test]
fn compound_and_brackets_state() {
    let cmp = Compound {
        elements: vec![
            Element {
                name: "Mg".to_owned(),
                count: 3,
            },
            Element {
                name: "P".to_owned(),
                count: 2,
            },
            Element {
                name: "O".to_owned(),
                count: 8,
            },
        ],
        coefficient: 4,
        state: Some(State::Solid),
    };
    assert_eq!(parse_compound("4Mg3(PO4)2(s)"), Ok(("", cmp)));
}

#[test]
fn compound_and_brackets_state_trailing_space() {
    let cmp = Compound {
        elements: vec![
            Element {
                name: "Mg".to_owned(),
                count: 3,
            },
            Element {
                name: "P".to_owned(),
                count: 2,
            },
            Element {
                name: "O".to_owned(),
                count: 8,
            },
        ],
        coefficient: 4,
        state: Some(State::Solid),
    };
    assert_eq!(parse_compound("4Mg3(PO4)2(s) + "), Ok((" + ", cmp)));
}

#[test]
fn compound_and_plus_simple() {
    let cmp = Compound {
        elements: vec![
            Element {
                name: "Na".to_owned(),
                count: 1,
            },
            Element {
                name: "Cl".to_owned(),
                count: 1,
            },
        ],
        coefficient: 1,
        state: None,
    };
    assert_eq!(compound_and_plus("NaCl + "), Ok(("", cmp)));
}

#[test]
fn compound_and_plus_two_compounds() {
    let cmp = Compound {
        elements: vec![
            Element {
                name: "Na".to_owned(),
                count: 1,
            },
            Element {
                name: "Cl".to_owned(),
                count: 1,
            },
        ],
        coefficient: 1,
        state: None,
    };
    assert_eq!(compound_and_plus("NaCl + Mg(OH)2"), Ok(("Mg(OH)2", cmp)));
}

#[test]
fn one_side() {
    let cmp = vec![
        Compound {
            elements: vec![
                Element {
                    name: "Na".to_owned(),
                    count: 1,
                },
                Element {
                    name: "Cl".to_owned(),
                    count: 1,
                },
            ],
            coefficient: 1,
            state: None,
        },
        Compound {
            elements: vec![
                Element {
                    name: "Mg".to_owned(),
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
            coefficient: 1,
            state: None,
        },
    ];
    assert_eq!(parse_side("NaCl + Mg(OH)2"), Ok(("", cmp)));
}

#[test]
fn one_side_three_comps() {
    let cmp = vec![
        Compound {
            elements: vec![
                Element {
                    name: "Na".to_owned(),
                    count: 1,
                },
                Element {
                    name: "Cl".to_owned(),
                    count: 1,
                },
            ],
            coefficient: 1,
            state: Some(State::Aqueous),
        },
        Compound {
            elements: vec![
                Element {
                    name: "Mg".to_owned(),
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
            coefficient: 1,
            state: Some(State::Solid),
        },
        Compound {
            elements: vec![Element {
                name: "O".to_owned(),
                count: 2,
            }],
            coefficient: 1,
            state: Some(State::Gas),
        },
    ];
    assert_eq!(parse_side("NaCl(aq) + Mg(OH)2(s) + O2(g)"), Ok(("", cmp)));
}

#[test]
fn simple_eq() {
    let eq = Equation {
        left: vec![
            Compound {
                elements: vec![Element {
                    name: "O".to_owned(),
                    count: 2,
                }],
                coefficient: 3,
                state: None,
            },
            Compound {
                elements: vec![Element {
                    name: "Fe".to_owned(),
                    count: 1,
                }],
                coefficient: 4,
                state: None,
            },
        ],
        right: vec![Compound {
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
        }],
        direction: Direction::Right,
        original_equation: "3O2 + 4Fe -> 2Fe2O3".to_owned(),
    };

    assert_eq!(parse_equation("3O2 + 4Fe -> 2Fe2O3"), Ok(("", eq)));
}

#[test]
fn kitchen_sink() {
    let eq = Equation {
        left: vec![
            Compound {
                elements: vec![
                    Element {
                        name: "N".to_owned(),
                        count: 2,
                    },
                    Element {
                        name: "H".to_owned(),
                        count: 8,
                    },
                    Element {
                        name: "S".to_owned(),
                        count: 1,
                    },
                    Element {
                        name: "O".to_owned(),
                        count: 4,
                    }
                ],
                coefficient: 3,
                state: Some(State::Aqueous),
            },
            Compound {
                elements: vec![Element {
                    name: "Fe".to_owned(),
                    count: 3,
                },
                Element {
                    name: "P".to_owned(),
                    count: 2,
                },
                Element {
                    name: "O".to_owned(),
                    count: 8,
                }],
                coefficient: 1,
                state: Some(State::Solid),
            },
        ],
        right: vec![
            Compound {
                elements: vec![
                    Element {
                        name: "N".to_owned(),
                        count: 3,
                    },
                    Element {
                        name: "H".to_owned(),
                        count: 12,
                    },
                    Element {
                        name: "P".to_owned(),
                        count: 1,
                    },
                    Element {
                        name: "O".to_owned(),
                        count: 4,
                    },
                ],
                coefficient: 2,
                state: Some(State::Aqueous),
            },
            Compound {
                elements: vec![Element {
                    name: "Fe".to_owned(),
                    count: 1,
                },
                Element {
                    name: "S".to_owned(),
                    count: 1,
                },
                Element {
                    name: "O".to_owned(),
                    count: 4,
                }],
                coefficient: 3,
                state: Some(State::Aqueous),
            },
        ],
        direction: Direction::Left,
        original_equation: "3(NH4)2SO4(aq) + Fe3(PO4)2(s) <- 2(NH4)3PO4(aq) + 3FeSO4(aq)"
            .to_owned(),
    };

    assert_eq!(parse_equation("3(NH4)2SO4(aq) + Fe3(PO4)2(s) <- 2(NH4)3PO4(aq) + 3FeSO4(aq)"), Ok(("", eq)));
}
