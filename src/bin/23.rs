use auto_enums::auto_enum;
use hashbrown::{HashMap, HashSet};

fn get_bounds(positions: &HashSet<(i32, i32)>) -> (i32, i32, i32, i32) {
    positions.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_y, max_y, min_x, max_x), &(y, x)| {
            (min_y.min(y), max_y.max(y), min_x.min(x), max_x.max(x))
        },
    )
}

fn get_pattern((y, x): (i32, i32), positions: &HashSet<(i32, i32)>) -> u8 {
    let adjacent_positions = [
        (y - 1, x - 1),
        (y - 1, x),
        (y - 1, x + 1),
        (y, x - 1),
        (y, x + 1),
        (y + 1, x - 1),
        (y + 1, x),
        (y + 1, x + 1),
    ];

    let mut pattern = 0;
    for coord in adjacent_positions {
        pattern = (pattern << 1) + positions.contains(&coord) as u8;
    }
    pattern
}

fn print_positions(positions: &HashSet<(i32, i32)>) {
    let (min_y, max_y, min_x, max_x) = get_bounds(&positions);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if positions.contains(&(y, x)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let mut positions =
        HashSet::<(i32, i32)>::from_iter(input.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((y as i32, x as i32))
                } else {
                    None
                }
            })
        }));

    let mut rules: [(u8, (i32, i32)); 4] = [
        // 1's mark positions that should be empty, 0's mark ignored positions
        (0b11100000, (-1, 0)),
        (0b00000111, (1, 0)),
        (0b10010100, (0, -1)),
        (0b00101001, (0, 1)),
    ];

    let mut result_one = None;
    let mut result_two = None;

    for round in 0.. {
        if round == 10 {
            let num_elves = positions.len();
            let (min_y, max_y, min_x, max_x) = get_bounds(&positions);
            let num_tiles = (max_y - min_y + 1) * (max_x - min_x + 1);
            result_one = Some(num_tiles as usize - num_elves)
        }

        let mut proposed_positions = HashMap::<(i32, i32), Vec<(i32, i32)>>::new();

        for &(y, x) in &positions {
            let pattern = get_pattern((y, x), &positions);

            let new_coord = if pattern == 0 {
                (y, x)
            } else {
                rules
                    .iter()
                    .find_map(|(mask, (dy, dx))| {
                        if pattern & mask == 0 {
                            Some((y + dy, x + dx))
                        } else {
                            None
                        }
                    })
                    .unwrap_or((y, x))
            };

            proposed_positions
                .entry(new_coord)
                .or_default()
                .push((y, x));
        }

        #[auto_enum(Iterator)]
        fn flat_map_cb(
            (new_coord, old_coords): ((i32, i32), Vec<(i32, i32)>),
        ) -> impl Iterator<Item = (i32, i32)> {
            if old_coords.len() == 1 {
                std::iter::once(new_coord)
            } else {
                old_coords.into_iter()
            }
        }

        let next_positions =
            HashSet::from_iter(proposed_positions.into_iter().flat_map(flat_map_cb));

        if positions.is_subset(&next_positions) {
            result_two = Some(round + 1);
            break;
        }

        positions = next_positions;
        rules.rotate_left(1);
    }

    (result_one, result_two)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input).0
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input).1
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
