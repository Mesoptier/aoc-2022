use itertools::Itertools;

fn both_parts(input: &str) -> Option<(u32, u32)> {
    let mut sums = input
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
        });

    let max_len = 3;

    let mut maximum_sums = sums.by_ref().take(max_len).collect_vec();
    maximum_sums.sort_unstable_by(|a, b| b.cmp(a));

    for sum in sums {
        if sum > maximum_sums[max_len - 1] {
            maximum_sums.pop();
            maximum_sums.push(sum);
            maximum_sums.sort_unstable_by(|a, b| b.cmp(a));
        }
    }

    maximum_sums
        .first()
        .cloned()
        .zip(maximum_sums.into_iter().sum1())
}

pub fn part_one(input: &str) -> Option<u32> {
    both_parts(input).map(|result| result.0)
}

pub fn part_two(input: &str) -> Option<u32> {
    both_parts(input).map(|result| result.1)
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
        assert_eq!(part_two(&input), None);
    }
}
