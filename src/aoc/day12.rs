use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
enum Node {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => f.write_str("start"),
            Self::End => f.write_str("end"),
            Self::Big(name) => f.write_str(name),
            Self::Small(name) => f.write_str(name),
        }
    }
}

impl Node {
    fn from_str(s: &str) -> Self {
        match s {
            "start" => Node::Start,
            "end" => Node::End,
            s if s.chars().next().unwrap().is_uppercase() => Node::Big(s.to_owned()),
            s => Node::Small(s.to_owned()),
        }
    }
}

struct Graph {
    graph: HashMap<Node, HashSet<Node>>,
}

impl Graph {
    fn from_str(s: &str) -> Self {
        let graph: HashMap<Node, HashSet<Node>> = s.trim().lines()
            .map(|line| line.trim())
            .map(|line| line.split_once("-").unwrap())
            .map(|(a, b)| (Node::from_str(a), Node::from_str(b)))
            .map(|(a, b)| match (a, b) {
                (a @ Node::Start, b) => vec![(a, b)],
                (a, b @ Node::End) => vec![(a, b)],
                (a, b @ Node::Start) => vec![(b, a)],
                (a @ Node::End, b) => vec![(b, a)],
                (a, b) => vec![(b.clone(), a.clone()), (a, b)],
            }).flatten()
            .into_grouping_map()
            .collect();
        Graph { graph }
    }

    fn get_paths(&self) -> Vec<Vec<Node>> {
        let mut to_visit_stack = VecDeque::new();
        let mut current_stack = VecDeque::new();
        let mut visited_small = HashSet::new();

        let mut paths = Vec::new();

        let starting_nodes = self.graph[&Node::Start].iter().cloned().collect::<VecDeque<_>>();
        to_visit_stack.push_back(starting_nodes);
        while !to_visit_stack.is_empty() {
            let current_node = to_visit_stack.back_mut().unwrap().pop_front();
            if current_node.is_none() {
                to_visit_stack.pop_back().unwrap();
                let last = current_stack.pop_back().unwrap_or(Node::Start);
                visited_small.remove(&last);
            } else {
                let current_node = current_node.unwrap();
                if let &Node::Small(_) = &current_node {
                    visited_small.insert(current_node.clone());
                }

                current_stack.push_back(current_node.clone());
                to_visit_stack.push_back(self.get_neighbors(&current_node, &visited_small));

                if let &Node::End = &current_node {
                    paths.push(current_stack.iter().cloned().collect::<Vec<_>>())
                }
            }
        }
        paths
    }

    fn get_paths2(&self) -> Vec<Vec<Node>> {
        let mut to_visit_stack = VecDeque::new();
        let mut current_stack = VecDeque::new();
        let mut visited_small = HashSet::new();
        let mut visited_twice: Option<Node> = None;

        let mut paths = Vec::new();

        let starting_nodes = self.graph[&Node::Start].iter().cloned().collect::<VecDeque<_>>();
        to_visit_stack.push_back(starting_nodes);
        while !to_visit_stack.is_empty() {
            let current_node = to_visit_stack.back_mut().unwrap().pop_front();
            if current_node.is_none() {
                to_visit_stack.pop_back().unwrap();
                let last = current_stack.pop_back().unwrap_or(Node::Start);
                if visited_twice.as_ref() == Some(&last) {
                    visited_twice = None;
                } else {
                    visited_small.remove(&last);
                }
            } else {
                let current_node = current_node.unwrap();
                if visited_small.contains(&current_node) {
                    if visited_twice.is_none() {
                        visited_twice = Some(current_node.clone());
                    } else {
                        continue;
                    }
                }

                if let &Node::Small(_) = &current_node {
                    visited_small.insert(current_node.clone());
                }

                current_stack.push_back(current_node.clone());
                to_visit_stack.push_back(self.get_neighbors2(&current_node));

                if let &Node::End = &current_node {
                    paths.push(current_stack.iter().cloned().collect::<Vec<_>>())
                }
            }
        }
        paths
    }

    fn get_neighbors(&self, node: &Node, visited_small: &HashSet<Node>) -> VecDeque<Node> {
        self.graph.get(node).unwrap_or(&HashSet::new()).iter()
            .filter(|node| !visited_small.contains(*node))
            .cloned()
            .collect()
    }

    fn get_neighbors2(&self, node: &Node) -> VecDeque<Node> {
        self.graph.get(node).unwrap_or(&HashSet::new()).iter()
            .cloned()
            .collect()
    }
}

fn process1(input: &str) -> usize {
    let graph = Graph::from_str(input);
    graph.get_paths().iter()
        .inspect(|path| {
            for node in path.iter() {
                print!("{},", node);
            }
            println!();
        })
        .count()
}

fn process2(input: &str) -> usize {
    let graph = Graph::from_str(input);
    graph.get_paths2().iter()
        .sorted()
        // .inspect(|path| {
        //     for node in path.iter() {
        //         print!("{},", node);
        //     }
        //     println!();
        // })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::aoc::day12::{process1, process2};
    use crate::common::read_to_string;

    const TEST_GRAPH1: &'static str = r#"
start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

    const TEST_GRAPH2: &'static str = r#"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;

    const TEST_GRAPH3: &'static str = r#"
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;

    const INPUT_FILE: &'static str = "input/input12";

    #[test]
    fn test1() {
        let result = process1(TEST_GRAPH1);
        assert_eq!(result, 10);
    }

    #[test]
    fn test2() {
        let result = process1(TEST_GRAPH2);
        assert_eq!(result, 19);
    }

    #[test]
    fn test3() {
        let result = process1(TEST_GRAPH3);
        assert_eq!(result, 226);
    }

    #[test]
    fn run() {
        let input = read_to_string(INPUT_FILE);
        let result = process1(&input);
        println!("{}", result);
    }

    #[test]
    fn test2_1() {
        let result = process2(TEST_GRAPH1);
        assert_eq!(result, 36);
    }

    #[test]
    fn test2_2() {
        let result = process2(TEST_GRAPH2);
        assert_eq!(result, 103);
    }

    #[test]
    fn test2_3() {
        let result = process2(TEST_GRAPH3);
        assert_eq!(result, 3509);
    }

    #[test]
    fn run2() {
        let input = read_to_string(INPUT_FILE);
        let result = process2(&input);
        println!("{}", result);
    }
}