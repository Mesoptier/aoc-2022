use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<(usize, usize)>>> {
    separated_list0(
        line_ending,
        separated_list0(
            tag(" -> "),
            separated_pair(
                map_res(digit1, str::parse),
                tag(","),
                map_res(digit1, str::parse),
            ),
        ),
    )(input)
}

fn solve(input: &str) -> (Option<u32>, Option<u32>) {
    let (_, traces) = parse_input(input).unwrap();

    let mut grid = [[false; 1000]; 500];
    let mut y_abyss = 0;

    for trace in traces {
        for points in trace.windows(2) {
            let p1 = points[0];
            let p2 = points[1];

            let x_min = p1.0.min(p2.0);
            let x_max = p1.0.max(p2.0);
            let y_min = p1.1.min(p2.1);
            let y_max = p1.1.max(p2.1);

            y_abyss = y_abyss.max(y_max);

            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    grid[y][x] = true;
                }
            }
        }
    }

    let y_floor = y_abyss + 2;

    let mut result_part1 = None;

    for unit in 0.. {
        let mut x = 500;
        let mut y = 0;
        loop {
            if !grid[y + 1][x] {
                y = y + 1;
            } else if !grid[y + 1][x - 1] {
                x = x - 1;
                y = y + 1;
            } else if !grid[y + 1][x + 1] {
                x = x + 1;
                y = y + 1;
            } else {
                if y == 0 && x == 500 {
                    return (result_part1, Some(unit + 1));
                }

                grid[y][x] = true;
                break;
            }

            if result_part1 == None && y >= y_abyss {
                result_part1 = Some(unit);
            }

            if y == y_floor - 1 {
                grid[y][x] = true;
                break;
            }
        }
    }

    unreachable!()
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input).0
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input).1
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
