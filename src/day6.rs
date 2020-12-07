use std::{collections::HashSet, fs};

pub fn part1() -> () {
    let input: String = fs::read_to_string("day6.txt").expect("Couldn't read file...");

    let sets: Vec<u64> = input
        .split("\n\n")
        .map(|group| {
            let stripped = group.lines().collect::<Vec<&str>>().join("");

            let answers = stripped.chars().collect::<HashSet<char>>();

            answers.len() as u64
        })
        .collect();

    let answer: u64 = sets.iter().sum();
    println!("{}", answer);
}

pub fn part2() -> () {
    let input: String = fs::read_to_string("day6.txt").expect("Couldn't read file...");

    let sets: Vec<u64> = input
        .split("\n\n")
        .map(|group| {
            let answers: Vec<HashSet<char>> = group
                .lines()
                .map(|ans| ans.chars().collect::<HashSet<char>>())
                .collect();

            let result = answers
                .iter()
                .cloned()
                .fold_first(|acc, curr| {
                    acc.intersection(&curr).cloned().collect()
                })
                .expect("Could not fold.")
                .len();

            result as u64
        })
        .collect();

    let answer: u64 = sets.iter().sum();
    println!("{:#?}", answer);
}
