#[derive(Debug)]
struct Node {
    value: i64,
    prev_idx: usize,
    next_idx: usize,
}

pub fn solve(input: &str, decryption_key: i64, mix_count: usize) -> Option<i64> {
    let file = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let len = file.len();
    let mut nodes = file
        .into_iter()
        .enumerate()
        .map(|(idx, value)| Node {
            value: value * decryption_key,
            prev_idx: (idx + len - 1) % len,
            next_idx: (idx + 1) % len,
        })
        .collect::<Vec<_>>();

    for _ in 0..mix_count {
        for idx in 0..len {
            let Node {
                value,
                prev_idx,
                next_idx,
            } = nodes[idx];

            // Remove node from list
            nodes[prev_idx].next_idx = next_idx;
            nodes[next_idx].prev_idx = prev_idx;

            // Find new position
            let mut prev_idx = prev_idx;
            let mut next_idx = next_idx;
            for _ in 0..(value.abs() as usize % (len - 1)) {
                if value > 0 {
                    prev_idx = next_idx;
                    next_idx = nodes[next_idx].next_idx;
                } else {
                    next_idx = prev_idx;
                    prev_idx = nodes[prev_idx].prev_idx;
                };
            }

            // Insert node into new position
            nodes[prev_idx].next_idx = idx;
            nodes[next_idx].prev_idx = idx;
            nodes[idx].prev_idx = prev_idx;
            nodes[idx].next_idx = next_idx;
        }
    }

    let zero_idx = nodes.iter().position(|node| node.value == 0).unwrap();
    let mut mixed_file = Vec::with_capacity(len);

    let mut cur_idx = zero_idx;
    loop {
        mixed_file.push(nodes[cur_idx].value);
        cur_idx = nodes[cur_idx].next_idx;
        if cur_idx == zero_idx {
            break;
        }
    }

    Some(mixed_file[1000 % len] + mixed_file[2000 % len] + mixed_file[3000 % len])
}

pub fn part_one(input: &str) -> Option<i64> {
    solve(input, 1, 1)
}

pub fn part_two(input: &str) -> Option<i64> {
    solve(input, 811589153, 10)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
