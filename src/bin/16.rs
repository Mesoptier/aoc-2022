use std::collections::BinaryHeap;

use hashbrown::HashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, Vec<(String, u32, Vec<String>)>> {
    separated_list0(
        line_ending,
        tuple((
            preceded(tag("Valve "), map(alpha1, String::from)),
            preceded(tag(" has flow rate="), map_res(digit1, str::parse)),
            preceded(
                alt((
                    tag("; tunnel leads to valve "),
                    tag("; tunnels lead to valves "),
                )),
                separated_list0(tag(", "), map(alpha1, String::from)),
            ),
        )),
    )(input)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct GState<T, V>
where
    T: Ord,
    V: Ord,
{
    time: T,
    valve_idx: V,
    opened_valves: u64,
}

impl<T, V> GState<T, V>
where
    T: Ord,
    V: Ord,
{
    fn open_valve(&mut self, idx: u32) {
        self.opened_valves |= 1 << idx;
    }

    fn is_valve_opened(&self, idx: u32) -> bool {
        (self.opened_valves & (1 << idx)) != 0
    }
}

#[derive(PartialEq, Eq)]
struct GHeapEntry<T, V>(u32, GState<T, V>)
where
    T: Ord,
    V: Ord;

impl<T, V> Ord for GHeapEntry<T, V>
where
    T: Ord,
    V: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).then_with(|| other.1.cmp(&self.1))
    }
}

impl<T, V> PartialOrd for GHeapEntry<T, V>
where
    T: Ord,
    V: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    type State = GState<u32, u32>;
    type HeapEntry = GHeapEntry<u32, u32>;

    let valves = parse_input(input).unwrap().1;
    let label_to_idx: HashMap<String, u32> = HashMap::from_iter(
        valves
            .iter()
            .enumerate()
            .map(|(idx, (label, _, _))| (label.clone(), idx as u32)),
    );
    let valves: Vec<(u32, u32, Vec<u32>)> = valves
        .into_iter()
        .map(|(label, flow_rate, adj_labels)| {
            (
                label_to_idx[&label],
                flow_rate,
                adj_labels
                    .into_iter()
                    .map(|adj_label| label_to_idx[&adj_label])
                    .collect(),
            )
        })
        .collect();

    // Find shortest path between all valve pairs
    let mut dist = vec![vec![u32::MAX; valves.len()]; valves.len()];
    for (valve_idx, _, adjacencies) in &valves {
        for adj_valve_idx in adjacencies {
            dist[*valve_idx as usize][*adj_valve_idx as usize] = 1;
        }
        dist[*valve_idx as usize][*valve_idx as usize] = 0;
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                dist[i][j] = dist[i][j].min(dist[i][k].saturating_add(dist[k][j]));
            }
        }
    }

    let start_state = State {
        time: 0,
        valve_idx: label_to_idx["AA"],
        opened_valves: 0,
    };

    let max_time = 30;

    let mut max_score = 0;
    let mut best_scores = HashMap::<State, u32>::new();
    let mut heap = BinaryHeap::<HeapEntry>::new();

    best_scores.insert(start_state, 0);
    heap.push(GHeapEntry(0, start_state));

    while let Some(GHeapEntry(score, state)) = heap.pop() {
        if state.time == max_time {
            continue;
        }
        if score != best_scores[&state] {
            continue;
        }

        max_score = max_score.max(score);

        for &(adj_valve_idx, adj_flow_rate, _) in &valves {
            if adj_flow_rate == 0 {
                continue;
            }

            if state.is_valve_opened(adj_valve_idx) {
                continue;
            }

            let mut adj_state = state;

            // Move to valve
            adj_state.time += dist[state.valve_idx as usize][adj_valve_idx as usize];
            adj_state.valve_idx = adj_valve_idx;

            // Open the valve
            adj_state.time += 1;
            adj_state.open_valve(adj_valve_idx);

            if adj_state.time >= max_time {
                continue;
            }

            let adj_score = score + adj_flow_rate * (max_time - adj_state.time);

            if best_scores
                .get(&adj_state)
                .map_or(true, |&best_score| adj_score > best_score)
            {
                best_scores.insert(adj_state, adj_score);
                heap.push(GHeapEntry(adj_score, adj_state));
            }
        }
    }

    Some(max_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    type State = GState<[u32; 2], [u32; 2]>;
    type HeapEntry = GHeapEntry<[u32; 2], [u32; 2]>;

    let valves = parse_input(input).unwrap().1;
    let label_to_idx: HashMap<String, u32> = HashMap::from_iter(
        valves
            .iter()
            .enumerate()
            .map(|(idx, (label, _, _))| (label.clone(), idx as u32)),
    );
    let valves: Vec<(u32, u32, Vec<u32>)> = valves
        .into_iter()
        .map(|(label, flow_rate, adj_labels)| {
            (
                label_to_idx[&label],
                flow_rate,
                adj_labels
                    .into_iter()
                    .map(|adj_label| label_to_idx[&adj_label])
                    .collect(),
            )
        })
        .collect();

    // Find shortest path between all valve pairs
    let mut dist = vec![vec![u32::MAX; valves.len()]; valves.len()];
    for (valve_idx, _, adjacencies) in &valves {
        for adj_valve_idx in adjacencies {
            dist[*valve_idx as usize][*adj_valve_idx as usize] = 1;
        }
        dist[*valve_idx as usize][*valve_idx as usize] = 0;
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                dist[i][j] = dist[i][j].min(dist[i][k].saturating_add(dist[k][j]));
            }
        }
    }

    let start_state = State {
        time: [0, 0],
        valve_idx: [label_to_idx["AA"], label_to_idx["AA"]],
        opened_valves: 0,
    };

    let max_time = 26;

    let mut max_score = 0;
    let mut best_scores = HashMap::<State, u32>::new();
    let mut heap = BinaryHeap::<HeapEntry>::new();

    best_scores.insert(start_state, 0);
    heap.push(GHeapEntry(0, start_state));

    while let Some(GHeapEntry(score, state)) = heap.pop() {
        //        if state.time == max_time {
        //            continue;
        //        }
        if score != best_scores[&state] {
            continue;
        }

        max_score = max_score.max(score);

        for &(adj_valve_idx, adj_flow_rate, _) in &valves {
            if adj_flow_rate == 0 {
                continue;
            }

            if state.is_valve_opened(adj_valve_idx) {
                continue;
            }

            for foo in [0, 1] {
                let mut adj_state = state;

                // Move to valve
                adj_state.time[foo] += dist[state.valve_idx[foo] as usize][adj_valve_idx as usize];
                adj_state.valve_idx[foo] = adj_valve_idx;

                // Open the valve
                adj_state.time[foo] += 1;
                adj_state.open_valve(adj_valve_idx);

                if adj_state.time[foo] >= max_time {
                    continue;
                }

                let adj_score = score + adj_flow_rate * (max_time - adj_state.time[foo]);

                if best_scores
                    .get(&adj_state)
                    .map_or(true, |&best_score| adj_score > best_score)
                {
                    best_scores.insert(adj_state, adj_score);
                    heap.push(GHeapEntry(adj_score, adj_state));
                }
            }
        }
    }

    Some(max_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
