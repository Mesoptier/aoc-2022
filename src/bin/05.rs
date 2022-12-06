use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1, line_ending, multispace1},
    combinator::{map, map_res, value},
    multi::{many1_count, separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<(u32, u32, u32)>)> {
    separated_pair(parse_stacks, multispace1, parse_moves)(input)
}

fn parse_stacks(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, rows) = separated_list0(multispace1, parse_stack_row)(input)?;
    let (input, num_stacks) = many1_count(tuple((multispace1, digit1)))(input)?;

    let mut stacks = vec![vec![]; num_stacks];
    for row in rows.into_iter().rev() {
        for (index, c) in row.into_iter().enumerate() {
            if let Some(c) = c {
                stacks[index].push(c);
            }
        }
    }

    Ok((input, stacks))
}

fn parse_stack_row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(
        tag(" "),
        alt((
            value(None, tag("   ")),
            map(delimited(tag("["), anychar, tag("]")), Some),
        )),
    )(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
    separated_list0(
        line_ending,
        tuple((
            preceded(tag("move "), map_res(digit1, str::parse::<u32>)),
            preceded(tag(" from "), map_res(digit1, str::parse::<u32>)),
            preceded(tag(" to "), map_res(digit1, str::parse::<u32>)),
        )),
    )(input)
}

/// Returns mutable references to a pair of indices at once.
///
/// Panics if `index1 == index2` or either index is out of range.
fn get_pair_mut<T>(slice: &mut [T], (index1, index2): (usize, usize)) -> (&mut T, &mut T) {
    assert_ne!(index1, index2, "indices must not be the same");

    if index1 < index2 {
        let (left, right) = slice.split_at_mut(index2);
        (&mut left[index1], &mut right[0])
    } else {
        let (left, right) = slice.split_at_mut(index1);
        (&mut right[0], &mut left[index2])
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (_, (mut stacks, moves)) = parse_input(input).unwrap();

    for (amount, from, to) in moves {
        let (from_stack, to_stack) =
            get_pair_mut(&mut stacks, ((from - 1) as usize, (to - 1) as usize));

        for _ in 0..amount {
            to_stack.push(from_stack.pop().unwrap());
        }
    }

    let result: String = stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().to_owned())
        .collect();
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, (mut stacks, moves)) = parse_input(input).unwrap();

    for (amount, from, to) in moves {
        let (from_stack, to_stack) =
            get_pair_mut(&mut stacks, ((from - 1) as usize, (to - 1) as usize));

        assert!(from_stack.len() >= amount as usize);

        let start = from_stack.len() - ((amount) as usize);
        to_stack.extend(from_stack.drain(start..));
    }

    let result: String = stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().to_owned())
        .collect();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
