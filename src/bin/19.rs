use hashbrown::HashSet;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use num::Integer;

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
    fn step_time(&mut self, time_steps: u32) {
        self.time += time_steps;
        for idx in 0..4 {
            self.resources[idx] += self.robots[idx] * time_steps;
        }
    }

    fn build_robot(&self, robot_idx: usize, costs: Costs) -> Option<Self> {
        let mut t = 0;

        for idx in (0..3).rev() {
            if self.resources[idx] >= costs[idx] {
                // Already have enough of this resource
                continue;
            }
            if self.robots[idx] == 0 {
                assert_eq!(self.resources[idx], 0);

                // No robot that produces this resource
                return None;
            }
            let diff_resources = costs[idx] - self.resources[idx];
            t = t.max(Integer::div_ceil(&diff_resources, &self.robots[idx]));
        }

        let mut state = *self;

        // Gather required resources
        state.step_time(t);

        // Build the robot
        for idx in 0..3 {
            state.resources[idx] -= costs[idx];
        }
        assert!(
            t == 0
                || state.resources[0] < costs[0]
                || state.resources[1] < costs[1]
                || state.resources[2] < costs[2]
        );
        state.step_time(1);
        state.robots[robot_idx] += 1;

        Some(state)
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

        // Build robots, prioritizing those that produce higher-tier resources (i.e. geodes)
        new_states.extend((0..4).rev().filter_map(|robot_idx| {
            state
                .build_robot(robot_idx, blueprint[robot_idx])
                .filter(|new_state| new_state.time <= max_time)
        }));

        // If no more robots can be built from this state, just run through to the end 
        if new_states.is_empty() {
            let mut new_state = state;
            new_state.step_time(max_time - state.time);
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
