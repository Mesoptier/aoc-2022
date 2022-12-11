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
    starting_items: Vec<usize>,
    operation: Operation,
    test_divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(usize),
    Mult(usize),
    Square,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(count(line_ending, 2), parse_monkey)(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tuple((tag("Monkey "), digit1, tag(":"), line_ending))(input)?;
    let (input, starting_items) = delimited(
        tag("  Starting items: "),
        separated_list0(tag(", "), map_res(digit1, str::parse::<usize>)),
        line_ending,
    )(input)?;
    let (input, operation) = delimited(
        tag("  Operation: new = old "),
        alt((
            map(
                preceded(tag("+ "), map_res(digit1, str::parse::<usize>)),
                Operation::Add,
            ),
            map(
                preceded(tag("* "), map_res(digit1, str::parse::<usize>)),
                Operation::Mult,
            ),
            value(Operation::Square, tag("* old")),
        )),
        line_ending,
    )(input)?;
    let (input, test_divisible_by) = delimited(
        tag("  Test: divisible by "),
        map_res(digit1, str::parse::<usize>),
        line_ending,
    )(input)?;
    let (input, if_true) = delimited(
        tag("    If true: throw to monkey "),
        map_res(digit1, str::parse::<usize>),
        line_ending,
    )(input)?;
    let (input, if_false) = preceded(
        tag("    If false: throw to monkey "),
        map_res(digit1, str::parse::<usize>),
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

fn solve(input: &str, num_rounds: usize, divisor: usize) -> Option<usize> {
    let (_, monkeys) = parse_input(input).unwrap();

    let mut monkeys_items = monkeys
        .iter()
        .map(|m| m.starting_items.clone())
        .collect::<Vec<_>>();

    let modulo: usize = monkeys.iter().map(|m| m.test_divisible_by).product();

    let mut inspections = vec![0; monkeys.len()];

    for _round in 0..num_rounds {
        for idx in 0..monkeys.len() {
            let monkey = &monkeys[idx];
            let monkey_items = std::mem::replace(&mut monkeys_items[idx], Vec::new());

            for item in monkey_items {
                inspections[idx] += 1;

                let item = match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Mult(x) => item * x,
                    Operation::Square => item * item,
                };
                let item = (item / divisor) % modulo;

                let target_idx = match item % monkey.test_divisible_by == 0 {
                    true => monkey.if_true,
                    false => monkey.if_false,
                };
                monkeys_items[target_idx].push(item);
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    Some(inspections[0] * inspections[1])
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 20, 3)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 10000, 1)
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
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
