enum Push {
    Left,
    Right,
}

const ROCK_SHAPES: [[u8; 4]; 5] = [
    [0b01111000, 0b00000000, 0b00000000, 0b00000000],
    [0b00100000, 0b01110000, 0b00100000, 0b00000000],
    [0b01110000, 0b00010000, 0b00010000, 0b00000000],
    [0b01000000, 0b01000000, 0b01000000, 0b01000000],
    [0b01100000, 0b01100000, 0b00000000, 0b00000000],
];

fn print_rocks(chamber: &Vec<u8>, rock: &[u8; 4], rock_y: usize) {
    let height = chamber.len() + 7;

    for y in (0..height).rev() {
        let chamber_row = *chamber.get(y).unwrap_or(&0);
        let rock_row = if rock_y <= y && y < rock_y + 4 {
            rock[y - rock_y]
        } else {
            0
        };

        print!("|");
        for x in (0..7).rev() {
            if rock_row & (1 << x) != 0 {
                print!("@");
            } else if chamber_row & (1 << x) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");
    println!();
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut jet_pattern = input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Push::Left),
            '>' => Some(Push::Right),
            _ => None,
        })
        .cycle();

    let mut rock_shapes = ROCK_SHAPES.iter().cycle();

    let mut chamber: Vec<u8> = vec![];

    for _step in 0..2022 {
        let mut rock = *rock_shapes.next().unwrap();
        for row in &mut rock {
            *row = *row >> 2;
        }

        let mut y = chamber.len() + 3;

        //        print_rocks(&chamber, &rock, y);

        loop {
            match jet_pattern.next().unwrap() {
                Push::Left => {
                    let can_move_left = rock.iter().enumerate().all(|(dy, row)| {
                        if (row & (1 << 6)) != 0 {
                            false
                        } else if y + dy < chamber.len() {
                            chamber[y + dy] & (row << 1) == 0
                        } else {
                            true
                        }
                    });
                    if can_move_left {
                        for row in &mut rock {
                            *row = *row << 1;
                        }
                    }
                }
                Push::Right => {
                    let can_move_right = rock.iter().enumerate().all(|(dy, row)| {
                        if (row & 1) != 0 {
                            false
                        } else if y + dy < chamber.len() {
                            chamber[y + dy] & (row >> 1) == 0
                        } else {
                            true
                        }
                    });
                    if can_move_right {
                        for row in &mut rock {
                            *row = *row >> 1;
                        }
                    }
                }
            }

            let can_move_down = y > 0
                && rock.iter().enumerate().all(|(dy, row)| {
                    if y + dy - 1 < chamber.len() {
                        chamber[y + dy - 1] & row == 0
                    } else {
                        true
                    }
                });
            if can_move_down {
                y -= 1;
            } else {
                for (dy, &row) in rock.iter().enumerate() {
                    if row == 0 {
                        continue;
                    }

                    if y + dy >= chamber.len() {
                        chamber.resize(y + dy + 1, 0);
                    }

                    chamber[y + dy] |= row;
                }

                break;
            }
        }
    }

    Some(chamber.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
