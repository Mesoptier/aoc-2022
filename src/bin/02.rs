use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let c1 = line.chars().nth(0).unwrap();
            let c2 = line.chars().nth(2).unwrap();
            (c1, c2)
        })
        .map(|(c1, c2)| {
            let shape_score = match c2 {
                'X' => 1, // rock
                'Y' => 2, // paper
                'Z' => 3, // scissors
                _ => unreachable!(),
            };
            let outcome_score = match (c1, c2) {
                ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0, // loss
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3, // draw
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6, // win
                _ => unreachable!(),
            };
            shape_score + outcome_score
        })
        .sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let c1 = line.chars().nth(0).unwrap();
            let c2 = line.chars().nth(2).unwrap();
            (c1, c2)
        })
        .map(|(c1, c2)| {
            let shape_score = match (c1, c2) {
                ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1, // rock
                ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 2, // paper
                ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 3, // scissors
                _ => unreachable!(),
            };
            let outcome_score = match c2 {
                'X' => 0, // loss
                'Y' => 3, // draw
                'Z' => 6, // win
                _ => unreachable!(),
            };
            shape_score + outcome_score
        })
        .sum1()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
