use std::{io::{BufRead, BufReader, Read, Lines}, fs::File};

pub fn get_lines_iterator(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).unwrap();
    BufReader::new(file).lines()
}

pub fn read_to_string(file_name: &str) -> String {
    let file = File::open(file_name).unwrap();
    let mut string = String::new();
    BufReader::new(file).read_to_string(&mut string).unwrap();
    string
}