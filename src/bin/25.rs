#[derive(Debug)]
struct ParseSnafuError;

fn parse_snafu(s: &str) -> Result<u64, ParseSnafuError> {
    let mut n = 0;
    for c in s.chars() {
        n = n * 5;
        n = match c {
            '2' => n + 2,
            '1' => n + 1,
            '0' => n + 0,
            '-' => n - 1,
            '=' => n - 2,
            _ => return Err(ParseSnafuError),
        };
    }
    Ok(n)
}

fn to_snafu_string(mut n: u64) -> String {
    let mut s = String::new();

    while n > 0 {
        s.push(match n % 5 {
            2 => '2',
            1 => '1',
            0 => '0',
            4 => '-',
            3 => '=',
            _ => unreachable!(),
        });
        n = match n % 5 {
            2 | 1 | 0 => n / 5,
            4 | 3 => n / 5 + 1,
            _ => unreachable!(),
        };
    }

    unsafe {
        // SAFETY: `s` consists only of single-byte characters, so reversing it is safe
        s.as_bytes_mut().reverse();
    };

    s
}

pub fn part_one(input: &str) -> Option<String> {
    Some(to_snafu_string(
        input.lines().map(|line| parse_snafu(line).unwrap()).sum(),
    ))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".into()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
