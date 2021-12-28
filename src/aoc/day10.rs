use std::collections::VecDeque;

use itertools::Itertools;

use crate::common;

static OPENING_BRACKETS: phf::Set<char> = phf::phf_set! { '(', '[', '{', '<' };
static CLOSING_BRACKETS: phf::Set<char> = phf::phf_set! { ')', ']', '}', '>' };

static BRACKET_MAP: phf::Map<char, char> = phf::phf_map! {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
};

static POINT_MAP: phf::Map<char, u32> = phf::phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
};

static POINT_MAP2: phf::Map<char, u64> = phf::phf_map! {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4,
};

fn get_line_score(line: &str) -> Option<u32> {
    let mut stack = VecDeque::new();
    for ch in line.trim().chars() {
        if OPENING_BRACKETS.contains(&ch) {
            stack.push_back(ch)
        } else if CLOSING_BRACKETS.contains(&ch) {
            let open_ch = stack.pop_back().unwrap();
            if BRACKET_MAP[&open_ch] != ch {
                return Some(POINT_MAP[&ch]);
            }
        }
    }
    None
}

fn get_line_score2(line: &str) -> Option<u64> {
    let mut stack = VecDeque::new();
    for ch in line.trim().chars() {
        if OPENING_BRACKETS.contains(&ch) {
            stack.push_back(ch)
        } else if CLOSING_BRACKETS.contains(&ch) {
            let open_ch = stack.pop_back().unwrap();
            if BRACKET_MAP[&open_ch] != ch {
                return None;
            }
        }
    }

    Some(
        stack
            .into_iter()
            .rev()
            .map(|ch| BRACKET_MAP[&ch])
            .map(|ch| POINT_MAP2[&ch])
            .fold(0u64, |acc, x| acc * 5 + x),
    )
}

fn process1(file_name: &str) -> u32 {
    common::get_lines_iterator(file_name)
        .map(|line| line.unwrap())
        .filter_map(|line| get_line_score(line.trim()))
        .sum()
}

fn process2(file_name: &str) -> u64 {
    let scores: Vec<u64> = common::get_lines_iterator(file_name)
        .map(|line| line.unwrap())
        .filter_map(|line| get_line_score2(line.trim()))
        .sorted()
        .collect();

    scores[scores.len()/2]
}

#[cfg(test)]
mod tests {

    const TEST_FILE: &'static str = "input/test10";
    const INPUT_FILE: &'static str = "input/input10";

    #[test]
    fn test() {
        let result = super::process1(TEST_FILE);
        assert_eq!(result, 26397);
    }

    #[test]
    fn run() {
        let result = super::process1(INPUT_FILE);
        println!("{}", result);
    }

    #[test]
    fn test2() {
        let result = super::process2(TEST_FILE);
        assert_eq!(result, 288957);
    }

    #[test]
    fn run2() {
        let result = super::process2(INPUT_FILE);
        println!("{}", result);
    }
}
