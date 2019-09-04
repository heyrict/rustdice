use std::str;

use crate::lib::dice::{CompareOp, DiceExpr, Throw, ThrowCompare};

#[derive(Debug, Clone)]
enum Identifier {
    Number(String),
    Mode(char),
    Symbol(String),
    Delimiter,
}

struct ParsedIdentifers {
    identifiers: Vec<Identifier>,
    cache: Option<Identifier>,
}

impl ParsedIdentifers {
    pub fn new() -> ParsedIdentifers {
        ParsedIdentifers {
            identifiers: vec![],
            cache: None,
        }
    }

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
                            Identifier::Mode(_pushed_mode) => flush_with(Some(cloned_ident)),
                            Identifier::Symbol(pushed_string) => {
                                flush_with(Some(Identifier::Symbol(pushed_string)))
                            }
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
                            Identifier::Number(pushed_string) => {
                                flush_with(Some(Identifier::Number(pushed_string)))
                            }
                            Identifier::Symbol(pushed_string) => {
                                flush_with(Some(Identifier::Symbol(pushed_string)))
                            }
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
                            Identifier::Number(pushed_string) => {
                                flush_with(Some(Identifier::Number(pushed_string)))
                            }
                            Identifier::Mode(_pushed_mode) => flush_with(Some(cloned_ident)),
                            Identifier::Delimiter => flush_with(None),
                        }
                    }
                    // If cached identifier is a delimiter
                    Identifier::Delimiter => flush_with(None),
                }
            }
        }
    }

    pub fn flush(&mut self) {
        let stored_ident = self.cache.clone();
        if let Some(ident) = stored_ident {
            self.identifiers.push(ident);
            self.cache = None;
        }
    }

    pub fn parse(&self) -> DiceExpr {
        let mut times: Option<i32> = None;
        let mut mode: Option<char> = None;
        let mut faces: Option<i32> = None;
        let mut op: Option<CompareOp> = None;
        let mut value: Option<i32> = None;

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
                Identifier::Delimiter => {
                    continue;
                }
            }
        }
        assert_eq!(mode.unwrap_or_default().to_ascii_lowercase(), 'd');
        if let None = op {
            return DiceExpr::DiceThrow(Throw::new(faces.unwrap_or(6), times.unwrap_or(1)));
        }
        DiceExpr::DiceThrowCompare(ThrowCompare::new(
            Throw::new(faces.unwrap_or(6), times.unwrap_or(1)),
            op.unwrap(),
            value.unwrap_or(0),
        ))
    }
}

pub fn parse(expression: &str) -> DiceExpr {
    let mut store = ParsedIdentifers::new();
    let mut mode: Option<char> = None;

    for c in expression.chars() {
        if c == ' ' {
            store.push(&Identifier::Delimiter);
        // Handling numbers
        } else if c >= '0' && c <= '9' {
            let mut s = String::new();
            s.push(c);
            store.push(&Identifier::Number(s));
        // Handling chars
        } else if vec!['<', '=', '>', '!'].contains(&c) {
            let mut s = String::new();
            s.push(c);
            store.push(&Identifier::Symbol(s));
        } else if let None = mode {
            store.push(&Identifier::Mode(c));
            mode = Some(c);
        } else {
            println!("{:?}", store.identifiers);
            panic!("rustdice:parse_str: Multiple mode chars detected");
        }
    }
    store.flush();
    store.parse()
}
