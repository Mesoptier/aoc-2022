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
enum Operation {
    Number(usize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn parse_input(input: &str) -> IResult<&str, Vec<(String, Operation)>> {
    separated_list0(
        line_ending,
        separated_pair(map(alpha1, String::from), tag(": "), parse_operation),
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        map(map_res(digit1, str::parse), Operation::Number),
        map(
            separated_pair(
                map(alpha1, String::from),
                tag(" + "),
                map(alpha1, String::from),
            ),
            |(a, b)| Operation::Add(a, b),
        ),
        map(
            separated_pair(
                map(alpha1, String::from),
                tag(" - "),
                map(alpha1, String::from),
            ),
            |(a, b)| Operation::Sub(a, b),
        ),
        map(
            separated_pair(
                map(alpha1, String::from),
                tag(" * "),
                map(alpha1, String::from),
            ),
            |(a, b)| Operation::Mul(a, b),
        ),
        map(
            separated_pair(
                map(alpha1, String::from),
                tag(" / "),
                map(alpha1, String::from),
            ),
            |(a, b)| Operation::Div(a, b),
        ),
    ))(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let monkeys = parse_input(input).unwrap().1;
    let monkeys = HashMap::from_iter(monkeys);

    fn eval(
        name: String,
        values: &mut HashMap<String, usize>,
        monkeys: &HashMap<String, Operation>,
    ) -> usize {
        if let Some(&value) = values.get(&name) {
            return value;
        }

        let mut eval = |name: String| eval(name, values, monkeys);
        let value = match monkeys.get(&name).unwrap().clone() {
            Operation::Number(value) => value,
            Operation::Add(a, b) => eval(a) + eval(b),
            Operation::Sub(a, b) => eval(a) - eval(b),
            Operation::Mul(a, b) => eval(a) * eval(b),
            Operation::Div(a, b) => eval(a) / eval(b),
        };
        values.insert(name, value);
        value
    }

    let mut values: HashMap<String, usize> = HashMap::new();
    Some(eval("root".to_string(), &mut values, &monkeys))
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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
