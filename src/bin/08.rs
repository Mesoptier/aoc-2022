use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let width = grid[0].len();
    let height = grid.len();
    let mut visible_grid = vec![vec![false; width]; height];
    let mut visible_trees = 0;

    for x in 0..width {
        // Top -> Bottom
        if !visible_grid[0][x] {
            visible_grid[0][x] = true;
            visible_trees += 1;
        }

        let mut tree_height = grid[0][x];
        for y in 1..height {
            if tree_height == 9 {
                break;
            }

            if grid[y][x] > tree_height {
                tree_height = grid[y][x];

                if !visible_grid[y][x] {
                    visible_grid[y][x] = true;
                    visible_trees += 1;
                }
            }
        }

        // Bottom -> Top
        if !visible_grid[height - 1][x] {
            visible_grid[height - 1][x] = true;
            visible_trees += 1;
        }

        let mut tree_height = grid[height - 1][x];
        for y in (0..(height - 1)).rev() {
            if tree_height == 9 {
                break;
            }

            if grid[y][x] > tree_height {
                tree_height = grid[y][x];

                if !visible_grid[y][x] {
                    visible_grid[y][x] = true;
                    visible_trees += 1;
                }
            }
        }
    }

    for y in 0..height {
        // Left -> Right
        if !visible_grid[y][0] {
            visible_grid[y][0] = true;
            visible_trees += 1;
        }

        let mut tree_height = grid[y][0];
        for x in 1..width {
            if tree_height == 9 {
                break;
            }

            if grid[y][x] > tree_height {
                tree_height = grid[y][x];

                if !visible_grid[y][x] {
                    visible_grid[y][x] = true;
                    visible_trees += 1;
                }
            }
        }

        // Right -> Left
        if !visible_grid[y][width - 1] {
            visible_grid[y][width - 1] = true;
            visible_trees += 1;
        }

        let mut tree_height = grid[y][width - 1];
        for x in (0..(width - 1)).rev() {
            if tree_height == 9 {
                break;
            }

            if grid[y][x] > tree_height {
                tree_height = grid[y][x];

                if !visible_grid[y][x] {
                    visible_grid[y][x] = true;
                    visible_trees += 1;
                }
            }
        }
    }

    Some(visible_trees)
}

pub fn part_two(input: &str) -> Option<usize> {
    let trees = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let width = trees[0].len();
    let height = trees.len();

    let mut max_scenic_score = 0;

    for y in 0..height {
        for x in 0..width {
            let tree_height = trees[y][x];
            let mut scenic_score = 1;

            // Looking up
            scenic_score *= (0..y)
                .rev()
                .map(|ny| trees[ny][x])
                .fold_while(0, |view_distance, other_tree_height| {
                    if other_tree_height >= tree_height {
                        Done(view_distance + 1)
                    } else {
                        Continue(view_distance + 1)
                    }
                })
                .into_inner();

            // Looking down
            scenic_score *= ((y + 1)..height)
                .map(|ny| trees[ny][x])
                .fold_while(0, |view_distance, other_tree_height| {
                    if other_tree_height >= tree_height {
                        Done(view_distance + 1)
                    } else {
                        Continue(view_distance + 1)
                    }
                })
                .into_inner();

            // Looking left
            scenic_score *= (0..x)
                .rev()
                .map(|nx| trees[y][nx])
                .fold_while(0, |view_distance, other_tree_height| {
                    if other_tree_height >= tree_height {
                        Done(view_distance + 1)
                    } else {
                        Continue(view_distance + 1)
                    }
                })
                .into_inner();

            // Looking right
            scenic_score *= ((x + 1)..width)
                .map(|nx| trees[y][nx])
                .fold_while(0, |view_distance, other_tree_height| {
                    if other_tree_height >= tree_height {
                        Done(view_distance + 1)
                    } else {
                        Continue(view_distance + 1)
                    }
                })
                .into_inner();

            max_scenic_score = max_scenic_score.max(scenic_score);
        }
    }

    Some(max_scenic_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
