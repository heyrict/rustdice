use rand::{seq::SliceRandom, thread_rng, Rng};
use std::cmp::{PartialEq, PartialOrd};
use std::fmt;

pub trait GoDice<T> {
    fn gen_result(&self) -> T;
    fn get_result(&self, result: &T) -> String;
    fn go(&self) -> String {
        self.get_result(&self.gen_result())
    }
}

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

impl GoDice<Vec<i32>> for Throw {
    fn gen_result(&self) -> Vec<i32> {
        let mut results: Vec<i32> = vec![];
        let mut rng = thread_rng();
        for _ in 0..self.times {
            let result: i32 = rng.gen_range(1, self.faces + 1);
            results.push(result);
        }
        results
    }
    fn get_result(&self, result: &Vec<i32>) -> String {
        let result_string: Vec<String> = result
            .iter()
            .map(|r: &i32| -> String { format!("{}", r) })
            .collect();
        let result_str = result_string.join(" ");
        result_str
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

impl GoDice<Vec<String>> for Shuffle {
    fn gen_result(&self) -> Vec<String> {
        let mut result = self.elements.clone();
        let mut rng = thread_rng();
        result.shuffle(&mut rng);
        result
    }
    fn get_result(&self, result: &Vec<String>) -> String {
        result.join(" ")
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

impl CompareOp {
    pub fn exec<T>(&self, a: T, b: T) -> bool
    where
        T: PartialEq + PartialOrd,
    {
        match &self {
            CompareOp::Equal => a == b,
            CompareOp::NotEqual => a != b,
            CompareOp::GreaterThan => a > b,
            CompareOp::GreaterThanOrEqual => a >= b,
            CompareOp::LessThan => a < b,
            CompareOp::LessThanOrEqual => a <= b,
        }
    }
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

impl GoDice<Vec<i32>> for ThrowCompare {
    fn gen_result(&self) -> Vec<i32> {
        self.throw.gen_result()
    }
    fn get_result(&self, result: &Vec<i32>) -> String {
        let throw_result = self.throw.get_result(result);
        let compare_result: String = {
            let pass_result: Vec<i32> = result
                .iter()
                .map(|value| self.op.exec(value, &self.value) as i32)
                .collect();
            let pass_times: i32 = pass_result.iter().sum();
            format!("Pass {} of {}", pass_times, self.throw.times)
        };
        format!(
            "{} {} {}: {}",
            throw_result, self.op, self.value, compare_result
        )
    }
}

/// Modes of dice expressions.
///
/// - DiceThrow: just throw a dice
///
///   Usage: [times]D[faces]
///   - 3D6: Throw a dice with 6 faces 3 times
///   - D: Throw a dice with 6 faces 1 time
///
/// - DiceThrowCompare: throw a dice and compare with a number
///
///   Usage: [times]D[faces][=|<|<=|>|>=|!=|<>][number]
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

impl DiceExpr {
    pub fn go(&self) -> String {
        match &self {
            DiceExpr::DiceThrow(throw) => throw.go(),
            DiceExpr::DiceThrowCompare(throw_cmp) => throw_cmp.go(),
            DiceExpr::ElementsShuffle(shfl) => shfl.go(),
        }
    }
}
