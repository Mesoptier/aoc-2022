use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;

fn solve(input: &str, max_len: usize) -> Option<u32> {
    input
        .lines()
        // Map to Option<u32>
        .map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.parse::<u32>().unwrap())
            }
        })
        // Sum sequential Some(u32)'s
        .batching(|it| match it.next() {
            None | Some(None) => None,
            Some(Some(mut sum)) => {
                while let Some(Some(x)) = it.next() {
                    sum += x;
                }
                Some(sum)
            }
        })
        // Record the `max_len` largest sums
        .fold(BinaryHeap::<Reverse<u32>>::new(), |mut min_heap, sum| {
            if min_heap.len() < max_len {
                min_heap.push(Reverse(sum));
            } else if min_heap.peek().unwrap().0 < sum {
                min_heap.pop();
                min_heap.push(Reverse(sum));
            }
            min_heap
        })
        .into_iter()
        .map(|rev| rev.0)
        .sum1()
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 3)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
