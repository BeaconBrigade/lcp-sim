use nom::error::{ContextError as NomContextError, Error as NomError, ErrorKind as NomErrorKind};

use super::*;

#[test]
fn element_without_number_one_letter() {
    let el = SimpleElement {
        name: "O".to_owned(),
        count: 1,
    };
    assert_eq!(parse_element("O"), Ok(("", el.into_element().unwrap())));
}

#[test]
fn element_with_number_one_letter() {
    let el = SimpleElement {
        name: "O".to_owned(),
        count: 1,
    };
    assert_eq!(parse_element("O1"), Ok(("", el.into_element().unwrap())));
}

#[test]
fn element_one_letter_multi_number() {
    let el = SimpleElement {
        name: "O".to_owned(),
        count: 12,
    };
    assert_eq!(parse_element("O12"), Ok(("", el.into_element().unwrap())));
}

#[test]
fn element_one_letter_from_compound() {
    let el = SimpleElement {
        name: "K".to_owned(),
        count: 3,
    };
    assert_eq!(parse_element("K3S"), Ok(("S", el.into_element().unwrap())));
}

#[test]
fn element_two_letter_no_number() {
    let el = SimpleElement {
        name: "Fe".to_owned(),
        count: 1,
    };
    assert_eq!(parse_element("Fe"), Ok(("", el.into_element().unwrap())));
}

#[test]
fn element_two_letter_with_number() {
    let el = SimpleElement {
        name: "Fe".to_owned(),
        count: 2,
    };
    assert_eq!(parse_element("Fe2"), Ok(("", el.into_element().unwrap())));
}

#[test]
fn element_in_compound_with_capital_no_number() {
    let el = SimpleElement {
        name: "Fe".to_owned(),
        count: 1,
    };
    assert_eq!(
        parse_element("FeK2"),
        Ok(("K2", el.into_element().unwrap()))
    );
}

#[test]
fn element_in_compound_with_capital_number() {
    let el = SimpleElement {
        name: "Fe".to_owned(),
        count: 3,
    };
    assert_eq!(
        parse_element("Fe3K2"),
        Ok(("K2", el.into_element().unwrap()))
    );
}

#[test]
fn element_not_letters() {
    let e = NomError::new("+2", NomErrorKind::Verify).into();
    let e = Error::add_context("+2", "starting element letter", e);
    let e = Error::add_context("+2", "element name", e);
    assert_eq!(parse_element("+2"), Err(nom::Err::Error(e)));
}

#[test]
fn element_not_letters_whitespace() {
    let e = NomError::new(" ", NomErrorKind::Verify).into();
    let e = Error::add_context(" ", "starting element letter", e);
    let e = Error::add_context(" ", "element name", e);
    assert_eq!(parse_element(" "), Err(nom::Err::Error(e)));
}

#[test]
fn compound_one_element_no_coef() {
    let cmp = Compound {
        elements: vec![SimpleElement {
            name: "O".to_owned(),
            count: 2,
        }
        .into_element()
        .unwrap()],
        coefficient: 1,
        ..Default::default()
    };
    assert_eq!(parse_compound("O2"), Ok(("", cmp)));
}

#[test]
fn compound_one_element_coef() {
    let cmp = Compound {
        elements: vec![SimpleElement {
            name: "O".to_owned(),
            count: 2,
        }
        .into_element()
        .unwrap()],
        coefficient: 1,
        ..Default::default()
    };
    assert_eq!(parse_compound("1O2"), Ok(("", cmp)));
}

#[test]
fn compound_one_element_coef_big() {
    let cmp = Compound {
        elements: vec![SimpleElement {
            name: "O".to_owned(),
            count: 2,
        }
        .into_element()
        .unwrap()],
        coefficient: 2,
        ..Default::default()
    };
    assert_eq!(parse_compound("2O2"), Ok(("", cmp)));
}

#[test]
fn compound_one_element_coef_long() {
    let cmp = Compound {
        elements: vec![SimpleElement {
            name: "O".to_owned(),
            count: 2,
        }
        .into_element()
        .unwrap()],
        coefficient: 13,
        ..Default::default()
    };
    assert_eq!(parse_compound("13O2"), Ok(("", cmp)));
}

