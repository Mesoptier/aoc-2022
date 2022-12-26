trait FromSnafu: Sized {
    type Err;

    fn from_snafu(s: &str) -> Result<Self, Self::Err>;
}

trait ToSnafu: Sized {
    fn to_snafu(self) -> String;
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSnafuError;

macro_rules! implement_snafu {
    ($($t:ty),*) => { $(
        impl FromSnafu for $t {
            type Err = ParseSnafuError;

            fn from_snafu(s: &str) -> Result<Self, Self::Err> {
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
        }

        impl ToSnafu for $t {
            fn to_snafu(mut self) -> String {
                let mut s = String::new();

                while self > 0 {
                    s.push(match self % 5 {
                        2 => '2',
                        1 => '1',
                        0 => '0',
                        4 => '-',
                        3 => '=',
                        _ => unreachable!(),
                    });
                    self = match self % 5 {
                        2 | 1 | 0 => self / 5,
                        4 | 3 => self / 5 + 1,
                        _ => unreachable!(),
                    };
                }

                unsafe {
                    // SAFETY: `s` consists only of single-byte characters, so reversing it is safe
                    s.as_bytes_mut().reverse();
                };

                s
            }
        }
    )* };
}

implement_snafu! { u8, u16, u32, u64 }

pub fn part_one(input: &str) -> Option<String> {
    Some(
        input
            .lines()
            .map(|line| u64::from_snafu(line).unwrap())
            .sum::<u64>()
            .to_snafu(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
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

#[cfg(test)]
mod tests_snafu {
    use super::*;

    macro_rules! from_snafu_tests {
        ($($name:ident: ($input:expr, $output:expr),)*) => { $(
            #[test]
            fn $name() {
                assert_eq!(u64::from_snafu($input), Ok($output));
            }
        )* };
    }

    from_snafu_tests! {
        from_snafu_case_1:  ("1=-0-2", 1747),
        from_snafu_case_2:  ( "12111",  906),
        from_snafu_case_3:  (  "2=0=",  198),
        from_snafu_case_4:  (    "21",   11),
        from_snafu_case_5:  (  "2=01",  201),
        from_snafu_case_6:  (   "111",   31),
        from_snafu_case_7:  ( "20012", 1257),
        from_snafu_case_8:  (   "112",   32),
        from_snafu_case_9:  ( "1=-1=",  353),
        from_snafu_case_10: (  "1-12",  107),
        from_snafu_case_11: (    "12",    7),
        from_snafu_case_12: (    "1=",    3),
        from_snafu_case_13: (   "122",   37),
    }

    macro_rules! to_snafu_tests {
        ($($name:ident: ($input:expr, $output:expr),)*) => { $(
            #[test]
            fn $name() {
                let v: u64 = $input;
                assert_eq!(v.to_snafu(), $output);
            }
        )* };
    }

    to_snafu_tests! {
        to_snafu_case_1:  (        1,              "1"),
        to_snafu_case_2:  (        2,              "2"),
        to_snafu_case_3:  (        3,             "1="),
        to_snafu_case_4:  (        4,             "1-"),
        to_snafu_case_5:  (        5,             "10"),
        to_snafu_case_6:  (        6,             "11"),
        to_snafu_case_7:  (        7,             "12"),
        to_snafu_case_8:  (        8,             "2="),
        to_snafu_case_9:  (        9,             "2-"),
        to_snafu_case_10: (       10,             "20"),
        to_snafu_case_11: (       15,            "1=0"),
        to_snafu_case_12: (       20,            "1-0"),
        to_snafu_case_13: (     2022,         "1=11-2"),
        to_snafu_case_14: (    12345,        "1-0---0"),
        to_snafu_case_15: (314159265,  "1121-1110-1=0"),
    }
}
