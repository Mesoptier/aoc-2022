use std::collections::VecDeque;

use hashbrown::HashSet;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map_res, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

type Costs = (u32, Option<u32>, Option<u32>);
type Blueprint = (Costs, Costs, Costs, Costs);

fn parse_input(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list0(line_ending, parse_blueprint)(input)
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    preceded(
        delimited(tag("Blueprint "), digit1, tag(":")),
        tuple((
            delimited(tag(" Each ore robot costs "), parse_costs, tag(".")),
            delimited(tag(" Each clay robot costs "), parse_costs, tag(".")),
            delimited(tag(" Each obsidian robot costs "), parse_costs, tag(".")),
            delimited(tag(" Each geode robot costs "), parse_costs, tag(".")),
        )),
    )(input)
}

fn parse_costs(input: &str) -> IResult<&str, Costs> {
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
    ))(input)
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct State {
    time: u32,
    robots: (u32, u32, u32, u32),
    resources: (u32, u32, u32, u32),
}

impl State {
    fn next_time_step(&self) -> Self {
        Self {
            time: self.time + 1,
            robots: self.robots,
            resources: (
                self.resources.0 + self.robots.0,
                self.resources.1 + self.robots.1,
                self.resources.2 + self.robots.2,
                self.resources.3 + self.robots.3,
            ),
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
            resources: (
                state.resources.0 - costs.0,
                state.resources.1 - costs.1.unwrap_or(0),
                state.resources.2 - costs.2.unwrap_or(0),
                state.resources.3,
            ),
            ..state
        })
    }

    fn will_be_affordable(&self, costs: Costs) -> bool {
        assert!(self.robots.0 > 0);
        if costs.1.unwrap_or(0) > 0 && self.robots.1 == 0 {
            assert_eq!(self.resources.1, 0);
            return false;
        }
        if costs.2.unwrap_or(0) > 0 && self.robots.2 == 0 {
            assert_eq!(self.resources.2, 0);
            return false;
        }
        true
    }

    fn is_affordable(&self, costs: Costs) -> bool {
        self.resources.0 >= costs.0
            && self.resources.1 >= costs.1.unwrap_or(0)
            && self.resources.2 >= costs.2.unwrap_or(0)
    }
}

fn eval_blueprint(blueprint: Blueprint, max_time: u32) -> u32 {
    let start_state = State {
        time: 0,
        robots: (1, 0, 0, 0),
        resources: (0, 0, 0, 0),
    };

    let mut visited = HashSet::<State>::new();

    let mut queue = VecDeque::<State>::new();
    queue.push_back(start_state);

    let mut max_geodes = 0;

    while let Some(state) = queue.pop_front() {
        if state.time > max_time {
            continue;
        }

        max_geodes = max_geodes.max(state.resources.3);

        if state.time == max_time {
            continue;
        }

        let mut adj_states = vec![];

        if let Some(adj_state) = state.spend(blueprint.0) {
            let mut adj_state = adj_state.next_time_step();
            adj_state.robots.0 += 1;
            adj_states.push(adj_state);
        }

        if let Some(adj_state) = state.spend(blueprint.1) {
            let mut adj_state = adj_state.next_time_step();
            adj_state.robots.1 += 1;
            adj_states.push(adj_state);
        }

        if let Some(adj_state) = state.spend(blueprint.2) {
            let mut adj_state = adj_state.next_time_step();
            adj_state.robots.2 += 1;
            adj_states.push(adj_state);
        }

        if let Some(adj_state) = state.spend(blueprint.3) {
            let mut adj_state = adj_state.next_time_step();
            adj_state.robots.3 += 1;
            adj_states.push(adj_state);
        }

        if adj_states.is_empty() {
            let mut adj_state = state.next_time_step();
            while adj_state.time < max_time {
                adj_state = adj_state.next_time_step();
            }
            adj_states.push(adj_state);
        }

        for adj_state in adj_states {
            if visited.contains(&adj_state) {
                continue;
            }

            visited.insert(adj_state);
            queue.push_back(adj_state);
        }
    }

    max_geodes
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
    None
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
        assert_eq!(part_two(&input), None);
    }
}
