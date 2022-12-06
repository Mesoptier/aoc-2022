fn solve(input: &str, marker_len: usize) -> Option<usize> {
    let input = input.as_bytes();

    // Map from character index ('a'=0, 'b'=1, ..., 'z'=25) to the number of
    // occurences in the last `marker_len` characters.
    let mut counts = [0; 26];
    // Number of duplicates in the last `marker_len` characters.
    let mut duplicates = 0;

    // Initalize `counts` and `duplicates`
    for c in &input[0..marker_len] {
        let idx = (*c as usize) - ('a' as usize);
        counts[idx] += 1;
        if counts[idx] >= 2 {
            duplicates += 1;
        }
    }

    // Process remainder of the input stream
    for pos in marker_len..input.len() {
        if duplicates == 0 {
            return Some(pos);
        }

        let old = input[pos - marker_len];
        let old_idx = (old as usize) - ('a' as usize);
        counts[old_idx] -= 1;
        if counts[old_idx] >= 1 {
            duplicates -= 1;
        }

        let new = input[pos];
        let new_idx = (new as usize) - ('a' as usize);
        counts[new_idx] += 1;
        if counts[new_idx] >= 2 {
            duplicates += 1;
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 14)
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