#[test]
fn compound_two_elements_no_coef() {
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "Fe".to_owned(),
                count: 2,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "O".to_owned(),
                count: 3,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 1,
        ..Default::default()
    };
    assert_eq!(parse_compound("Fe2O3"), Ok(("", cmp)));
}

#[test]
fn compound_two_elements_coef() {
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "Fe".to_owned(),
                count: 2,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "O".to_owned(),
                count: 3,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 2,
        ..Default::default()
    };
    assert_eq!(parse_compound("2Fe2O3"), Ok(("", cmp)));
}

#[test]
fn compound_three_elements_coef() {
    // I know this is not a regular compound
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "Fe".to_owned(),
                count: 2,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "O".to_owned(),
                count: 3,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "Pb".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 2,
        ..Default::default()
    };
    assert_eq!(parse_compound("2Fe2O3Pb"), Ok(("", cmp)));
}

#[test]
fn compound_two_elements_coef_no_num() {
    // I know this is not a regular compound
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "O".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "H".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 33,
        ..Default::default()
    };
    assert_eq!(parse_compound("33OH"), Ok(("", cmp)));
}

#[test]
fn two_compounds_coef() {
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "O".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "H".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 3,
        ..Default::default()
    };
    assert_eq!(parse_compound("3OH + Fe"), Ok((" + Fe", cmp)));
}

#[test]
fn compound_and_state() {
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "O".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "H".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 3,
        state: Some(State::Aqueous),
        ..Default::default()
    };
    assert_eq!(parse_compound("3OH(aq) + Fe"), Ok((" + Fe", cmp)));
}

#[test]
fn compound_and_state_solid() {
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "O".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "H".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 3,
        state: Some(State::Solid),
        ..Default::default()
    };
    assert_eq!(parse_compound("3OH(s) + Fe"), Ok((" + Fe", cmp)));
}

#[test]
fn compound_and_state_invalid() {
    assert_eq!(
        parse_compound("3OH(f) + Fe"),
        Err(nom::Err::Error(
            NomError::new("f) + Fe", NomErrorKind::MapRes).into()
        ))
    );
}

#[test]
fn compound_and_brackets() {
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "Ca".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "O".to_owned(),
                count: 2,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "H".to_owned(),
                count: 2,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 1,
        ..Default::default()
    };
    assert_eq!(parse_compound("Ca(OH)2"), Ok(("", cmp)));
}

#[test]
fn compound_and_brackets_front() {
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "C".to_owned(),
                count: 9,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "H".to_owned(),
                count: 18,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "C".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "H".to_owned(),
                count: 3,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 1,
        ..Default::default()
    };
    assert_eq!(parse_compound("(CH2)9CH3"), Ok(("", cmp)));
}

#[test]
fn compound_and_brackets_state() {
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "Mg".to_owned(),
                count: 3,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "P".to_owned(),
                count: 2,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "O".to_owned(),
                count: 8,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 4,
        state: Some(State::Solid),
        ..Default::default()
    };
    assert_eq!(parse_compound("4Mg3(PO4)2(s)"), Ok(("", cmp)));
}

#[test]
fn compound_and_brackets_state_trailing_space() {
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "Mg".to_owned(),
                count: 3,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "P".to_owned(),
                count: 2,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "O".to_owned(),
                count: 8,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 4,
        state: Some(State::Solid),
        ..Default::default()
    };
    assert_eq!(parse_compound("4Mg3(PO4)2(s) + "), Ok((" + ", cmp)));
}

#[test]
fn compound_and_plus_simple() {
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "Na".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "Cl".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 1,
        ..Default::default()
    };
    assert_eq!(compound_and_plus("NaCl + "), Ok(("", cmp)));
}

