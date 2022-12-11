use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res, value},
    multi::{count, separated_list0},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Monkey {
    starting_items: Vec<u32>,
    operation: Operation,
    test_divisible_by: u32,
    if_true: u32,
    if_false: u32,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(u32),
    Mult(u32),
    Square,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(count(line_ending, 2), parse_monkey)(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tuple((tag("Monkey "), digit1, tag(":"), line_ending))(input)?;
    let (input, starting_items) = delimited(
        tag("  Starting items: "),
        separated_list0(tag(", "), map_res(digit1, str::parse::<u32>)),
        line_ending,
    )(input)?;
    let (input, operation) = delimited(
        tag("  Operation: new = old "),
        alt((
            map(
                preceded(tag("+ "), map_res(digit1, str::parse::<u32>)),
                Operation::Add,
            ),
            map(
                preceded(tag("* "), map_res(digit1, str::parse::<u32>)),
                Operation::Mult,
            ),
            value(Operation::Square, tag("* old")),
        )),
        line_ending,
    )(input)?;
    let (input, test_divisible_by) = delimited(
        tag("  Test: divisible by "),
        map_res(digit1, str::parse::<u32>),
        line_ending,
    )(input)?;
    let (input, if_true) = delimited(
        tag("    If true: throw to monkey "),
        map_res(digit1, str::parse::<u32>),
        line_ending,
    )(input)?;
    let (input, if_false) = preceded(
        tag("    If false: throw to monkey "),
        map_res(digit1, str::parse::<u32>),
    )(input)?;

    Ok((
        input,
        Monkey {
            starting_items,
            operation,
            test_divisible_by,
            if_true,
            if_false,
        },
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, monkeys) = parse_input(input).unwrap();

    let mut monkeys_items = monkeys
        .iter()
        .map(|m| VecDeque::from_iter(m.starting_items.iter().cloned()))
        .collect::<Vec<_>>();

    let mut inspections = vec![0; monkeys.len()];

    for _round in 0..20 {
        for idx in 0..monkeys.len() {
            let monkey = &monkeys[idx];
            let monkey_items = std::mem::replace(&mut monkeys_items[idx], VecDeque::default());

            for item in monkey_items {
                inspections[idx] += 1;

                let item = match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Mult(x) => item * x,
                    Operation::Square => item * item,
                } / 3;

                let target_idx = match item % monkey.test_divisible_by == 0 {
                    true => monkey.if_true,
                    false => monkey.if_false,
                };
                monkeys_items[target_idx as usize].push_back(item);
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    Some(inspections[0] * inspections[1])
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
