use std::collections::HashSet;
use std::iter::FromIterator;
use std::{fs, println};

pub fn create_lines<'a>() -> Vec<u64> {
    let input: String = fs::read_to_string("day9.txt").expect("Couldn't read file...");

    input
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .expect(&format!("Could not parse number: {}", line))
        })
        .collect()
}

pub fn part1() -> () {
    let lines = create_lines();

    let preamble_size: usize = 25;

    let mut counter: usize = 0;
    let mut index: usize = preamble_size;

    'outer: loop {
        let to_check = &lines[index];
        let preamble = &lines[counter..index];
        let set: HashSet<&u64> = HashSet::from_iter(preamble.iter());

        'inner: for number in preamble {
            if number > to_check {
                continue 'inner;
            }

            let result = to_check - number;

            if set.contains(&result) && set.get(&result) != Some(&number) {
                counter += 1;
                index += 1;
                continue 'outer;
            }
        }

        println!("{:#?}", to_check);
        break;
    }
}

pub fn part2() -> () {
    let number = 105950735;
    let lines = create_lines()
        .iter()
        .cloned()
        .filter(|line| line < &number)
        .collect::<Vec<u64>>();
    let lines_len = lines.len();

    type NumVec<'a> = Vec<&'a u64>;
    let mut nums_to_use: usize = 2;
    let mut sum_vec: NumVec = Vec::new();

    'outer: for index in 0..lines_len {
        'inner: loop {
            if index + nums_to_use >= lines_len {
                nums_to_use = 2;
                continue 'outer;
            }

            let to_sum = &lines[index..index + nums_to_use];
            let sum = to_sum.iter().sum::<u64>();

            if sum == number {
                sum_vec = to_sum.iter().collect::<NumVec>();
                break 'outer;
            }

            nums_to_use += 1;
            continue 'inner;
        }
    }

    sum_vec.sort();

    println!(
        "{:#?}",
        sum_vec[0] + sum_vec.pop().expect("Could not pop last value")
    );
}
