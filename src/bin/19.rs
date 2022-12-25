use hashbrown::HashSet;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

type Costs = [u32; 3];
type Blueprint = [Costs; 4];

fn parse_input(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list0(line_ending, parse_blueprint)(input)
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    preceded(
        delimited(tag("Blueprint "), digit1, tag(":")),
        map(
            tuple((
                delimited(tag(" Each ore robot costs "), parse_costs, tag(".")),
                delimited(tag(" Each clay robot costs "), parse_costs, tag(".")),
                delimited(tag(" Each obsidian robot costs "), parse_costs, tag(".")),
                delimited(tag(" Each geode robot costs "), parse_costs, tag(".")),
            )),
            |(a, b, c, d)| [a, b, c, d],
        ),
    )(input)
}

fn parse_costs(input: &str) -> IResult<&str, Costs> {
    map(
        tuple((
            terminated(map_res(digit1, str::parse), tag(" ore")),
            opt(delimited(
                tag(" and "),
                map_res(digit1, str::parse),
                tag(" clay"),
            )),
            opt(delimited(
                tag(" and "),
                map_res(digit1, str::parse),
                tag(" obsidian"),
            )),
        )),
        |(a, b, c)| [a, b.unwrap_or(0), c.unwrap_or(0)],
    )(input)
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct State {
    time: u32,
    robots: [u32; 4],
    resources: [u32; 4],
}

impl State {
    fn next_time_step(&self) -> Self {
        Self {
            time: self.time + 1,
            robots: self.robots,
            resources: [
                self.resources[0] + self.robots[0],
                self.resources[1] + self.robots[1],
                self.resources[2] + self.robots[2],
                self.resources[3] + self.robots[3],
            ],
        }
    }

    fn spend(&self, costs: Costs) -> Option<Self> {
        if !self.will_be_affordable(costs) {
            return None;
        }

        let mut state = *self;
        while !state.is_affordable(costs) {
            state = state.next_time_step();
        }

        Some(Self {
            resources: [
                state.resources[0] - costs[0],
                state.resources[1] - costs[1],
                state.resources[2] - costs[2],
                state.resources[3],
            ],
            ..state
        })
    }

    fn will_be_affordable(&self, costs: Costs) -> bool {
        assert!(self.robots[0] > 0);
        if costs[1] > 0 && self.robots[1] == 0 {
            assert_eq!(self.resources[1], 0);
            return false;
        }
        if costs[2] > 0 && self.robots[2] == 0 {
            assert_eq!(self.resources[2], 0);
            return false;
        }
        true
    }

    fn is_affordable(&self, costs: Costs) -> bool {
        self.resources[0] >= costs[0]
            && self.resources[1] >= costs[1]
            && self.resources[2] >= costs[2]
    }
}

fn eval_blueprint(blueprint: Blueprint, max_time: u32) -> u32 {
    let start_state = State {
        time: 0,
        robots: [1, 0, 0, 0],
        resources: [0, 0, 0, 0],
    };

    let branch = |state: State| -> Vec<State> {
        let mut new_states = vec![];

        new_states.extend((0..4).rev().filter_map(|idx| {
            state
                .spend(blueprint[idx])
                .map(|new_state| {
                    let mut new_state = new_state.next_time_step();
                    new_state.robots[idx] += 1;
                    new_state
                })
                .filter(|new_state| new_state.time <= max_time)
        }));

        if new_states.is_empty() {
            let mut new_state = state.next_time_step();
            while new_state.time < max_time {
                new_state = new_state.next_time_step();
            }
            new_states.push(new_state);
        }

        new_states
    };

    let bound = |state: State| -> u32 {
        let remaining_time = max_time - state.time;
        let mut upper_bound = 0;

        // Current geodes
        upper_bound += state.resources[3];

        // Geodes produced by current robots
        upper_bound += remaining_time * state.robots[3];

        // Geodes produced if we build a robot every timestep (triangular number)
        if remaining_time > 1 {
            upper_bound += (remaining_time - 1) * remaining_time / 2;
        }

        upper_bound
    };

    let solution = |state: State| -> Option<u32> {
        if state.time == max_time {
            Some(state.resources[3])
        } else {
            None
        }
    };

    let mut visited = HashSet::<State>::new();
    let mut stack = Vec::<State>::new();

    visited.insert(start_state);
    stack.push(start_state);

    let mut best_num_geodes = 0;

    while let Some(state) = stack.pop() {
        if let Some(num_geodes) = solution(state) {
            if num_geodes > best_num_geodes {
                best_num_geodes = num_geodes
            }
        } else {
            for next_state in branch(state) {
                if bound(next_state) <= best_num_geodes {
                    continue;
                }
                if visited.contains(&next_state) {
                    continue;
                }

                stack.push(next_state);
                visited.insert(next_state);
            }
        }
    }

    best_num_geodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = parse_input(input).unwrap().1;

    Some(
        blueprints
            .into_iter()
            .enumerate()
            .map(|(id, blueprint)| eval_blueprint(blueprint, 24) * (id as u32 + 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let blueprints = parse_input(input).unwrap().1;

    Some(
        blueprints
            .into_iter()
            .take(3)
            .map(|blueprint| eval_blueprint(blueprint, 32))
            .product(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(3472));
    }
}
