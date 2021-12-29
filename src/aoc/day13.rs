use itertools::Itertools;
use crate::aoc::day13::implementation::{Operation, Point, State};
use crate::common::get_lines_iterator;

mod implementation {
    use std::borrow::BorrowMut;
    use std::collections::{BTreeSet, HashSet};
    use itertools::Itertools;
    use crate::aoc::day13::Operation::FoldX;

    #[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
    pub struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }

        pub fn from_str(s: &str) -> Point {
            let (x, y) = s.trim().split_once(",").unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            Point::new(x, y)
        }
    }

    pub enum Operation {
        FoldX(i32),
        FoldY(i32),
    }

    impl Operation {
        pub fn from_str(s: &str) -> Self {
            let (direction, n) = s.trim().split_once("=").unwrap();
            let n = n.parse::<i32>().unwrap();
            match direction.trim() {
                "fold along x" => Operation::FoldX(n),
                "fold along y" => Operation::FoldY(n),
                _ => panic!("Unknown operation: {}", direction),
            }
        }
    }

    pub struct State {
        points: BTreeSet<Point>,
    }

    impl State {
        pub fn from_points(points: &[Point]) -> State {
            let points: BTreeSet<_> = points.into_iter().cloned().collect();
            State { points }
        }

        pub fn execute_operation(&mut self, operation: Operation) {
            match operation {
                Operation::FoldX(x) => self.fold_x(x),
                Operation::FoldY(y) => self.fold_y(y),
            }
        }

        pub fn get_point_count(&self) -> usize {
            self.points.len()
        }

        pub fn draw(&self) {
            let max_x = self.points.iter()
                .max_by_key(|point| point.x)
                .map(|point| point.x)
                .unwrap();
            let max_y = self.points.iter()
                .max_by_key(|point| point.y)
                .map(|point| point.y)
                .unwrap();

            for y in 0..=max_y {
                for x in 0..=max_x {
                    let ch = if self.points.contains(&Point::new(x,y)) { '#' } else { '.' };
                    print!("{}", ch);
                }
                println!();
            }
        }

        fn fold_x(&mut self, x: i32) {
            let points = self.points.borrow_mut();
            let to_flip = points.iter().filter(|point| point.x > x).cloned().collect_vec();
            for point in &to_flip {
                points.remove(point);
            }

            to_flip.into_iter()
                .map(|point| Point::new(x - (point.x - x), point.y))
                .for_each(|point| {
                    points.insert(point);
                })
        }

        fn fold_y(&mut self, y: i32) {
            let points = self.points.borrow_mut();
            let to_flip = points.iter().filter(|point| point.y > y).cloned().collect_vec();
            for point in &to_flip {
                points.remove(point);
            }

            to_flip.into_iter()
                .map(|point| Point::new(point.x, y - (point.y - y)))
                .for_each(|point| {
                    points.insert(point);
                })
        }
    }
}

fn process1(file_name: &str) -> usize {
    let mut lines = get_lines_iterator(file_name).map(|line| line.unwrap()).collect_vec();
    let mut split = lines.split_inclusive(|line| line.is_empty());
    let point_lines = split.next().unwrap();
    let operation_lines = split.next().unwrap();

    let points = point_lines.into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| Point::from_str(line))
        .collect_vec();
    let mut state = State::from_points(&points);

    operation_lines.into_iter()
        .map(|line| Operation::from_str(&line))
        .take(1)
        .for_each(|command| {
            state.execute_operation(command);
        });
    state.get_point_count()
}

fn process2(file_name: &str) {
    let mut lines = get_lines_iterator(file_name).map(|line| line.unwrap()).collect_vec();
    let mut split = lines.split_inclusive(|line| line.is_empty());
    let point_lines = split.next().unwrap();
    let operation_lines = split.next().unwrap();

    let points = point_lines.into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| Point::from_str(line))
        .collect_vec();
    let mut state = State::from_points(&points);

    operation_lines.into_iter()
        .map(|line| Operation::from_str(&line))
        .for_each(|command| {
            state.execute_operation(command);
        });
    state.draw();
}

#[cfg(test)]
mod tests {
    use crate::aoc::day13::process2;
    use super::process1;

    const TEST_FILE: &'static str = "input/test13";
    const INPUT_FILE: &'static str = "input/input13";

    #[test]
    fn test() {
        let result = process1(TEST_FILE);
        assert_eq!(result, 17);
    }

    #[test]
    fn run() {
        let result = process1(INPUT_FILE);
        println!("{}", result);
    }

    #[test]
    fn run2() {
        process2(INPUT_FILE);
    }
}