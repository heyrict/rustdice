use std::fmt;

/// Throw a dice with `faces` faces `times` times.
pub struct Throw {
    faces: i32,
    times: i32,
}

impl Throw {
    pub fn new(faces: i32, times: i32) -> Throw {
        Throw { faces, times }
    }
}

impl fmt::Display for Throw {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}D{}", self.times, self.faces)
    }
}

/// Shuffle several elements
pub struct Shuffle {
    elements: Vec<String>,
}

impl Shuffle {
    pub fn new(elements: Vec<String>) -> Shuffle {
        Shuffle { elements }
    }
}

impl fmt::Display for Shuffle {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "S {}", self.elements.join(" "))
    }
}

/// Comparison operations
pub enum CompareOp {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
}

impl fmt::Display for CompareOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match &self {
                CompareOp::Equal => "==",
                CompareOp::NotEqual => "!=",
                CompareOp::GreaterThan => ">",
                CompareOp::GreaterThanOrEqual => ">=",
                CompareOp::LessThan => "<",
                CompareOp::LessThanOrEqual => "<=",
            }
        )
    }
}

/// compare with value after `Throw`
pub struct ThrowCompare {
    throw: Throw,
    op: CompareOp,
    value: i32,
}

impl ThrowCompare {
    pub fn new(throw: Throw, op: CompareOp, value: i32) -> ThrowCompare {
        ThrowCompare { throw, op, value }
    }
}

impl fmt::Display for ThrowCompare {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}{}{}", self.throw, self.op, self.value)
    }
}

/// Modes of dice expressions.
///
/// - DiceThrow: just throw a dice
///
///   Usage: ([times])D([faces])
///   - 3D6: Throw a dice with 6 faces 3 times
///   - D: Throw a dice with 6 faces 1 time
///   
/// - DiceThrowCompare: throw a dice and compare with a number
///
///   Usage: ([times])D([faces])[=|<|<=|>|>=|!=|<>][number]
///   - 3D6 > 4: Throw a dice with 6 faces and compare with 4, repeat 3 times
///
/// - Shuffle: shuffle a list
///
///   Usage: S [valueA [valueB [...]]]
///   - S A B C: Shuffle list [A, B, C]
pub enum DiceExpr {
    DiceThrow(Throw),
    DiceThrowCompare(ThrowCompare),
    ElementsShuffle(Shuffle),
}

impl fmt::Display for DiceExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self {
            DiceExpr::DiceThrow(throw) => write!(f, "{}", throw),
            DiceExpr::DiceThrowCompare(throw_cmp) => write!(f, "{}", throw_cmp),
            DiceExpr::ElementsShuffle(shfl) => write!(f, "{}", shfl),
        }
    }
}
