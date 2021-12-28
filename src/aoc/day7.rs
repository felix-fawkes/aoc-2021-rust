use std::collections::BTreeMap;

type Crabs = BTreeMap<i32, i32>;
struct State {
    crabs: Crabs,
}

impl State {
    fn from_str(s: &str) -> Self {
        let mut crabs = Crabs::new();

        s.split(",")
            .map(|st| st.parse::<i32>().unwrap())
            .for_each(|num| {
                *crabs.entry(num).or_insert(0) += 1;
            });

        State { crabs }
    }

    fn compute_cost_for_position(&self, position: i32) -> i32 {
        let crabs = &self.crabs;
        crabs
            .iter()
            .map(|(current_pos, num)| *num * (*current_pos - position).abs())
            .sum()
    }

    fn compute_cost_for_position2(&self, position: i32) -> i32 {
        let crabs = &self.crabs;
        crabs
            .iter()
            .map(|(current_pos, num)| {
                let dist = (*current_pos - position).abs();
                let cost: i32 = (1..=dist).sum();
                cost * num
            })
            .sum()
    }

    fn get_optimal_position(&self) -> i32 {
        let crabs = &self.crabs;
        let min_pos = *crabs.keys().min().unwrap();
        let max_pos = *crabs.keys().max().unwrap();

        (min_pos..max_pos)
            .map(|pos| (pos, self.compute_cost_for_position(pos)))
            .min_by(|x, y| (x.1).cmp(&(y.1)))
            .unwrap()
            .1
    }

    fn get_optimal_position2(&self) -> i32 {
        let crabs = &self.crabs;
        let min_pos = *crabs.keys().min().unwrap();
        let max_pos = *crabs.keys().max().unwrap();

        (min_pos..max_pos)
            .map(|pos| (pos, self.compute_cost_for_position2(pos)))
            .min_by(|x, y| (x.1).cmp(&(y.1)))
            .unwrap()
            .1
    }
}

pub fn process1(input: &str) -> i32 {
    let state = State::from_str(input);
    state.get_optimal_position()
}

pub fn process2(input: &str) -> i32 {
    let state = State::from_str(input);
    state.get_optimal_position2()
}

#[cfg(test)]
mod tests {
    use crate::common;

    const TEST_STRING: &'static str = "16,1,2,0,4,2,7,1,2,14";
    const INPUT_FILE_NAME: &'static str = "input/input7";

    #[test]
    fn test() {
        let input = TEST_STRING;
        let result = super::process1(input);
        assert_eq!(result, 37);
    }

    #[test]
    fn run() {
        let input = common::read_to_string(INPUT_FILE_NAME);
        let result = super::process1(&input);
        println!("{}", result);
    }

    #[test]
    fn test2() {
        let result = super::process2(TEST_STRING);
        assert_eq!(result, 168);
    }

    #[test]
    fn run2() {
        let input = common::read_to_string(INPUT_FILE_NAME);
        let result = super::process2(&input);
        println!("{}", result);
    }
}
