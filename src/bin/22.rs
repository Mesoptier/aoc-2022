use hashbrown::HashMap;
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
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

#[derive(Clone, Copy)]
struct State {
    region_x: usize,
    region_y: usize,
    local_x: usize,
    local_y: usize,
    facing: Direction,
}

impl State {
    fn password(self, region_size: usize) -> usize {
        let x = self.region_x * region_size + self.local_x;
        let y = self.region_y * region_size + self.local_y;
        1000 * (y + 1) + 4 * (x + 1) + self.facing as usize
    }

    fn turn_left(self) -> Self {
        Self {
            facing: self.facing.turn_left(),
            ..self
        }
    }

    fn turn_right(self) -> Self {
        Self {
            facing: self.facing.turn_right(),
            ..self
        }
    }

    fn turn_region_towards(mut self, facing: Direction, region_size: usize) -> Self {
        while self.facing != facing {
            self.facing = self.facing.turn_left();
            (self.local_x, self.local_y) = (self.local_y, region_size - self.local_x - 1);
        }
        self
    }

    fn move_forward(
        mut self,
        region_size: usize,
        adjacencies: &HashMap<(usize, usize, Direction), (usize, usize, Direction)>,
    ) -> Self {
        match self.facing {
            Direction::Right if self.local_x + 1 < region_size => {
                self.local_x += 1;
                return self;
            }
            Direction::Right => self.local_x = 0,
            Direction::Down if self.local_y + 1 < region_size => {
                self.local_y += 1;
                return self;
            }
            Direction::Down => self.local_y = 0,
            Direction::Left if self.local_x > 0 => {
                self.local_x -= 1;
                return self;
            }
            Direction::Left => self.local_x = region_size - 1,
            Direction::Up if self.local_y > 0 => {
                self.local_y -= 1;
                return self;
            }
            Direction::Up => self.local_y = region_size - 1,
        }

        let (next_region_y, next_region_x, next_facing) =
            adjacencies[&(self.region_y, self.region_x, self.facing)];
        self.region_x = next_region_x;
        self.region_y = next_region_y;
        self.turn_region_towards(next_facing, region_size)
    }
}

fn solve(
    input: &str,
    region_size: usize,
    region_adjacencies: HashMap<(usize, usize, Direction), (usize, usize, Direction)>,
) -> Option<usize> {
    let (board_rows, path_instructions) = parse_input(input).unwrap().1;

    let start_region_y = 0;
    let start_region_x = board_rows[start_region_y].0 / region_size;

    let mut regions: HashMap<(usize, usize), Vec<Vec<Tile>>> = HashMap::new();
    board_rows
        .into_iter()
        .enumerate()
        .for_each(|(y, (x, mut tiles))| {
            let region_y = y / region_size;
            let mut region_x = x / region_size;

            while !tiles.is_empty() {
                let remaining_tiles = tiles.split_off(region_size);

                let region = regions.entry((region_y, region_x)).or_insert(vec![]);
                region.push(tiles);

                tiles = remaining_tiles;
                region_x += 1;
            }
        });

    let mut state = State {
        region_x: start_region_x,
        region_y: start_region_y,
        local_x: 0,
        local_y: 0,
        facing: Direction::Right,
    };

    for instruction in path_instructions {
        match instruction {
            PathInstruction::Move(steps) => {
                for _ in 0..steps {
                    let next_state = state.move_forward(region_size, &region_adjacencies);
                    let region = &regions[&(next_state.region_y, next_state.region_x)];
                    if matches!(region[next_state.local_y][next_state.local_x], Tile::Wall) {
                        break;
                    }
                    state = next_state;
                }
            }
            PathInstruction::TurnLeft => {
                state = state.turn_left();
            }
            PathInstruction::TurnRight => {
                state = state.turn_right();
            }
        }
    }

    Some(state.password(region_size))
}

fn solve_part_one(input: &str, region_size: usize) -> Option<usize> {
    use Direction::*;

    // TODO: find a non-hardcoded way of computing these adjacencies
    let region_adjacencies: HashMap<(usize, usize, Direction), (usize, usize, Direction)> =
        if region_size == 4 {
            HashMap::from([
                ((0, 2, Right), (0, 2, Right)),
                ((0, 2, Down), (1, 2, Down)),
                ((0, 2, Left), (0, 2, Left)),
                ((0, 2, Up), (2, 2, Up)),
                //
                ((1, 0, Right), (1, 1, Right)),
                ((1, 0, Down), (1, 0, Down)),
                ((1, 0, Left), (1, 2, Left)),
                ((1, 0, Up), (1, 0, Up)),
                //
                ((1, 1, Right), (1, 2, Right)),
                ((1, 1, Down), (1, 1, Down)),
                ((1, 1, Left), (1, 0, Left)),
                ((1, 1, Up), (1, 1, Up)),
                //
                ((1, 2, Right), (1, 0, Right)),
                ((1, 2, Down), (2, 2, Down)),
                ((1, 2, Left), (1, 1, Left)),
                ((1, 2, Up), (0, 2, Up)),
                //
                ((2, 2, Right), (2, 3, Right)),
                ((2, 2, Down), (0, 2, Down)),
                ((2, 2, Left), (2, 3, Left)),
                ((2, 2, Up), (1, 2, Up)),
                //
                ((2, 3, Right), (2, 2, Right)),
                ((2, 3, Down), (2, 3, Down)),
                ((2, 3, Left), (2, 2, Left)),
                ((2, 3, Up), (2, 3, Up)),
            ])
        } else {
            HashMap::from([
                ((0, 1, Right), (0, 2, Right)),
                ((0, 1, Down), (1, 1, Down)),
                ((0, 1, Left), (0, 2, Left)),
                ((0, 1, Up), (2, 1, Up)),
                //
                ((0, 2, Right), (0, 1, Right)),
                ((0, 2, Down), (0, 2, Down)),
                ((0, 2, Left), (0, 1, Left)),
                ((0, 2, Up), (0, 2, Up)),
                //
                ((1, 1, Right), (1, 1, Right)),
                ((1, 1, Down), (2, 1, Down)),
                ((1, 1, Left), (1, 1, Left)),
                ((1, 1, Up), (0, 1, Up)),
                //
                ((2, 0, Right), (2, 1, Right)),
                ((2, 0, Down), (3, 0, Down)),
                ((2, 0, Left), (2, 1, Left)),
                ((2, 0, Up), (3, 0, Up)),
                //
                ((2, 1, Right), (2, 0, Right)),
                ((2, 1, Down), (0, 1, Down)),
                ((2, 1, Left), (2, 0, Left)),
                ((2, 1, Up), (1, 1, Up)),
                //
                ((3, 0, Right), (3, 0, Right)),
                ((3, 0, Down), (2, 0, Down)),
                ((3, 0, Left), (3, 0, Left)),
                ((3, 0, Up), (2, 0, Up)),
            ])
        };

    solve(input, region_size, region_adjacencies)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_part_one(input, 50)
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
        assert_eq!(solve_part_one(&input, 4), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
