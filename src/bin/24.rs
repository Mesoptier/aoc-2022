use std::{
    collections::BinaryHeap,
    ops::{Add, Rem, Sub},
};

use hashbrown::{HashMap, HashSet};

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
    Goal,
    Valley(Coord),
    Start,
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

impl State {
    fn is_goal(&self) -> bool {
        self.pos == Position::Goal
    }
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

pub fn part_one(input: &str) -> Option<usize> {
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

    let grid = BlizzardGrid {
        width: blizzards[0].len(),
        height: blizzards.len(),
        blizzards,
    };

    let grid_width = grid.width;
    let grid_height = grid.height;

    let get_valid_neighbors = |pos: Position| match pos {
        Position::Start => vec![Position::Valley((0, 0)), Position::Start],
        Position::Goal => unreachable!(),
        Position::Valley((y, x)) => {
            let mut neighbors = vec![];

            if y + 1 == grid_height && x + 1 == grid_width {
                neighbors.push(Position::Goal);
            }

            if x + 1 < grid_width {
                neighbors.push(Position::Valley((y, x + 1)));
            }
            if y + 1 < grid_height {
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

    let goal_dist = |pos: Position| match pos {
        Position::Goal => 0,
        Position::Valley((y, x)) => (grid_height - y) + (grid_width - x) - 1,
        Position::Start => grid_width + grid_height,
    };

    let start_state = State {
        time: 0,
        pos: Position::Start,
    };

    let mut visited = HashSet::<State>::new();
    let mut heap = BinaryHeap::<HeapEntry>::new();

    visited.insert(start_state);
    heap.push(HeapEntry(goal_dist(Position::Start), start_state));

    while let Some(HeapEntry(cost, state)) = heap.pop() {
        if state.is_goal() {
            return Some(state.time);
        }

        for pos in get_valid_neighbors(state.pos) {
            let is_reachable = match pos {
                Position::Start | Position::Goal => true,
                Position::Valley(coord) => grid.is_cell_empty(coord, state.time + 1),
            };
            if !is_reachable {
                continue;
            }

            let new_state = State {
                time: (state.time + 1) % (grid_width * grid_height),
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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
