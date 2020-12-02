use clap::{Arg, App};
mod utils;
mod day1;

fn main() {
    println!("Jeremy's Advent Of Code 2020 Solutions");

    let matches = App::new("jmonson-advent-of-code-2020")
        .version("1.0")
        .author("Jeremy M. <monson.jeremy@gmail.com>")
        .about("Runs my advent of code solutions")
        .arg(Arg::new("day")
            .short('d')
            .long("day")
            .about("-d, --day=[DAY_NUMBER] 'Chooses which day's solution to run'")
            .required(true)
            .takes_value(true))
        .get_matches();

    match matches.value_of("day").unwrap() {
        "1" => {
            day1::part1();
            day1::part2();
        },
        _ => println!("Day not found or implemented")
    }
}
