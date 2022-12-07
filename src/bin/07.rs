use nom::{character::complete::digit1, combinator::map_res, IResult};

fn parse_size_prefix(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse::<usize>)(input)
}

fn process_input(input: &str) -> Vec<usize> {
    // ASSUMPTION: We never revisit a directory after leaving it.

    let mut sizes = vec![];
    let mut stack = vec![0];

    for line in input.lines() {
        if let Some(arg) = line.strip_prefix("$ cd ") {
            // `cd` command
            match arg {
                "/" => {
                    while stack.len() > 1 {
                        let size = stack.pop().unwrap();
                        sizes.push(size);
                        *stack.last_mut().unwrap() += size;
                    }
                }
                ".." => {
                    let size = stack.pop().unwrap();
                    sizes.push(size);
                    *stack.last_mut().unwrap() += size;
                }
                _ => {
                    stack.push(0);
                }
            }
        } else if let Ok((_, size)) = parse_size_prefix(line) {
            // `ls` file output
            *stack.last_mut().unwrap() += size;
        }
        // `ls` command and `ls` dir output are ignored
    }

    // Clean up sizes left on the stack
    while stack.len() > 1 {
        let size = stack.pop().unwrap();
        sizes.push(size);
        *stack.last_mut().unwrap() += size;
    }
    sizes.push(stack[0]);

    sizes
}

pub fn part_one(input: &str) -> Option<usize> {
    let sizes = process_input(input);
    let result = sizes.into_iter().filter(|size| *size <= 100_000).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let sizes = process_input(input);
    let root_size = sizes.last().unwrap();
    let target_size = root_size - (70_000_000 - 30_000_000);
    sizes.into_iter().filter(|size| *size >= target_size).min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
