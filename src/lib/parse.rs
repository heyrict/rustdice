use std::str;

use crate::lib::dice::{CompareOp, DiceExpr, Shuffle, Throw, ThrowCompare};

#[derive(Debug, Clone)]
enum Identifier {
    Number(String),
    Mode(char),
    Symbol(String),
    Element(String),
    Delimiter,
}

struct ParsedIdentifers {
    identifiers: Vec<Identifier>,
    cache: Option<Identifier>,
}

impl ParsedIdentifers {
    /// Create a identifier parser
    pub fn new() -> ParsedIdentifers {
        ParsedIdentifers {
            identifiers: vec![],
            cache: None,
        }
    }

    /// Push an identifier to the parser
    pub fn push(&mut self, ident: &Identifier) {
        let cloned_ident = ident.clone();
        let cache_clone = self.cache.clone();

        match &cache_clone {
            None => {
                self.cache = Some(cloned_ident);
            }
            Some(cached_ident) => {
                let mut flush_with = |new_cache| {
                    self.flush();
                    self.cache = new_cache;
                };

                match cached_ident {
                    // If cached identifier is a number
                    Identifier::Number(prev_string) => {
                        match cloned_ident {
                            // if pushed identifier is a number, concatenate them
                            Identifier::Number(pushed_string) => {
                                let mut new_string = prev_string.clone();
                                new_string.push_str(pushed_string.as_str());
                                self.cache = Some(Identifier::Number(new_string));
                            }
                            // otherwise, update cache
                            Identifier::Mode(_)
                            | Identifier::Symbol(_)
                            | Identifier::Element(_) => flush_with(Some(cloned_ident)),
                            Identifier::Delimiter => flush_with(None),
                        }
                    }
                    // If cached identifier is a mode
                    Identifier::Mode(mode) => {
                        match cloned_ident {
                            // if pushed identifier is a mode, panic
                            Identifier::Mode(pushed_mode) => {
                                panic!(
                                    "rustdice:ParsedIdentifers:push: Multiple modes detected.\
                                     Current={} Pushed={}",
                                    mode, pushed_mode
                                );
                            }
                            // otherwise, update cache
                            Identifier::Number(_)
                            | Identifier::Symbol(_)
                            | Identifier::Element(_) => flush_with(Some(cloned_ident)),
                            Identifier::Delimiter => flush_with(None),
                        }
                    }
                    // If cached identifier is a symbol
                    Identifier::Symbol(prev_string) => {
                        match cloned_ident {
                            // if pushed identifier is a symbol, concatenate them
                            Identifier::Symbol(pushed_string) => {
                                let mut new_string = prev_string.clone();
                                new_string.push_str(pushed_string.as_str());
                                self.cache = Some(Identifier::Symbol(new_string));
                            }
                            // otherwise, update cache
                            Identifier::Number(_)
                            | Identifier::Mode(_)
                            | Identifier::Element(_) => flush_with(Some(cloned_ident)),
                            Identifier::Delimiter => flush_with(None),
                        }
                    }
                    // If cached identifier is a delimiter
                    Identifier::Delimiter => flush_with(None),
                    // If cached identifier is an element
                    Identifier::Element(prev_string) => {
                        match cloned_ident {
                            // If pushed identifier is an element, concatenate them
                            Identifier::Element(pushed_string) => {
                                let mut new_string = prev_string.clone();
                                new_string.push_str(pushed_string.as_str());
                                self.cache = Some(Identifier::Element(new_string));
                            }
                            // otherwise, update cache
                            Identifier::Number(_) | Identifier::Mode(_) | Identifier::Symbol(_) => {
                                flush_with(Some(cloned_ident))
                            }
                            Identifier::Delimiter => flush_with(None),
                        }
                    }
                }
            }
        }
    }

    /// Flush cache to stored identifiers
    pub fn flush(&mut self) {
        if let Some(ident) = self.cache.clone() {
            self.identifiers.push(ident);
            self.cache = None;
        }
    }

