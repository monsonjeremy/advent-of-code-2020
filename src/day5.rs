use std::fs;

fn calc_row_seat(row: &str, seat: &str) -> (f32, f32) {
    let mut row_result: f32 = 0.0;
    let mut seat_result: f32 = 0.0;

    let (mut row_lower, mut row_upper): (f32, f32) = (0.0, 127.0);
    let (mut seat_lower, mut seat_upper): (f32, f32) = (0.0, 7.0);

    for (index, &character) in row.as_bytes().iter().enumerate() {
        let half: f32 = row_lower + ((row_upper - row_lower) / 2.0);
        let is_final_index = index == 6;
        match character {
            b'F' => {
                if !is_final_index {
                    row_upper = half.floor()
                }
                row_result = row_lower
            }
            b'B' => {
                if !is_final_index {
                    row_lower = half.ceil()
                }
                row_result = row_upper
            }
            _ => {
                println!("Could not match character {:#?}", character.to_string())
            }
        }
    }

    for (index, &character) in seat.as_bytes().iter().enumerate() {
        let half: f32 = seat_lower + ((seat_upper - seat_lower) / 2.0);
        let is_final_index = index == 2;
        match character {
            b'L' => {
                if !is_final_index {
                    seat_upper = half.floor()
                }
                seat_result = seat_lower
            }
            b'R' => {
                if !is_final_index {
                    seat_lower = half.ceil()
                }
                seat_result = seat_upper
            }
            _ => {
                println!("Could not match character {:#?}", character.to_string())
            }
        }
    }

    (row_result, seat_result)
}
pub fn part1() -> () {
    let input: String = fs::read_to_string("day5.txt").expect("Couldn't read file...");

    let output = input
        .split("\n")
        .map(|line| {
            let (row, seat) = line.split_at(7);

            let (row_result, seat_result) = calc_row_seat(row, seat);

            (row_result * 8.0 + seat_result) as u128
        })
        .max()
        .unwrap();

    println!("{:#?}", output);
}

pub fn part2() -> () {
    let input: String = fs::read_to_string("day5.txt").expect("Couldn't read file...");

    let output = input
        .split("\n")
        .filter_map(|line| {
            let (row, seat) = line.split_at(7);

            let (row_result, seat_result) = calc_row_seat(row, seat);

            if row_result == 0.0 || row_result == 127.0 {
                ()
            }
            Some((row_result as u128, seat_result as u128))
        })
        .collect::<Vec<(u128, u128)>>();

    let mut result = 0;
    for row in 5..110 {
        for seat in 0..7 {
            if !output.contains(&(row, seat)) {
                result = row * 8 + seat;
            }
        }
    }
    println!("{}", result)
}
