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
    println!("It's friday, i'm going to skip.")
}
