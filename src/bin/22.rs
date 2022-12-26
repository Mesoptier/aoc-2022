use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res, value},
    multi::{count, many0_count, many1, separated_list0},
    sequence::{pair, separated_pair},
    IResult,
};

#[derive(Clone, Copy, Debug)]
enum Tile {
    Open,
    Wall,
}

#[derive(Clone, Copy, Debug)]
enum PathInstruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

fn parse_input(input: &str) -> IResult<&str, (Vec<(usize, Vec<Tile>)>, Vec<PathInstruction>)> {
    separated_pair(
        parse_board_rows,
        count(line_ending, 2),
        parse_path_instructions,
    )(input)
}

fn parse_board_rows(input: &str) -> IResult<&str, Vec<(usize, Vec<Tile>)>> {
    separated_list0(
        line_ending,
        pair(
            many0_count(tag(" ")),
            many1(alt((
                value(Tile::Open, tag(".")),
                value(Tile::Wall, tag("#")),
            ))),
        ),
    )(input)
}

fn parse_path_instructions(input: &str) -> IResult<&str, Vec<PathInstruction>> {
    many1(alt((
        value(PathInstruction::TurnLeft, tag("L")),
        value(PathInstruction::TurnRight, tag("R")),
        map(map_res(digit1, str::parse), PathInstruction::Move),
    )))(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (board_rows, path_instructions) = parse_input(input).unwrap().1;

    let row_bounds = board_rows
        .iter()
        .map(|(min_x, tiles)| (*min_x, min_x + tiles.len() - 1))
        .collect::<Vec<_>>();

    let grid_width = row_bounds.iter().map(|(_, max_x)| *max_x).max().unwrap() + 1;
    let grid_height = board_rows.len();

    let mut col_bounds = vec![(usize::MAX, usize::MIN); grid_width];
    let mut grid: Vec<Vec<Option<Tile>>> = vec![vec![None; grid_width]; grid_height];

    for (y, (min_x, tiles)) in board_rows.into_iter().enumerate() {
        for i in 0..tiles.len() {
            let x = min_x + i;
            grid[y][x] = Some(tiles[i]);

            let (min_y, max_y) = &mut col_bounds[x];
            *min_y = (*min_y).min(y);
            *max_y = (*max_y).max(y);
        }
    }

    let mut y = 0;
    let mut x = row_bounds[y].0;
    let mut facing = Direction::Right;

    for instruction in path_instructions {
        match instruction {
            PathInstruction::Move(steps) => {
                for _ in 0..steps {
                    let (min_x, max_x) = row_bounds[y];
                    let (min_y, max_y) = col_bounds[x];

                    let (nx, ny) = match facing {
                        Direction::Right if x < max_x => (x + 1, y),
                        Direction::Right => (min_x, y),
                        Direction::Down if y < max_y => (x, y + 1),
                        Direction::Down => (x, min_y),
                        Direction::Left if x > min_x => (x - 1, y),
                        Direction::Left => (max_x, y),
                        Direction::Up if y > min_y => (x, y - 1),
                        Direction::Up => (x, max_y),
                    };

                    if matches!(grid[ny][nx], Some(Tile::Wall)) {
                        break;
                    }

                    y = ny;
                    x = nx;
                }
            }
            PathInstruction::TurnLeft => {
                facing = match facing {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                };
            }
            PathInstruction::TurnRight => {
                facing = match facing {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                };
            }
        }
    }

    Some(1000 * (y + 1) + 4 * (x + 1) + facing as usize)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
