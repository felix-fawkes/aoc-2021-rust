use std::collections::{BTreeMap, BTreeSet, VecDeque};

use itertools::Itertools;

use crate::common;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y + 1),
            Self::new(self.x, self.y - 1),
        ]
    }
}
struct AreaMap {
    points: BTreeMap<Point, u8>,
}

impl AreaMap {
    fn from_file(path: &str) -> AreaMap {
        let points: BTreeMap<Point, u8> = common::get_lines_iterator(path)
            .map(|line| line.unwrap().trim().to_owned())
            .enumerate()
            .map(|(y, line)| {
                let entries: Vec<_> = line
                    .chars()
                    .enumerate()
                    .map(|(x, ch)| (x, ch.to_string().parse::<u8>().unwrap()))
                    .map(|(x, n)| (Point::new(x as i32, y as i32), n))
                    .collect();
                entries
            })
            .flatten()
            .collect();

        AreaMap { points }
    }

    fn get_neighbors(&self, point: Point) -> Vec<Point> {
        point
            .neighbors()
            .into_iter()
            .filter(|point| self.points.contains_key(point))
            .collect()
    }

    fn get_neighbor_values(&self, point: Point) -> Vec<u8> {
        point
            .neighbors()
            .into_iter()
            .filter_map(|point| self.points.get(&point).cloned())
            .collect()
    }

    fn find_danger_value(&self) -> u32 {
        self.points
            .iter()
            .filter_map(|(point, value)| {
                let neighbor_values = self.get_neighbor_values(*point);
                if neighbor_values.into_iter().all(|val| val > *value) {
                    Some(*value + 1)
                } else {
                    None
                }
            })
            .map(|val| val as u32)
            .sum()
    }

    fn find_low_points(&self) -> Vec<Point> {
        self.points
            .iter()
            .filter_map(|(point, value)| {
                let neighbor_values = self.get_neighbor_values(*point);
                if neighbor_values.into_iter().all(|val| val > *value) {
                    Some(*point)
                } else {
                    None
                }
            })
            .collect()
    }

    fn find_basin_size(&self, low_point: Point) -> u32 {
        let mut visited = BTreeSet::<Point>::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back(low_point);
        while !to_visit.is_empty() {
            let next_point = to_visit.pop_front().unwrap();
            visited.insert(next_point);            
            self.get_neighbors(next_point)
                .into_iter()
                .filter(|point| !visited.contains(point) && self.points[point] != 9)
                .for_each(|point| to_visit.push_back(point));
        }
        visited.len() as u32
    }
}

fn process1(file_name: &str) -> u32 {
    let area_map = AreaMap::from_file(file_name);
    area_map.find_danger_value()
}

fn process2(file_name: &str) -> u32 {
    let area_map = AreaMap::from_file(file_name);
    let low_points = area_map.find_low_points();
    low_points.into_iter()
    .map(|low_point| area_map.find_basin_size(low_point))
    .sorted_by(|a,b| Ord::cmp(b, a))
    .take(3)
    .product()
}

#[cfg(test)]
mod tests {

    const TEST_FILE: &'static str = "input/test9";
    const INPUT_FILE: &'static str = "input/input9";

    #[test]
    fn test() {
        let result = super::process1(TEST_FILE);
        assert_eq!(result, 15);
    }

    #[test]
    fn run() {
        let result = super::process1(INPUT_FILE);
        println!("{}", result);
    }

    #[test]
    fn test2() {
        let result = super::process2(TEST_FILE);
        assert_eq!(result, 1134);
    }

    #[test]
    fn run2() {
        let result = super::process2(INPUT_FILE);
        println!("{}", result);
    }
}
