use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, opt, recognize, value},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Noop, tag("noop")),
        map(preceded(tag("addx "), parse_signed_int), Instruction::AddX),
    ))(input)
}

fn parse_signed_int<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    map_res(recognize(tuple((opt(tag("-")), digit1))), str::parse::<T>)(input)
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = input.lines().map(|line| parse_instruction(line).unwrap().1);

    let mut cycle = 0;
    let mut x = 1;

    let mut total_signal_strength = 0;

    for instruction in instructions {
        let duration = match instruction {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        };

        for _ in 0..duration {
            cycle += 1;

            if cycle % 40 == 20 {
                total_signal_strength += cycle * x;
            }
        }

        match instruction {
            Instruction::Noop => {}
            Instruction::AddX(v) => x += v,
        }
    }

    Some(total_signal_strength)
}

pub fn part_two(input: &str) -> Option<String> {
    let instructions = input.lines().map(|line| parse_instruction(line).unwrap().1);

    let mut cycle = 0;
    let mut sprite_x = 1;

    let mut screen = String::new();

    for instruction in instructions {
        let duration = match instruction {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        };

        for _ in 0..duration {
            let screen_x = cycle % 40;
            if sprite_x - 1 <= screen_x && screen_x <= sprite_x + 1 {
                screen.push('#');
            } else {
                screen.push('.');
            }

            cycle += 1;

            if cycle != 0 && cycle % 40 == 0 {
                screen.push('\n');
            }
        }

        match instruction {
            Instruction::Noop => {}
            Instruction::AddX(v) => sprite_x += v,
        }
    }

    Some(screen)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(
                "##..##..##..##..##..##..##..##..##..##..\n\
                 ###...###...###...###...###...###...###.\n\
                 ####....####....####....####....####....\n\
                 #####.....#####.....#####.....#####.....\n\
                 ######......######......######......####\n\
                 #######.......#######.......#######.....\n"
                    .to_string()
            )
        );
    }
}
