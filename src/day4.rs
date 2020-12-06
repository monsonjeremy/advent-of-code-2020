use regex::Regex;
use std::fs;

pub fn part1() -> () {
    let input = fs::read_to_string("day4.txt").expect("Couldn't read file...");

    let required_keys = ["pid:", "iyr:", "eyr:", "byr:", "hgt:", "hcl:", "ecl:"];
    let output: Vec<&str> = input
        .split("\n\n")
        .filter(|item| {
            let mut outcome = true;
            for key in required_keys.iter() {
                if !item.contains(key) {
                    outcome = false;
                }
            }
            outcome
        })
        .collect();

    println!("{}", output.len());
}

pub fn part2() -> () {
    let input = fs::read_to_string("day4.txt").expect("Couldn't read file...");

    let reg = Regex::new(
        r"(byr:(19[2-9][0-9]|200[0-2]))|(iyr:(201[0-9]|2020))|(eyr:(202[0-9]|2030))|(hgt:((1([5-8][0-9]|9[0-3])cm)|(59|6[0-9]|7[0-6])in))|(hcl:\#[a-z0-9]{6})|(ecl:(amb|blu|brn|gry|grn|hzl|oth))|(pid:[0-9]{9}\b)",
    ).expect("Invalid regex");

    let output: Vec<&str> = input
        .split("\n\n")
        .filter(|item| reg.find_iter(item).count() == 7)
        .collect();

    println!("{:#?}", output.len());
}
