use hashbrown::{hash_map::Entry, HashMap};

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

pub fn part_two(input: &str) -> Option<usize> {
    let jet_pattern = input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Push::Left),
            '>' => Some(Push::Right),
            _ => None,
        })
        .collect::<Vec<_>>();
    let jet_pattern_len = jet_pattern.len();
    let mut jet_pattern = jet_pattern.iter().cycle();

    let mut rock_shapes = ROCK_SHAPES.iter().cycle();

    let mut chamber: Vec<u8> = vec![];

    let max_steps: usize = 1000000000000;
    let min_cycle_len = jet_pattern_len * 5;

    const SIGNATURE_LEN: usize = 20;
    let mut signatures = HashMap::<[u8; SIGNATURE_LEN], (usize, usize)>::new();

    let mut skipped_height = None;
    let mut step = 0;
    while step < max_steps {
        if step != 0 && step % min_cycle_len == 0 && skipped_height.is_none() {
            let height = chamber.len();
            let signature: [u8; SIGNATURE_LEN] = chamber[(chamber.len() - SIGNATURE_LEN)..]
                .try_into()
                .unwrap();
            match signatures.entry(signature) {
                Entry::Occupied(entry) => {
                    let (prev_step, prev_height) = *entry.get();
                    let diff_step = step - prev_step;
                    let diff_height = height - prev_height;

                    let remaining_steps = max_steps - step;
                    let skip_cycles = remaining_steps / diff_step;

                    step += skip_cycles * diff_step;
                    skipped_height = Some(skip_cycles * diff_height);
                }
                Entry::Vacant(entry) => {
                    entry.insert((step, height));
                }
            }
        }

        let mut rock = *rock_shapes.next().unwrap();
        for row in &mut rock {
            *row = *row >> 2;
        }

        let mut y = chamber.len() + 3;

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

        step += 1;
    }

    Some(chamber.len() + skipped_height.unwrap())
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
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
