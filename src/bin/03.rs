use itertools::Itertools;

fn parse_input(input: &str) -> impl Iterator<Item = Vec<u32>> + '_ {
    input.lines().map(|line| {
        line.chars()
            .map(|c| match c {
                'a'..='z' => (c as u32) - ('a' as u32) + 1,
                'A'..='Z' => (c as u32) - ('A' as u32) + 27,
                _ => unreachable!(),
            })
            .collect_vec()
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    parse_input(input)
        .map(|items| {
            let (left_items, right_items) = items.split_at(items.len() / 2);
            for item in left_items {
                if right_items.contains(item) {
                    return *item;
                }
            }
            unreachable!();
        })
        .sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    parse_input(input)
        .tuples()
        .map(|(items1, items2, items3)| {
            for item in items1 {
                if items2.contains(&item) {
                    if items3.contains(&item) {
                        return item;
                    }
                }
            }
            unreachable!();
        })
        .sum1()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
