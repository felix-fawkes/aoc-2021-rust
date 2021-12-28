use std::collections::{HashMap, HashSet};

use common_macros::{hash_map, hash_set};
use itertools::Itertools;
use lazy_static::lazy_static;

use crate::common;
struct State {
    candidates: HashMap<Segment, HashSet<char>>,
    mapping: HashMap<Segment, char>,
}

#[derive(PartialEq, Eq, Hash)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(PartialEq, Eq, Hash)]
enum Numbers {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
}

lazy_static! {
    static ref CHARS: HashMap<Numbers, HashSet<Segment>> = hash_map! {
        Numbers::ZERO => hash_set!{Segment::A, Segment::B, Segment::C, Segment::E, Segment::F, Segment::G},
        Numbers::ONE => hash_set!{Segment::C, Segment::F},
        Numbers::TWO => hash_set!{Segment::A, Segment::C, Segment::D, Segment::E, Segment::G},
        Numbers::THREE => hash_set!{Segment::A, Segment::C, Segment::D, Segment::F, Segment::G},
        Numbers::FOUR => hash_set!{Segment::B, Segment::C, Segment::D, Segment::F},
        Numbers::FIVE => hash_set!{Segment::A, Segment::B, Segment::D, Segment::F, Segment::G},
        Numbers::SIX => hash_set!{Segment::A, Segment::B, Segment::D, Segment::E, Segment::F, Segment::G},
        Numbers::SEVEN => hash_set!{Segment::A, Segment::C, Segment::F},
        Numbers::EIGHT => hash_set!{Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F, Segment::G},
        Numbers::NINE => hash_set!{Segment::A, Segment::B, Segment::C, Segment::D, Segment::F, Segment::G}
    };
    static ref UNIQ_LENGTHS: HashSet<u32> = hash_set! {2, 3, 4, 7};
}

fn get_sets(line: &str) -> HashMap<u32, Vec<HashSet<char>>> {
    line.trim()
        .split_whitespace()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .map(|set| (set.len() as u32, set))
        .into_group_map()
}

fn find_set_235<'a>(
    candidates: &'a [HashSet<char>],
    set_bd: &HashSet<char>,
    set_cf: &HashSet<char>,
) -> (&'a HashSet<char>, &'a HashSet<char>, &'a HashSet<char>) {
    let mut set5 = None;
    let mut set3 = None;
    let mut set2 = None;
    candidates
        .into_iter()
        .map(|candidate| {
            let intersection_bd = candidate.intersection(set_bd).count();
            let intersection_cf = candidate.intersection(set_cf).count();
            (candidate, intersection_bd, intersection_cf)
        })
        .for_each(|(candidate, bd_size, cf_size)| {
            if bd_size == 1 && cf_size == 2 {
                set3 = Some(candidate);
            }
            if bd_size == 1 && cf_size == 1 {
                set2 = Some(candidate);
            }
            if bd_size == 2 && cf_size == 1 {
                set5 = Some(candidate);
            }
        });

    (set2.unwrap(), set3.unwrap(), set5.unwrap())
}

fn find_set_069<'a>(
    candidates: &'a [HashSet<char>],
    set_bd: &HashSet<char>,
    set_cf: &HashSet<char>,
) -> (&'a HashSet<char>, &'a HashSet<char>, &'a HashSet<char>) {
    let mut set0 = None;
    let mut set6 = None;
    let mut set9 = None;
    candidates
        .into_iter()
        .map(|candidate| {
            let intersection_bd = candidate.intersection(set_bd).count();
            let intersection_cf = candidate.intersection(set_cf).count();
            (candidate, intersection_bd, intersection_cf)
        })
        .for_each(|(candidate, bd_size, cf_size)| {
            if bd_size == 1 && cf_size == 2 {
                set0 = Some(candidate);
            }
            if bd_size == 2 && cf_size == 1 {
                set6 = Some(candidate);
            }
            if bd_size == 2 && cf_size == 2 {
                set9 = Some(candidate);
            }
        });

    (set0.unwrap(), set6.unwrap(), set9.unwrap())
}

fn normalize_set(set: &HashSet<char>) -> String {
    set.into_iter().sorted().collect()
}

fn normalize_input_string(input: &str) -> String {
    input.chars().sorted().collect()
}

fn get_number(mapping: &HashMap<String, char>, input: &str) -> u32 {
    let string: String = input
        .trim()
        .split_whitespace()
        .map(|s| mapping.get(&normalize_input_string(s)).unwrap())
        .collect();
    string.parse::<u32>().unwrap()
}

fn process_line2(line: &str) -> u32 {
    let (data, number) = line.split_once("|").unwrap();
    let sets = get_sets(data);
    let set_cf = sets[&2].first().unwrap();
    let set_bd_cf = sets[&4].first().unwrap();
    let set_bd = &set_bd_cf
        .difference(set_cf)
        .cloned()
        .collect::<HashSet<_>>();

    let set_1 = set_cf;
    let set_4 = set_bd_cf;
    let set_7 = sets[&3].first().unwrap();
    let set_8 = sets[&7].first().unwrap();
    let (set_2, set_3, set_5) = find_set_235(&sets[&5], set_bd, set_cf);
    let (set_0, set_6, set_9) = find_set_069(&sets[&6], set_bd, set_cf);

    let mapping = hash_map! {
        normalize_set(set_0) => '0',
        normalize_set(set_1) => '1',
        normalize_set(set_2) => '2',
        normalize_set(set_3) => '3',
        normalize_set(set_4) => '4',
        normalize_set(set_5) => '5',
        normalize_set(set_6) => '6',
        normalize_set(set_7) => '7',
        normalize_set(set_8) => '8',
        normalize_set(set_9) => '9',
    };

    get_number(&mapping, number)
}

fn process_line1(line: &str) -> u32 {
    let (_, text) = line.split_once("|").unwrap();
    text.trim()
        .split_whitespace()
        .filter(|s| UNIQ_LENGTHS.contains(&(s.len() as u32)))
        .count() as u32
}

pub fn process1(file_name: &str) -> u32 {
    common::get_lines_iterator(file_name)
        .map(|line| line.unwrap())
        .map(|line| process_line1(&line))
        .sum()
}

pub fn process2(file_name: &str) -> u32 {
    common::get_lines_iterator(file_name)
    .map(|line| line.unwrap())
    .map(|line| process_line2(&line))
    .sum()
}

#[cfg(test)]
mod tests {

    const TEST_FILE: &'static str = "input/test8";
    const INPUT_FILE: &'static str = "input/input8";

    #[test]
    fn test() {
        let result = super::process1(TEST_FILE);
        assert_eq!(result, 26);
    }

    #[test]
    fn run() {
        let result = super::process1(INPUT_FILE);
        println!("{}", result);
    }

    #[test]
    fn test2() {
        let result = super::process2(TEST_FILE);
        assert_eq!(result, 61229);
    }

    #[test]
    fn run2() {
        let result = super::process2(INPUT_FILE);
        println!("{}", result);
    }
}
