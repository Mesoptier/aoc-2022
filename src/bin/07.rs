use std::{
    cell::RefCell,
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res, rest},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum Node {
    File(usize),
    Directory(Rc<RefCell<HashMap<String, Node>>>),
}

fn parse_node(input: &str) -> IResult<&str, (Node, String)> {
    separated_pair(
        alt((
            map(tag("dir"), |_| Node::Directory(Rc::default())),
            map(map_res(digit1, str::parse::<usize>), Node::File),
        )),
        tag(" "),
        map(rest, String::from),
    )(input)
}

fn process_input(input: &str) -> HashMap<String, Node> {
    let root_nodes = Rc::new(RefCell::new(HashMap::<String, Node>::new()));
    let mut stack = vec![root_nodes.clone()];

    for line in input.lines() {
        const CD_PREFIX: &'static str = "$ cd ";
        const LS_PREFIX: &'static str = "$ ls";

        if line.starts_with(CD_PREFIX) {
            // `cd` command
            let arg = &line[CD_PREFIX.len()..];
            match arg {
                "/" => {
                    stack.truncate(1);
                }
                ".." => {
                    stack.pop();
                }
                dir_name => {
                    let cur_dir_ptr = stack.last().unwrap().clone();
                    let cur_dir = cur_dir_ptr.borrow();
                    let dir_node = cur_dir.get(dir_name).unwrap();

                    if let Node::Directory(dir) = dir_node {
                        stack.push(dir.clone());
                    }
                }
            }
        } else if line.starts_with(LS_PREFIX) {
            // `ls` command
            continue;
        } else {
            // `ls` output
            let (_, (node, name)) = parse_node(line).unwrap();
            let cur_dir = stack.last().unwrap();
            cur_dir.borrow_mut().insert(name, node);
        }
    }

    stack.clear();
    Rc::try_unwrap(root_nodes).unwrap().into_inner()
}

pub fn part_one(input: &str) -> Option<usize> {
    fn process_nodes(nodes: &HashMap<String, Node>) -> (usize, usize) {
        let (result, dir_size) =
            nodes
                .values()
                .fold((0, 0), |(result, dir_size), node| match node {
                    Node::File(file_size) => (result, dir_size + file_size),
                    Node::Directory(child_nodes) => {
                        let (child_result, child_dir_size) = process_nodes(&child_nodes.borrow());
                        (result + child_result, dir_size + child_dir_size)
                    }
                });

        if dir_size <= 100000 {
            (result + dir_size, dir_size)
        } else {
            (result, dir_size)
        }
    }

    let root_nodes = process_input(input);
    let (result, _) = process_nodes(&root_nodes);
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    fn compute_directory_size(
        nodes: &HashMap<String, Node>,
        sizes: &mut BinaryHeap<usize>,
    ) -> usize {
        let dir_size = nodes
            .values()
            .map(|node| match node {
                Node::File(file_size) => *file_size,
                Node::Directory(child_nodes) => {
                    compute_directory_size(&child_nodes.borrow(), sizes)
                }
            })
            .sum();

        sizes.push(dir_size);
        dir_size
    }

    let root_nodes = process_input(input);

    let mut sizes = BinaryHeap::new();
    let dir_used_size = compute_directory_size(&root_nodes, &mut sizes);
    let min_delete_size = dir_used_size - (70_000_000 - 30_000_000);

    let sizes = sizes.into_sorted_vec();
    let idx = sizes.partition_point(|dir_size| *dir_size <= min_delete_size);
    Some(sizes[idx])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
