#[derive(Debug)]
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

#[derive(Debug)]
pub enum CompareOp {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
}

#[derive(Debug)]
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

#[derive(Debug)]
/// Modes of dice expressions.
///
/// - DiceThrow: just throw a dice
/// - DiceThrowCmp: throw a dice and compare with a number
/// - Shuffle: shuffle a list
pub enum DiceExpr {
    DiceThrow(Throw),
    DiceThrowCompare(ThrowCompare),
}
