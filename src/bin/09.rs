use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, (char, u32)> {
    separated_pair(anychar, tag(" "), map_res(digit1, str::parse::<u32>))(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let motions = input.lines().map(|line| parse_line(line).unwrap().1);

    let mut visited = HashSet::<(i32, i32)>::new();

    let mut h_pos = (0i32, 0i32);
    let mut t_pos = (0i32, 0i32);

    for (dir, steps) in motions {
        for _step in 0..steps {
            match dir {
                'L' => h_pos.0 -= 1,
                'R' => h_pos.0 += 1,
                'U' => h_pos.1 += 1,
                'D' => h_pos.1 -= 1,
                _ => unreachable!(),
            }

            let diff = ((h_pos.0 - t_pos.0), (h_pos.1 - t_pos.1));
            if diff.0.abs() >= 2 || (h_pos.1 - t_pos.1).abs() >= 2 {
                let normalized_diff = (diff.0.signum(), diff.1.signum());
                t_pos.0 += normalized_diff.0;
                t_pos.1 += normalized_diff.1;
            }

            visited.insert(t_pos);
        }
    }

    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
