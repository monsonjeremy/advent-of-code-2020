#![feature(iterator_fold_self)]
#![feature(str_split_once)]

use clap::{App, Arg};
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn main() {
    println!("Jeremy's Advent Of Code 2020 Solutions");

    let matches = App::new("jmonson-advent-of-code-2020")
        .version("1.0")
        .author("Jeremy M. <monson.jeremy@gmail.com>")
        .about("Runs my advent of code solutions")
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .about("-d, --day=[DAY_NUMBER] 'Chooses which day's solution to run'")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    match matches.value_of("day").unwrap() {
        "1" => {
            day1::part1();
            day1::part2();
        }
        "2" => {
            day2::part1();
            day2::part2();
        }
        "3" => {
            day3::part1();
            day3::part2();
        }
        "4" => {
            day4::part1();
            day4::part2();
        }
        "5" => {
            day5::part1();
            day5::part2();
        }
        "6" => {
            day6::part1();
            day6::part2();
        }
        "7" => {
            day7::part1();
            day7::part2();
        }
        "8" => {
            day8::part1();
            day8::part2();
        }
        "9" => {
            day9::part1();
            day9::part2();
        }
        _ => println!("Day not found or implemented"),
    }
}
