use std::collections::HashSet;
use std::{fs, println};

pub fn create_lines<'a>(input: &'a str) -> Vec<(&'a str, i128)> {
    input
        .lines()
        .map(|line| {
            let tuple = line.split_whitespace().collect::<Vec<&str>>();

            (
                tuple[0],
                tuple[1]
                    .parse::<i128>()
                    .expect(&format!("Could not parse the number.. {}", tuple[1])),
            )
        })
        .collect()
}

pub fn part1() -> () {
    let input: String = fs::read_to_string("day8.txt").expect("Couldn't read file...");
    let lines = create_lines(&input);

    let mut instruction_set = HashSet::new();

    let mut acc = 0;
    let mut instruction_index = 0;

    while !instruction_set.contains(&instruction_index) {
        instruction_set.insert(instruction_index);

        let (inst, arg) = lines[instruction_index as usize];

        if inst == "jmp" {
            instruction_index += arg
        } else if inst == "acc" {
            instruction_index += 1;
            acc += arg
        } else if inst == "nop" {
            instruction_index += 1
        }
    }

    println!("{}", acc);
}

pub fn part2() -> () {
    let input: String = fs::read_to_string("day8.txt").expect("Couldn't read file...");

    let lines = create_lines(&input);

    let will_complete_with_changed_inst = |inst_index: i128, changed_to: &str| -> bool {
        let mut instruction_set = HashSet::new();
        let mut index: i128 = 0;

        while !instruction_set.contains(&index) && index < lines.len() as i128 && index >= 0 {
            instruction_set.insert(index);

            let line = lines[index as usize];

            let (inst, arg) = if inst_index == index {
                (changed_to, line.1)
            } else {
                line
            };

            if inst == "jmp" {
                index += arg
            } else if inst == "acc" {
                index += 1;
            } else if inst == "nop" {
                index += 1
            }
        }

        index >= lines.len() as i128
    };

    let mut acc = 0;
    let mut instruction_index: i128 = 0;

    let mut set = HashSet::new();

    while !(instruction_index as usize >= lines.len()) && !set.contains(&instruction_index) {
        set.insert(instruction_index);
        let (inst, arg) = lines[instruction_index as usize];

        if inst == "jmp" {
            let will_work = will_complete_with_changed_inst(instruction_index.clone(), "nop");

            if will_work.clone() {
                instruction_index += 1
            } else {
                instruction_index += arg
            }
        } else if inst == "acc" {
            instruction_index += 1;
            acc += arg;
        } else if inst == "nop" {
            let will_work = will_complete_with_changed_inst(instruction_index.clone(), "jmp");

            if will_work.clone() {
                instruction_index += arg
            } else {
                instruction_index += 1
            }
        }
    }

    println!("{}", acc);
}
