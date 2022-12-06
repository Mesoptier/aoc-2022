use std::collections::VecDeque;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    input
        .chars()
        .tuple_windows()
        .find_position(|(a, b, c, d)| a != b && a != c && a != d && b != c && b != d && c != d)
        .map(|(position, _)| position + 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.as_bytes();
    let (input_head, input_tail) = input.split_at(14);

    let mut buffer = VecDeque::from_iter(input_head);

    let mut duplicates_in_buffer: usize = {
        let mut counts = [0; 26];

        for c in input_head {
            let i = c - ('a' as u8);
            counts[i as usize] += 1;
        }

        counts.into_iter().fold(0, |sum, n| {
            if n == 0 {
                sum
            } else {
                sum + n - 1
            }
        })
    };

    for (position, new_char) in input_tail.into_iter().enumerate() {
        if duplicates_in_buffer == 0 {
            return Some(position + 14);
        }

        let old_char = buffer.pop_front().unwrap();

        // Number of duplicates in buffer only changes if old_char and new_char are not the same
        if old_char != new_char {
            if buffer.contains(&old_char) {
                duplicates_in_buffer -= 1;
            }
            if buffer.contains(&new_char) {
                duplicates_in_buffer += 1;
            }
        }

        buffer.push_back(new_char);
    }

    unreachable!()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
