use crate::utils::{parse_num, read_input};

pub fn part1() -> () {
    let lines = read_input("day1.txt");

    for i in 0..lines.len() {
        let number = lines[i].parse::<i32>().unwrap();
        let mut did_find = false;
        let start = i + 1;
        for j in start..lines.len() {
            let second_number = lines[j].parse::<i32>().unwrap();
            if number + second_number == 2020 {
                println!("Solution to part 1: {:#?}", number * second_number);
                did_find = true;
                break;
            }
        }
        if did_find {
            break;
        }
    }
}

pub fn part2() -> () {
    let lines = read_input("day1.txt");
    let mut did_find: bool = false;

    for i in 0..lines.len() {
        let number = parse_num(lines[i].clone());

        let num_lines = lines.len();

        let first_num = i + 1;
        let second_num = i + 2;

        for j in first_num..num_lines {
            let second_number = parse_num(lines[j].clone());

            for k in second_num..num_lines {
                let third_number = parse_num(lines[k].clone());
                let sum = number + second_number + third_number;
                if sum == 2020 {
                    println!(
                        "Solution to part 2: {:#?}",
                        number * second_number * third_number
                    );
                    did_find = true;
                    break;
                }
            }

            if did_find {
                break;
            }
        }

        if did_find {
            break;
        }
    }
}
