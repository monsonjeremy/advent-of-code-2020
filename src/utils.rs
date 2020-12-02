use std::{fs, path::Path};

pub fn read_input(filename: impl AsRef<Path>) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Could not parse file.");

    contents.lines().map(|l| l.to_owned()).collect()
}

pub fn parse_num(input: String) -> i32 {
    input.parse().expect("could not parse number")
}
