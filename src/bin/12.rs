use std::collections::BinaryHeap;

fn parse_input(input: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut start_point = None;
    let mut end_point = None;

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start_point = Some((x, y));
                        0
                    }
                    'E' => {
                        end_point = Some((x, y));
                        25
                    }
                    'a'..='z' => (c as u8) - ('a' as u8),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (grid, start_point.unwrap(), end_point.unwrap())
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    point: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, start_point, end_point) = parse_input(input);

    let width = grid[0].len();
    let height = grid.len();

    let point_to_idx = |(x, y): (usize, usize)| width * y + x;

    let valid_neighbors = |(x, y): (usize, usize)| {
        let mut neighbors = vec![];
        if 0 < x {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x < width - 1 {
            neighbors.push((x + 1, y));
        }
        if y < height - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    };

    let mut dist = vec![usize::MAX; width * height];
    let mut heap = BinaryHeap::new();

    dist[point_to_idx(start_point)] = 0;
    heap.push(State {
        cost: 0,
        point: start_point,
    });

    while let Some(State { cost, point }) = heap.pop() {
        if point == end_point {
            return Some(cost);
        }

        if cost > dist[point_to_idx(point)] {
            continue;
        }

        let cur_height = grid[point.1][point.0];

        for next_point in valid_neighbors(point) {
            let next_height = grid[next_point.1][next_point.0];
            if next_height > cur_height + 1 {
                continue;
            }

            let next_cost = cost + 1;
            if next_cost < dist[point_to_idx(next_point)] {
                heap.push(State {
                    cost: next_cost,
                    point: next_point,
                });
                dist[point_to_idx(next_point)] = next_cost;
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, _, end_point) = parse_input(input);

    let width = grid[0].len();
    let height = grid.len();

    let point_to_idx = |(x, y): (usize, usize)| width * y + x;

    let valid_neighbors = |(x, y): (usize, usize)| {
        let mut neighbors = vec![];
        if 0 < x {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x < width - 1 {
            neighbors.push((x + 1, y));
        }
        if y < height - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    };

    let mut dist = vec![usize::MAX; width * height];
    let mut heap = BinaryHeap::new();

    dist[point_to_idx(end_point)] = 0;
    heap.push(State {
        cost: 0,
        point: end_point,
    });

    let mut min_cost = usize::MAX;

    while let Some(State { cost, point }) = heap.pop() {
        let cur_height = grid[point.1][point.0];

        if cur_height == 0 {
            // Reached a starting square
            min_cost = min_cost.min(cost);
            continue;
        }

        if cost > dist[point_to_idx(point)] {
            continue;
        }

        for next_point in valid_neighbors(point) {
            let next_height = grid[next_point.1][next_point.0];
            if next_height < cur_height - 1 {
                continue;
            }

            let next_cost = cost + 1;
            if next_cost < dist[point_to_idx(next_point)] {
                heap.push(State {
                    cost: next_cost,
                    point: next_point,
                });
                dist[point_to_idx(next_point)] = next_cost;
            }
        }
    }

    Some(min_cost)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
