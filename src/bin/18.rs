use std::collections::VecDeque;

use hashbrown::HashSet;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

type Coord = (usize, usize, usize);

fn parse_input(input: &str) -> IResult<&str, Vec<Coord>> {
    separated_list0(
        line_ending,
        tuple((
            map_res(digit1, str::parse),
            preceded(tag(","), map_res(digit1, str::parse)),
            preceded(tag(","), map_res(digit1, str::parse)),
        )),
    )(input)
}

fn adj_coords((x, y, z): Coord, (max_x, max_y, max_z): Coord) -> Vec<Coord> {
    let mut adj_coords = vec![];

    if x > 0 {
        adj_coords.push((x - 1, y, z));
    }
    if x + 1 < max_x {
        adj_coords.push((x + 1, y, z));
    }
    if y > 0 {
        adj_coords.push((x, y - 1, z));
    }
    if y + 1 < max_y {
        adj_coords.push((x, y + 1, z));
    }
    if z > 0 {
        adj_coords.push((x, y, z - 1));
    }
    if z + 1 < max_z {
        adj_coords.push((x, y, z + 1));
    }

    adj_coords
}

pub fn part_one(input: &str) -> Option<u32> {
    let coords = {
        let mut coords = parse_input(input).unwrap().1;
        for coord in &mut coords {
            coord.0 += 1;
            coord.1 += 1;
            coord.2 += 1;
        }
        coords
    };

    let (max_x, max_y, max_z) = coords
        .iter()
        .fold((0, 0, 0), |(max_x, max_y, max_z), &(x, y, z)| {
            (max_x.max(x + 2), max_y.max(y + 2), max_z.max(z + 2))
        });

    let mut grid = vec![vec![vec![false; max_z]; max_y]; max_x];
    for &(x, y, z) in &coords {
        grid[x][y][z] = true;
    }

    let mut surface_area = 0;

    for &(x, y, z) in &coords {
        for (adj_x, adj_y, adj_z) in adj_coords((x, y, z), (max_x, max_y, max_z)) {
            if !grid[adj_x][adj_y][adj_z] {
                surface_area += 1;
            }
        }
    }

    Some(surface_area)
}

pub fn part_two(input: &str) -> Option<u32> {
    let coords = {
        let mut coords = parse_input(input).unwrap().1;
        for coord in &mut coords {
            coord.0 += 1;
            coord.1 += 1;
            coord.2 += 1;
        }
        coords
    };

    let (max_x, max_y, max_z) = coords
        .iter()
        .fold((0, 0, 0), |(max_x, max_y, max_z), &(x, y, z)| {
            (max_x.max(x + 2), max_y.max(y + 2), max_z.max(z + 2))
        });

    let mut grid = vec![vec![vec![false; max_z]; max_y]; max_x];
    for &(x, y, z) in &coords {
        grid[x][y][z] = true;
    }

    let mut visited = HashSet::<Coord>::new();
    let mut queue = VecDeque::<Coord>::new();

    queue.push_back((0, 0, 0));
    visited.get_or_insert((0, 0, 0));

    let mut surface_area = 0;

    while let Some(coord) = queue.pop_front() {
        for (adj_x, adj_y, adj_z) in adj_coords(coord, (max_x, max_y, max_z)) {
            if grid[adj_x][adj_y][adj_z] {
                surface_area += 1;
            } else if !visited.contains(&(adj_x, adj_y, adj_z)) {
                visited.insert((adj_x, adj_y, adj_z));
                queue.push_back((adj_x, adj_y, adj_z));
            }
        }
    }

    Some(surface_area)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
