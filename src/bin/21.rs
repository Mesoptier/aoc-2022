use std::fmt::Display;

use hashbrown::HashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

#[derive(Clone, Debug)]
enum OperationSpec {
    Number(usize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[derive(Clone, Debug)]
enum Operation {
    Unknown,
    Number(usize),
    Add(Box<Operation>, Box<Operation>),
    Sub(Box<Operation>, Box<Operation>),
    Mul(Box<Operation>, Box<Operation>),
    Div(Box<Operation>, Box<Operation>),
}

impl Operation {
    fn eval(&self) -> usize {
        match self {
            Operation::Unknown => unreachable!(),
            Operation::Number(n) => *n,
            Operation::Add(left, right) => left.eval() + right.eval(),
            Operation::Sub(left, right) => left.eval() - right.eval(),
            Operation::Mul(left, right) => left.eval() * right.eval(),
            Operation::Div(left, right) => left.eval() / right.eval(),
        }
    }

    fn simplify(&self) -> Self {
        let (left, right) = match self {
            Operation::Unknown => {
                return Operation::Unknown;
            }
            Operation::Number(n) => {
                return Operation::Number(*n);
            }
            Operation::Add(left, right)
            | Operation::Sub(left, right)
            | Operation::Mul(left, right)
            | Operation::Div(left, right) => (left, right),
        };

        let left = left.simplify();
        let right = right.simplify();

        if let (Operation::Number(left), Operation::Number(right)) = (&left, &right) {
            return match self {
                Operation::Unknown | Operation::Number(_) => unreachable!(),
                Operation::Add(_, _) => Operation::Number(left + right),
                Operation::Sub(_, _) => Operation::Number(left - right),
                Operation::Mul(_, _) => Operation::Number(left * right),
                Operation::Div(_, _) => Operation::Number(left / right),
            };
        }

        return match self {
            Operation::Unknown | Operation::Number(_) => unreachable!(),
            Operation::Add(_, _) => Operation::Add(left.into(), right.into()),
            Operation::Sub(_, _) => Operation::Sub(left.into(), right.into()),
            Operation::Mul(_, _) => Operation::Mul(left.into(), right.into()),
            Operation::Div(_, _) => Operation::Div(left.into(), right.into()),
        };
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Unknown => write!(f, "x"),
            Operation::Number(n) => write!(f, "{}", n),
            Operation::Add(left, right) => write!(f, "({} + {})", left, right),
            Operation::Sub(left, right) => write!(f, "({} - {})", left, right),
            Operation::Mul(left, right) => write!(f, "({} * {})", left, right),
            Operation::Div(left, right) => write!(f, "({} / {})", left, right),
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(String, OperationSpec)>> {
    separated_list0(
        line_ending,
        separated_pair(map(alpha1, String::from), tag(": "), parse_operation),
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, OperationSpec> {
    alt((
        map(map_res(digit1, str::parse), OperationSpec::Number),
        map(
            separated_pair(
                map(alpha1, String::from),
                tag(" + "),
                map(alpha1, String::from),
            ),
            |(a, b)| OperationSpec::Add(a, b),
        ),
        map(
            separated_pair(
                map(alpha1, String::from),
                tag(" - "),
                map(alpha1, String::from),
            ),
            |(a, b)| OperationSpec::Sub(a, b),
        ),
        map(
            separated_pair(
                map(alpha1, String::from),
                tag(" * "),
                map(alpha1, String::from),
            ),
            |(a, b)| OperationSpec::Mul(a, b),
        ),
        map(
            separated_pair(
                map(alpha1, String::from),
                tag(" / "),
                map(alpha1, String::from),
            ),
            |(a, b)| OperationSpec::Div(a, b),
        ),
    ))(input)
}

fn parse_operation_tree(input: &str, humn_is_unknown: bool) -> Operation {
    let operation_specs = parse_input(input).unwrap().1;
    let operation_specs = HashMap::<String, OperationSpec>::from_iter(operation_specs);

    fn eval(
        name: String,
        specs: &HashMap<String, OperationSpec>,
        humn_is_unknown: bool,
    ) -> Operation {
        let eval = |name: String| -> Box<Operation> { eval(name, specs, humn_is_unknown).into() };
        match specs.get(&name).unwrap().clone() {
            _ if name == "humn" && humn_is_unknown => Operation::Unknown,
            OperationSpec::Number(n) => Operation::Number(n),
            OperationSpec::Add(a, b) => Operation::Add(eval(a), eval(b)),
            OperationSpec::Sub(a, b) => Operation::Sub(eval(a), eval(b)),
            OperationSpec::Mul(a, b) => Operation::Mul(eval(a), eval(b)),
            OperationSpec::Div(a, b) => Operation::Div(eval(a), eval(b)),
        }
    }

    eval("root".to_string(), &operation_specs, humn_is_unknown)
}

pub fn part_one(input: &str) -> Option<usize> {
    let root = parse_operation_tree(input, false);
    Some(root.eval())
}

pub fn part_two(input: &str) -> Option<usize> {
    let root = parse_operation_tree(input, true);

    let (left, right) = match root {
        Operation::Unknown | Operation::Number(_) => unreachable!(),
        Operation::Add(left, right)
        | Operation::Sub(left, right)
        | Operation::Mul(left, right)
        | Operation::Div(left, right) => (left, right),
    };

    fn solve_eq(formula: Operation, c: usize) -> usize {
        match formula {
            Operation::Unknown => c,
            Operation::Number(_) => unreachable!(),
            Operation::Add(left, right) => match (*left, *right) {
                // (a + x = c) | (a + x = c) => x = c - a
                (Operation::Number(a), x) | (x, Operation::Number(a)) => solve_eq(x, c - a),
                _ => unreachable!(),
            },
            Operation::Sub(left, right) => match (*left, *right) {
                // (a - x = c) => x = a - c
                (Operation::Number(a), x) => solve_eq(x, a - c),
                // (x - a = c) => x = c + a
                (x, Operation::Number(a)) => solve_eq(x, c + a),
                _ => unreachable!(),
            },
            Operation::Mul(left, right) => match (*left, *right) {
                // (a * x = c) | (a * x = c) => x = c / a
                (Operation::Number(a), x) | (x, Operation::Number(a)) => solve_eq(x, c / a),
                _ => unreachable!(),
            },
            Operation::Div(left, right) => match (*left, *right) {
                // (a / x = c) => x = a / c
                (Operation::Number(a), x) => solve_eq(x, a / c),
                // (x / a = c) => x = c * a
                (x, Operation::Number(a)) => solve_eq(x, c * a),
                _ => unreachable!(),
            },
        }
    }

    let (formula, constant) = match (left.simplify(), right.simplify()) {
        (formula, Operation::Number(constant)) => (formula, constant),
        (Operation::Number(constant), formula) => (formula, constant),
        _ => unreachable!(),
    };

    Some(solve_eq(formula, constant))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
