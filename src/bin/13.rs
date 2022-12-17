use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::{count, separated_list0},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum PacketValue {
    Integer(u32),
    List(Vec<PacketValue>),
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketValue::Integer(left), PacketValue::Integer(right)) => left.cmp(right),
            (PacketValue::List(left), PacketValue::List(right)) => {
                let mut left_it = left.iter();
                let mut right_it = right.iter();

                loop {
                    match (left_it.next(), right_it.next()) {
                        (Some(left), Some(right)) => {
                            let cmp = left.cmp(right);
                            if !cmp.is_eq() {
                                break cmp;
                            }
                        }
                        (None, None) => {
                            break std::cmp::Ordering::Equal;
                        }
                        (None, Some(_)) => {
                            break std::cmp::Ordering::Less;
                        }
                        (Some(_), None) => {
                            break std::cmp::Ordering::Greater;
                        }
                    }
                }
            }
            (PacketValue::Integer(_), PacketValue::List(_)) => {
                PacketValue::List(vec![self.clone()]).cmp(other)
            }
            (PacketValue::List(_), PacketValue::Integer(_)) => {
                self.cmp(&PacketValue::List(vec![other.clone()]))
            }
        }
    }
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(PacketValue, PacketValue)>> {
    separated_list0(
        count(line_ending, 2),
        separated_pair(parse_packet_value, line_ending, parse_packet_value),
    )(input)
}

fn parse_packet_value(input: &str) -> IResult<&str, PacketValue> {
    alt((
        map(map_res(digit1, str::parse::<u32>), PacketValue::Integer),
        map(
            delimited(
                tag("["),
                separated_list0(tag(","), parse_packet_value),
                tag("]"),
            ),
            PacketValue::List,
        ),
    ))(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, packet_pairs) = parse_input(input).unwrap();

    let mut result = 0;

    for (index, (left, right)) in packet_pairs.into_iter().enumerate() {
        if left < right {
            result += index + 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, packet_pairs) = parse_input(input).unwrap();
    let mut packets: Vec<PacketValue> = packet_pairs
        .into_iter()
        .flat_map(|(left, right)| [left, right])
        .collect();

    let divider_packet1 = PacketValue::List(vec![PacketValue::List(vec![PacketValue::Integer(2)])]);
    let divider_packet2 = PacketValue::List(vec![PacketValue::List(vec![PacketValue::Integer(6)])]);
    packets.push(divider_packet1.clone());
    packets.push(divider_packet2.clone());

    packets.sort();

    let mut result = 1;
    for (index, packet) in packets.into_iter().enumerate() {
        if packet == divider_packet1 || packet == divider_packet2 {
            result *= index + 1;
        }
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