#[test]
fn compound_and_plus_two_compounds() {
    let cmp = Compound {
        elements: vec![
            SimpleElement {
                name: "Na".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
            SimpleElement {
                name: "Cl".to_owned(),
                count: 1,
            }
            .into_element()
            .unwrap(),
        ],
        coefficient: 1,
        ..Default::default()
    };
    assert_eq!(compound_and_plus("NaCl + Mg(OH)2"), Ok(("Mg(OH)2", cmp)));
}

#[test]
fn one_side() {
    let cmp = vec![
        Compound {
            elements: vec![
                SimpleElement {
                    name: "Na".to_owned(),
                    count: 1,
                }
                .into_element()
                .unwrap(),
                SimpleElement {
                    name: "Cl".to_owned(),
                    count: 1,
                }
                .into_element()
                .unwrap(),
            ],
            coefficient: 1,
            ..Default::default()
        },
        Compound {
            elements: vec![
                SimpleElement {
                    name: "Mg".to_owned(),
                    count: 1,
                }
                .into_element()
                .unwrap(),
                SimpleElement {
                    name: "O".to_owned(),
                    count: 2,
                }
                .into_element()
                .unwrap(),
                SimpleElement {
                    name: "H".to_owned(),
                    count: 2,
                }
                .into_element()
                .unwrap(),
            ],
            coefficient: 1,
            ..Default::default()
        },
    ];
    assert_eq!(parse_side("NaCl + Mg(OH)2"), Ok(("", cmp)));
}

#[test]
fn one_side_three_comps() {
    let cmp = vec![
        Compound {
            elements: vec![
                SimpleElement {
                    name: "Na".to_owned(),
                    count: 1,
                }
                .into_element()
                .unwrap(),
                SimpleElement {
                    name: "Cl".to_owned(),
                    count: 1,
                }
                .into_element()
                .unwrap(),
            ],
            coefficient: 1,
            state: Some(State::Aqueous),
            ..Default::default()
        },
        Compound {
            elements: vec![
                SimpleElement {
                    name: "Mg".to_owned(),
                    count: 1,
                }
                .into_element()
                .unwrap(),
                SimpleElement {
                    name: "O".to_owned(),
                    count: 2,
                }
                .into_element()
                .unwrap(),
                SimpleElement {
                    name: "H".to_owned(),
                    count: 2,
                }
                .into_element()
                .unwrap(),
            ],
            coefficient: 1,
            state: Some(State::Solid),
            ..Default::default()
        },
        Compound {
            elements: vec![SimpleElement {
                name: "O".to_owned(),
                count: 2,
            }
            .into_element()
            .unwrap()],
            coefficient: 1,
            state: Some(State::Gas),
            ..Default::default()
        },
    ];
    assert_eq!(parse_side("NaCl(aq) + Mg(OH)2(s) + O2(g)"), Ok(("", cmp)));
}

#[test]
fn simple_eq() {
    let eq = Equation {
        left: vec![
            Compound {
                elements: vec![SimpleElement {
                    name: "O".to_owned(),
                    count: 2,
                }
                .into_element()
                .unwrap()],
                coefficient: 3,
                ..Default::default()
            },
            Compound {
                elements: vec![SimpleElement {
                    name: "Fe".to_owned(),
                    count: 1,
                }
                .into_element()
                .unwrap()],
                coefficient: 4,
                ..Default::default()
            },
        ],
        right: vec![Compound {
            elements: vec![
                SimpleElement {
                    name: "Fe".to_owned(),
                    count: 2,
                }
                .into_element()
                .unwrap(),
                SimpleElement {
                    name: "O".to_owned(),
                    count: 3,
                }
                .into_element()
                .unwrap(),
            ],
            coefficient: 2,
            ..Default::default()
        }],
        direction: Direction::Right,
        equation: "3O2 + 4Fe -> 2Fe2O3".to_owned(),
        ..Default::default()
    };

    assert_eq!(parse_equation("3O2 + 4Fe -> 2Fe2O3"), Ok(("", eq)));
}

#[test]
fn equation_unnecessary_brackets() {
    let eq = Equation {
        left: vec![
            Compound {
                elements: vec![
                    SimpleElement {
                        name: "Mg".to_owned(),
                        count: 1,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "O".to_owned(),
                        count: 2,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "H".to_owned(),
                        count: 2,
                    }
                    .into_element()
                    .unwrap(),
                ],
                coefficient: 1,
                ..Default::default()
            },
            Compound {
                elements: vec![SimpleElement {
                    name: "Fe".to_owned(),
                    count: 1,
                }
                .into_element()
                .unwrap()],
                coefficient: 1,
                ..Default::default()
            },
        ],
        right: vec![
            Compound {
                elements: vec![
                    SimpleElement {
                        name: "Fe".to_owned(),
                        count: 1,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "O".to_owned(),
                        count: 3,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "H".to_owned(),
                        count: 3,
                    }
                    .into_element()
                    .unwrap(),
                ],
                coefficient: 1,
                ..Default::default()
            },
            Compound {
                elements: vec![SimpleElement {
                    name: "Mg".to_owned(),
                    count: 1,
                }
                .into_element()
                .unwrap()],
                coefficient: 1,
                ..Default::default()
            },
        ],
        direction: Direction::Right,
        equation: "(Mg)(OH)2 + (Fe) -> (Fe)(OH)3 + (Mg)".to_owned(),
        ..Default::default()
    };

    assert_eq!(
        parse_equation("(Mg)(OH)2 + (Fe) -> (Fe)(OH)3 + (Mg)"),
        Ok(("", eq))
    );
}

#[test]
fn equation_combustion() {
    let eq = Equation {
        left: vec![
            Compound {
                elements: vec![
                    SimpleElement {
                        name: "C".to_owned(),
                        count: 2,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "H".to_owned(),
                        count: 6,
                    }
                    .into_element()
                    .unwrap(),
                ],
                coefficient: 1,
                ..Default::default()
            },
            Compound {
                elements: vec![SimpleElement {
                    name: "O".to_owned(),
                    count: 2,
                }
                .into_element()
                .unwrap()],
                coefficient: 1,
                ..Default::default()
            },
        ],
        right: vec![
            Compound {
                elements: vec![
                    SimpleElement {
                        name: "C".to_owned(),
                        count: 1,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "O".to_owned(),
                        count: 2,
                    }
                    .into_element()
                    .unwrap(),
                ],
                coefficient: 1,
                ..Default::default()
            },
            Compound {
                elements: vec![
                    SimpleElement {
                        name: "H".to_owned(),
                        count: 2,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "O".to_owned(),
                        count: 1,
                    }
                    .into_element()
                    .unwrap(),
                ],
                coefficient: 1,
                ..Default::default()
            },
        ],
        direction: Direction::Right,
        equation: "C2H6 + O2 -> CO2 + H2O".to_owned(),
        ..Default::default()
    };

    assert_eq!(parse_equation("C2H6 + O2 -> CO2 + H2O"), Ok(("", eq)));
}

#[test]
fn kitchen_sink() {
    let eq = Equation {
        left: vec![
            Compound {
                elements: vec![
                    SimpleElement {
                        name: "N".to_owned(),
                        count: 2,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "H".to_owned(),
                        count: 8,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "S".to_owned(),
                        count: 1,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "O".to_owned(),
                        count: 4,
                    }
                    .into_element()
                    .unwrap(),
                ],
                coefficient: 3,
                state: Some(State::Aqueous),
                ..Default::default()
            },
            Compound {
                elements: vec![
                    SimpleElement {
                        name: "Fe".to_owned(),
                        count: 3,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "P".to_owned(),
                        count: 2,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "O".to_owned(),
                        count: 8,
                    }
                    .into_element()
                    .unwrap(),
                ],
                coefficient: 1,
                state: Some(State::Solid),
                ..Default::default()
            },
        ],
        right: vec![
            Compound {
                elements: vec![
                    SimpleElement {
                        name: "N".to_owned(),
                        count: 3,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "H".to_owned(),
                        count: 12,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "P".to_owned(),
                        count: 1,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "O".to_owned(),
                        count: 4,
                    }
                    .into_element()
                    .unwrap(),
                ],
                coefficient: 2,
                state: Some(State::Aqueous),
                ..Default::default()
            },
            Compound {
                elements: vec![
                    SimpleElement {
                        name: "Fe".to_owned(),
                        count: 1,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "S".to_owned(),
                        count: 1,
                    }
                    .into_element()
                    .unwrap(),
                    SimpleElement {
                        name: "O".to_owned(),
                        count: 4,
                    }
                    .into_element()
                    .unwrap(),
                ],
                coefficient: 3,
                state: Some(State::Aqueous),
                ..Default::default()
            },
        ],
        direction: Direction::Left,
        equation: "3(NH4)2SO4(aq) + Fe3(PO4)2(s) <- 2(NH4)3PO4(aq) + 3FeSO4(aq)".to_owned(),
        ..Default::default()
    };

    assert_eq!(
        parse_equation("3(NH4)2SO4(aq) + Fe3(PO4)2(s) <- 2(NH4)3PO4(aq) + 3FeSO4(aq)"),
        Ok(("", eq))
    );
}
