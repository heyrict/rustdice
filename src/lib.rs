use std::str;

#[derive(Debug)]
/// Throw a dice with `faces` faces `times` times.
pub struct Throw {
    faces: i32,
    times: i32,
}

impl Throw {
    pub fn new(faces: i32, times: i32) -> Throw {
        Throw {
            faces,
            times,
        }
    }
}

#[derive(Debug)]
/// Modes of dice expressions.
///
/// - DiceThrow: just throw a dice
/// - DiceThrowCmp: throw a dice and compare with a number
/// - Shuffle: shuffle a list
pub enum DiceExpr {
    DiceThrow(Throw),
}

#[derive(Debug, Clone)]
enum Identifier {
    Number(String),
    Mode(char),
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

        match &self.cache {
            None => {
                self.cache = Some(cloned_ident);
            }
            Some(cached_ident) => {
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
                            // if pushed identifier is a mode, update cache
                            Identifier::Mode(_pushed_mode) => {
                                self.flush();
                                self.cache = Some(cloned_ident);
                            }
                        }
                    }
                    // If cached identifier is a mode
                    Identifier::Mode(mode) => {
                        match cloned_ident {
                            // if pushed identifier is a number, update cache
                            Identifier::Number(pushed_string) => {
                                self.flush();
                                self.cache = Some(Identifier::Number(pushed_string));
                            }
                            // if pushed identifier is a mode, update cache
                            Identifier::Mode(pushed_mode) => {
                                panic!("rustdice:ParsedIdentifers:push: Multiple modes detected.\
                                Current={} Pushed={}", mode, pushed_mode);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn flush(&mut self) {
        let stored_ident = self.cache.clone().unwrap();
        self.identifiers.push(stored_ident);
        self.cache = None;
    }

    pub fn parse(&self) -> DiceExpr {
        let mut times: Option<i32> = None;
        let mut mode: Option<char> = None;
        let mut faces: Option<i32> = None;

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
                }
                Identifier::Mode(c) => {
                    mode = Some(*c);
                }
            }
        }
        assert_eq!(mode.unwrap_or_default().to_ascii_lowercase(), 'd');
        DiceExpr::DiceThrow(Throw::new(faces.unwrap_or(6), times.unwrap_or(1)))
    }
}

pub fn parse(expression: &str) -> DiceExpr {
    let mut store = ParsedIdentifers::new();
    let mut mode_exists = false;

    for c in expression.chars() {
        // Handling numbers
        if c as u8 >= '0' as u8 && c as u8 <= '9' as u8 {
            let mut s = String::new();
            s.push(c);
            store.push(&Identifier::Number(s));
        // Handling chars
        } else if !mode_exists {
            store.push(&Identifier::Mode(c));
            mode_exists = true;
        } else {
            panic!("rustdice:parse_str: Multiple mode chars detected");
        }
    }
    store.flush();
    store.parse()
}
