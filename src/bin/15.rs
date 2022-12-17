use advent_of_code::helpers::parse_signed_int;
use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, Vec<((i32, i32), (i32, i32))>> {
    separated_list0(
        line_ending,
        separated_pair(
            preceded(tag("Sensor at "), parse_coord),
            tag(": "),
            preceded(tag("closest beacon is at "), parse_coord),
        ),
    )(input)
}

fn parse_coord(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), parse_signed_int),
        tag(", "),
        preceded(tag("y="), parse_signed_int),
    )(input)
}

fn solve_part_one(input: &str, target_y: i32) -> Option<u32> {
    let (_, input) = parse_input(input).unwrap();

    let mut x_intervals = Vec::with_capacity(input.len());

    for (sensor, closest_beacon) in input {
        let (x, y) = sensor;
        let (bx, by) = closest_beacon;
        let diamond_size = (bx - x).abs() + (by - y).abs();
        let interval_size = (diamond_size - (target_y - y).abs()).abs();

        x_intervals.push((x - interval_size, x + interval_size));
    }

    x_intervals.sort();

    let mut x_cur = x_intervals[0].0;
    let mut result = 0;

    for (x_min, x_max) in x_intervals {
        let x_min = x_min.max(x_cur);

        if x_min <= x_max {
            result += x_min.abs_diff(x_max);
        }

        x_cur = x_cur.max(x_max);
    }

    Some(result)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve_part_one(input, 2000000)
}

pub fn solve_part_two(input: &str, size: u32) -> Option<u64> {
    let (_, input) = parse_input(input).unwrap();
    let sensors: Vec<((u32, u32), u32)> = input
        .into_iter()
        .map(|((x, y), (bx, by))| {
            let range = x.abs_diff(bx) + y.abs_diff(by);
            ((x as u32, y as u32), range)
        })
        .collect();

    for y in 0..=size {
        let mut x_intervals = vec![];

        for &((sx, sy), range) in &sensors {
            let y_diff = y.abs_diff(sy);
            if y_diff <= range {
                let interval_size = range - y_diff;
                x_intervals.push((
                    sx.saturating_sub(interval_size),
                    (sx + interval_size).min(size),
                ));
            }
        }

        x_intervals.sort();

        let mut x = 0;
        for (x_min, x_max) in x_intervals {
            if x < x_min {
                return Some((x as u64) * 4000000 + y as u64);
            }
            x = x.max(x_max + 1);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_part_two(input, 4000000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(solve_part_one(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(solve_part_two(&input, 20), Some(56000011));
    }
}
