use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    separated_list0(line_ending, parse_pair)(input)
}

fn parse_pair(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    separated_pair(parse_range, tag(","), parse_range)(input)
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    map(
        separated_pair(
            map_res(digit1, str::parse::<u32>),
            tag("-"),
            map_res(digit1, str::parse::<u32>),
        ),
        |(start, end)| start..=end,
    )(input)
}

trait RangeIntersect<Rhs = Self> {
    type Output;
    fn intersect(self, rhs: Rhs) -> Self::Output;
}

impl<T> RangeIntersect for RangeInclusive<T>
where
    T: Ord,
{
    type Output = RangeInclusive<T>;

    fn intersect(self, rhs: Self) -> Self::Output {
        let (self_start, self_end) = self.into_inner();
        let (rhs_start, rhs_end) = rhs.into_inner();
        (self_start.max(rhs_start))..=(self_end.min(rhs_end))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, list) = parse_input(input).unwrap();

    let count = list
        .into_iter()
        .filter(|(first_range, second_range)| {
            let intersection = first_range.clone().intersect(second_range.clone());
            &intersection == first_range || &intersection == second_range
        })
        .count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, list) = parse_input(input).unwrap();

    let count = list
        .into_iter()
        .filter(|(first_range, second_range)| {
            let intersection = first_range.clone().intersect(second_range.clone());
            !intersection.is_empty()
        })
        .count();

    Some(count as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
