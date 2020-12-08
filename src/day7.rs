use std::collections::HashSet;
use std::{fs, println};

const GOLD_BAG: &str = "shiny gold";

pub fn recurse_through_bags<'a>(
    set: &mut HashSet<&'a str>,
    list: &Vec<(&'a str, String)>,
    bag_to_check: &str,
) {
    for output in list {
        if output.1.contains(bag_to_check) {
            set.insert(output.0);
            recurse_through_bags(set, &list, output.0)
        } else {
            ()
        }
    }
}

pub fn part1() -> () {
    let input: String = fs::read_to_string("day7.txt").expect("Couldn't read file...");

    let mut bag_set = HashSet::new();

    let lines = input.lines();
    let output: Vec<(&str, String)> = lines
        .map(|line| {
            let bag_and_contents: Vec<&str> = line.split(" contain ").collect();
            let (bag, contents) = (bag_and_contents[0], bag_and_contents[1]);

            (bag.trim_end_matches('s'), contents.replace("bags", "bag"))
        })
        .collect();

    recurse_through_bags(&mut bag_set, &output, GOLD_BAG);

    println!("{:#?}", bag_set.len());
}

pub fn recurse_part2<'a>(list: &Vec<(String, String)>, bag_to_check: &str, num_bags: u128) -> u128 {
    let (_, contents) = list
        .iter()
        .find(|bags| bags.0.contains(bag_to_check))
        .expect(&format!("Could not find bag {}", bag_to_check));

    if contents.contains("no other bag") {
        num_bags
    } else {
        let total_bags: u128 = contents
            .split(", ")
            .map(|bag| {
                let (num_str, color) = bag.split_once(" ").expect("Could not split string.");

                let trimmed_color = color.trim_end_matches(".");

                let num: u128 = num_str.parse().unwrap();

                recurse_part2(list, trimmed_color, num)
            })
            .sum();

        if bag_to_check == GOLD_BAG {
            total_bags
        } else {
            num_bags + (num_bags * total_bags)
        }
    }
}

pub fn part2() -> () {
    let input: String = fs::read_to_string("day7.txt").expect("Couldn't read file...");

    let lines = input.lines();
    let output: Vec<(String, String)> = lines
        .map(|line| {
            let replaced = line.replace("bags", "bag");
            let bag_and_contents: Vec<&str> = replaced.split(" contain ").collect();

            let (bag, contents) = (bag_and_contents[0], bag_and_contents[1]);

            (bag.to_owned(), contents.to_owned())
        })
        .collect();

    let result = recurse_part2(&output, GOLD_BAG, 1);
    println!("{:#?}", result);
}
