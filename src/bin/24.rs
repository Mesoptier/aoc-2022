use std::{
    collections::BinaryHeap,
    ops::{Add, Rem, Sub},
};

use hashbrown::HashSet;

trait ModAdd<Rhs = Self, Mod = Self> {
    type Output;

    fn mod_add(self, rhs: Rhs, modulus: Mod) -> Self::Output;
}

impl<T> ModAdd for T
where
    T: Add<Output = T> + Rem<Output = T>,
{
    type Output = T;

    fn mod_add(self, rhs: Self, modulus: Self) -> Self::Output {
        (self + rhs) % modulus
    }
}

trait ModSub<Rhs = Self, Mod = Self> {
    type Output;

    fn mod_sub(self, rhs: Rhs, modulus: Mod) -> Self::Output;
}

impl<T> ModSub for T
where
    T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + Copy,
{
    type Output = T;

    fn mod_sub(self, rhs: Self, modulus: Self) -> Self::Output {
        (self + (modulus - rhs % modulus)) % modulus
    }
}

type Coord = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum Position {
    TopLeft,
    Valley(Coord),
    BottomRight,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct BlizzardGrid {
    width: usize,
    height: usize,
    blizzards: Vec<Vec<Option<Direction>>>,
}

impl BlizzardGrid {
    fn is_cell_empty(&self, (y, x): Coord, time: usize) -> bool {
        self.blizzards[y.mod_sub(time, self.height)][x] != Some(Direction::Down)
            && self.blizzards[y.mod_add(time, self.height)][x] != Some(Direction::Up)
            && self.blizzards[y][x.mod_sub(time, self.width)] != Some(Direction::Right)
            && self.blizzards[y][x.mod_add(time, self.width)] != Some(Direction::Left)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct State {
    time: usize,
    pos: Position,
}

#[derive(PartialEq, Eq)]
struct HeapEntry(usize, State);

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0).then_with(|| self.1.cmp(&other.1))
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_blizzard_grid(input: &str) -> BlizzardGrid {
    let blizzards: Vec<Vec<Option<Direction>>> = input
        .lines()
        .skip(1)
        .filter_map(|line| {
            if line.starts_with("##") {
                return None;
            }

            Some(
                line.chars()
                    .filter(|&c| c != '#')
                    .map(|c| match c {
                        '.' => None,
                        '^' => Some(Direction::Up),
                        'v' => Some(Direction::Down),
                        '<' => Some(Direction::Left),
                        '>' => Some(Direction::Right),
                        _ => unreachable!(),
                    })
                    .collect(),
            )
        })
        .collect();

    BlizzardGrid {
        width: blizzards[0].len(),
        height: blizzards.len(),
        blizzards,
    }
}

fn shortest_path(grid: &BlizzardGrid, start_state: State, goal_pos: Position) -> Option<usize> {
    let max_x = grid.width - 1;
    let max_y = grid.height - 1;

    let get_valid_neighbors = |pos: Position| match pos {
        Position::TopLeft => vec![Position::Valley((0, 0)), Position::TopLeft],
        Position::BottomRight => vec![Position::Valley((max_y, max_x)), Position::BottomRight],
        Position::Valley((y, x)) => {
            let mut neighbors = vec![];

            if y == max_y && x == max_x {
                neighbors.push(Position::BottomRight);
            }
            if y == 0 && x == 0 {
                neighbors.push(Position::TopLeft)
            }

            if x < max_x {
                neighbors.push(Position::Valley((y, x + 1)));
            }
            if y < max_y {
                neighbors.push(Position::Valley((y + 1, x)));
            }
            if x > 0 {
                neighbors.push(Position::Valley((y, x - 1)));
            }
            if y > 0 {
                neighbors.push(Position::Valley((y - 1, x)));
            }

            neighbors.push(Position::Valley((y, x)));

            neighbors
        }
    };

    let goal_dist = |pos: Position| {
        let dist = match pos {
            Position::BottomRight => 0,
            Position::Valley((y, x)) => (max_y - y) + (max_x - x) + 1,
            Position::TopLeft => max_y + max_x + 2,
        };

        match goal_pos {
            Position::TopLeft => max_y + max_x + 2 - dist,
            Position::BottomRight => dist,
            Position::Valley(_) => unreachable!(),
        }
    };

    let mut visited = HashSet::<State>::new();
    let mut heap = BinaryHeap::<HeapEntry>::new();

    visited.insert(start_state);
    heap.push(HeapEntry(goal_dist(goal_pos), start_state));

    while let Some(HeapEntry(_, state)) = heap.pop() {
        if state.pos == goal_pos {
            return Some(state.time);
        }

        for pos in get_valid_neighbors(state.pos) {
            let is_reachable = match pos {
                Position::TopLeft | Position::BottomRight => true,
                Position::Valley(coord) => grid.is_cell_empty(coord, state.time + 1),
            };
            if !is_reachable {
                continue;
            }

            let new_state = State {
                time: (state.time + 1),
                pos,
            };

            if visited.contains(&new_state) {
                continue;
            }

            visited.insert(new_state);
            heap.push(HeapEntry(
                new_state.time + goal_dist(new_state.pos),
                new_state,
            ));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_blizzard_grid(input);
    shortest_path(
        &grid,
        State {
            time: 0,
            pos: Position::TopLeft,
        },
        Position::BottomRight,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_blizzard_grid(input);
    let time = shortest_path(
        &grid,
        State {
            time: 0,
            pos: Position::TopLeft,
        },
        Position::BottomRight,
    )
    .unwrap();
    let time = shortest_path(
        &grid,
        State {
            time,
            pos: Position::BottomRight,
        },
        Position::TopLeft,
    )
    .unwrap();
    shortest_path(
        &grid,
        State {
            time,
            pos: Position::TopLeft,
        },
        Position::BottomRight,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