    /// Get a `DiceExpr` representation of the parsed identifiers
    pub fn parse(&self) -> DiceExpr {
        let mut times: Option<i32> = None;
        let mut mode: Option<char> = None;
        let mut faces: Option<i32> = None;
        let mut op: Option<CompareOp> = None;
        let mut value: Option<i32> = None;
        let mut elements: Vec<String> = vec![];

        for ident in self.identifiers.iter() {
            match ident {
                Identifier::Number(s) => {
                    let number: i32 = s.parse().unwrap();
                    if let None = times {
                        if let None = mode {
                            times = Some(number);
                            continue;
                        }
                    }
                    if let None = faces {
                        faces = Some(number);
                        continue;
                    }
                    if let None = value {
                        if let Some(_) = op {
                            value = Some(number);
                            break; // No more values expected
                        }
                    }
                }
                Identifier::Mode(c) => {
                    mode = Some(*c);
                }
                Identifier::Symbol(s) => {
                    let symbol = match s.as_str() {
                        "<>" | "!=" => CompareOp::NotEqual,
                        "=" | "==" | "===" => CompareOp::Equal,
                        ">" => CompareOp::GreaterThan,
                        ">=" => CompareOp::GreaterThanOrEqual,
                        "<" => CompareOp::LessThan,
                        "<=" => CompareOp::LessThanOrEqual,
                        _ => panic!(
                            "rustdice:ParsedIdentifers:parse Symbol `{}` unrecognised",
                            s
                        ),
                    };
                    op = Some(symbol);
                }
                Identifier::Delimiter => continue,
                Identifier::Element(e) => {
                    elements.push(e.clone());
                }
            }
        }

        match mode {
            None => {
                panic!("No mode is provided");
            }
            Some(mode) => match mode {
                'D' => {
                    // DiceThrow
                    return if let None = op {
                        DiceExpr::DiceThrow(Throw::new(faces.unwrap_or(6), times.unwrap_or(1)))
                    } else {
                        // DiceThrowCompare
                        DiceExpr::DiceThrowCompare(ThrowCompare::new(
                            Throw::new(faces.unwrap_or(6), times.unwrap_or(1)),
                            op.unwrap(),
                            value.unwrap_or(0),
                        ))
                    };
                }
                'S' => {
                    // Shuffle
                    DiceExpr::ElementsShuffle(Shuffle::new(elements))
                }
                _ => {
                    panic!(
                        "parse:ParsedIdentifers:parse: Unrecognised mode: `{}`",
                        mode
                    );
                }
            },
        }
    }
}

pub fn parse(expression: &str) -> DiceExpr {
    let mut store = ParsedIdentifers::new();
    let mut mode: Option<char> = None; // Upper-case character

    for c in expression.chars() {
        // A space always function as a delimiter
        if c == ' ' {
            store.push(&Identifier::Delimiter);
            continue;
        }

        // Check if the mode exists.
        // If mode is `Shuffle`, all following characters are considered `Element`.
        if let Some(mode_char) = mode {
            match mode_char {
                'S' => {
                    let mut s = String::new();
                    s.push(c);
                    store.push(&Identifier::Element(s));
                    continue;
                }
                'D' => {}
                _ => {
                    panic!("rustdice:parse_str: Unrecognized mode `{}`", mode_char);
                }
            }
        } else {
            // Handling numbers
            if c >= '0' && c <= '9' {
                let mut s = String::new();
                s.push(c);
                store.push(&Identifier::Number(s));
            // Handling chars
            } else if vec!['<', '=', '>', '!'].contains(&c) {
                let mut s = String::new();
                s.push(c);
                store.push(&Identifier::Symbol(s));
            } else if let None = mode {
                store.push(&Identifier::Mode(c.to_ascii_uppercase()));
                mode = Some(c);
            } else {
                panic!("rustdice:parse_str: Multiple mode chars detected");
            }
        }
    }
    store.flush();
    store.parse()
}
