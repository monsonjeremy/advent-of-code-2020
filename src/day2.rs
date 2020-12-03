use crate::utils::{parse_num, read_input};
use regex::Regex;
use std::convert::TryInto;

pub fn part1() -> () {
    let lines = read_input("day2.txt");

    let mut counter: u16 = 0;
    for line in lines {
        let re =
            Regex::new(r"(?P<lower>\d+)-(?P<upper>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)")
                .expect("Could not create regex");
        let caps = re.captures(&line).expect("could not perform regex on line");

        let lower = parse_num(caps.name("lower").unwrap().as_str().to_string());
        let upper = parse_num(caps.name("upper").unwrap().as_str().to_string());
        let letter = caps.name("letter").unwrap().as_str().to_string();
        let password_vec: Vec<&str> = caps
            .name("password")
            .unwrap()
            .as_str()
            .rmatches(&letter)
            .collect();
        let num_matches = password_vec.len().try_into().unwrap();

        if lower <= num_matches && upper >= num_matches {
            counter += 1;
        }
    }

    println!("{}", counter);
}

pub fn part2() -> () {
    let lines = read_input("day2.txt");

    let mut counter: u16 = 0;
    for line in lines {
        let re =
            Regex::new(r"(?P<first>\d+)-(?P<second>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)")
                .expect("Could not create regex");
        let caps = re.captures(&line).expect("could not perform regex on line");

        let first = parse_num(caps.name("first").unwrap().as_str().to_string()) - 1;
        let second = parse_num(caps.name("second").unwrap().as_str().to_string()) - 1;
        let letter = caps.name("letter").unwrap().as_str().to_string();
        let password: Vec<char> = caps.name("password").unwrap().as_str().chars().collect();

        let has_first = password[first as usize].to_string() == letter;
        let has_second = password[second as usize].to_string() == letter;

        let has_char_at_either_index = has_first ^ has_second;

        if has_char_at_either_index {
            counter += 1;
        }
    }

    println!("{}", counter);
}
