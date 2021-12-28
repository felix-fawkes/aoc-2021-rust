use std::collections::BTreeMap;
use itertools::Itertools;

#[derive(PartialEq, Eq, Clone)]
enum Octopus {
    Ready(u8),
    Flashed,
}

impl Octopus {
    fn is_flashing(&self) -> bool {
        match *self {
            Self::Ready(v) if v > 9 => { true }
            _ => false,
        }
    }

    fn inc(&mut self) {
        *self = match self.clone() {
            Self::Ready(v) => (Self::Ready(v + 1)),
            oct => oct.clone(),
        }
    }

    fn flash(&mut self) {
        *self = match self.clone() {
            Self::Ready(v) if v > 9 => Self::Flashed,
            oct => oct,
        }
    }

    fn clean_flashed(&mut self) {
        *self = match self.clone() {
            Self::Flashed => Self::Ready(0),
            oct => oct,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn neighbors(&self) -> Vec<Point> {
        vec![
            Point::new(self.x + 1, self.y),
            Point::new(self.x + 1, self.y + 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
        ]
    }
}

struct State {
    octopuses: BTreeMap<Point, Octopus>,
}

impl State {
    pub fn from_str(s: &str) -> State {
        let mut octopuses = BTreeMap::new();
        s.split_whitespace()
            .map(|s| s.trim())
            .enumerate()
            .map(|(y, s)| {
                s.chars().enumerate()
                    .map(|(x, ch)| (x, y, ch.to_string().parse::<u8>().unwrap()))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .map(|(x, y, v)| (Point::new(x as i32, y as i32), v))
            .for_each(|(p, v)| {
                octopuses.insert(p, Octopus::Ready(v));
            });

        State { octopuses }
    }

    pub fn step(&mut self) -> usize {
        self.inc_all();
        while let true = self.flash_all() {}
        self.clean_flashed()
    }

    fn inc_all(&mut self) {
        self.octopuses.values_mut()
            .for_each(|octopus| octopus.inc())
    }

    fn flash_all(&mut self) -> bool {
        let flashing_points = self.octopuses.iter_mut()
            .filter_map(|(point, octopus)| {
                if octopus.is_flashing() { Some(point) } else { None }
            }).cloned()
            .collect_vec();

        for point in &flashing_points {
            self.octopuses.get_mut(point).unwrap().flash();
        }

        let neighbors: Vec<_> = flashing_points.iter()
            .flat_map(|point| point.neighbors().into_iter())
            .filter(|point| self.octopuses.contains_key(point))
            .collect();
        neighbors.into_iter()
            .for_each(|point| self.octopuses.get_mut(&point).unwrap().inc());

        !flashing_points.is_empty()
    }

    fn clean_flashed(&mut self) -> usize {
        let mut count = 0;
        self.octopuses.values_mut()
            .for_each(|oct| {
                oct.clean_flashed();
                if oct == &Octopus::Ready(0) { count += 1 };
            });
        count
    }
}

fn process1(map: &str, iterations: usize) -> usize {
    let mut state = State::from_str(map);

    (0..iterations)
        .map(|_| {
            state.step()
        })
        .sum()
}

fn process2(map: &str) -> usize {
    let mut state = State::from_str(map);
    let map_size = state.octopuses.len();

    let mut counter = 0;
    while state.step() != map_size {
        counter += 1;
    }

    counter+1
}


#[cfg(test)]
mod tests {
    use crate::aoc::day11::{process1, process2};

    const TEST_MAP: &'static str = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;
    const INPUT_FILE: &'static str = "input/input11";

    #[test]
    fn test() {
        let result = process1(TEST_MAP, 100);
        assert_eq!(result, 1656);
    }

    #[test]
    fn run() {
        let input_map = crate::common::read_to_string(INPUT_FILE);
        let result = process1(&input_map, 100);
        println!("{}", result);
    }

    #[test]
    fn test2() {
        let result = process2(TEST_MAP);
        assert_eq!(result, 195);
    }

    #[test]
    fn run2() {
        let input_map = crate::common::read_to_string(INPUT_FILE);
        let result = process2(&input_map);
        println!("{}", result);
    }
}