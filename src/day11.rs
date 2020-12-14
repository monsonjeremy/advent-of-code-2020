use std::{fs, println};

const EMPTY: char = 'L';
const OCCUPIED: char = '#';
const FLOOR: char = '.';

type Line = Vec<char>;
type Lines = Vec<Line>;

pub fn create_lines<'a>() -> Lines {
    let input: String = fs::read_to_string("day11.txt").expect("Couldn't read file...");

    let lines = input.lines();

    lines.map(|line| line.chars().collect()).collect()
}

pub fn part1() -> () {
    let mut lines = create_lines();

    fn get_surrounding_seats(lines: &Lines, line_num: usize, seat_num: usize) -> Vec<char> {
        let last_line_index = lines.len() - 1;
        let last_seat_index = lines[0].len() - 1;
        let mut surrounding_seats = vec![];

        let low_line = if line_num == 0 { 0 } else { line_num - 1 };
        let high_line = if line_num == last_line_index {
            line_num
        } else {
            line_num + 1
        };
        let low_seat = if seat_num == 0 { 0 } else { seat_num - 1 };
        let high_seat = if seat_num == last_seat_index {
            seat_num
        } else {
            seat_num + 1
        };

        for i in low_line..=high_line {
            for j in low_seat..=high_seat {
                if i == line_num && j == seat_num {
                    continue;
                }
                surrounding_seats.push(lines[i][j]);
            }
        }

        surrounding_seats
    };

    let mut previous_tot = 0;

    loop {
        let mut total_occ = 0;

        let lines_clone = lines.clone();
        let last_line_index = lines_clone.len() - 1;
        let last_seat_index = lines_clone[0].len() - 1;

        let mut temp_lines = lines.clone();

        for line_num in 0..=last_line_index {
            for seat_num in 0..=last_seat_index {
                let curr_seat = lines_clone[line_num][seat_num];
                let surrounding_seats = get_surrounding_seats(&lines_clone, line_num, seat_num);

                match curr_seat {
                    EMPTY => {
                        let all_empty = surrounding_seats
                            .iter()
                            .cloned()
                            .all(|seat| seat == EMPTY || seat == FLOOR);

                        temp_lines[line_num][seat_num] = if all_empty {
                            total_occ += 1;
                            OCCUPIED
                        } else {
                            EMPTY
                        };
                    }
                    OCCUPIED => {
                        let occ_seats = surrounding_seats
                            .iter()
                            .cloned()
                            .filter(|seat| seat == &OCCUPIED)
                            .count();

                        temp_lines[line_num][seat_num] = if occ_seats >= 4 {
                            EMPTY
                        } else {
                            total_occ += 1;
                            OCCUPIED
                        };
                    }
                    FLOOR => {
                        temp_lines[line_num][seat_num] = FLOOR;
                    }
                    _ => unreachable!(),
                }
            }
        }

        if previous_tot == total_occ {
            break;
        }

        lines = temp_lines;
        previous_tot = total_occ;
    }

    println!("{:#?}", previous_tot);
}

pub fn part2() -> () {
    let mut lines = create_lines();

    let mut previous_tot = 0;

    loop {
        let mut total_occ = 0;

        let lines_clone = lines.clone();
        let last_line_index = lines_clone.len() - 1;
        let last_seat_index = lines_clone[0].len() - 1;

        let mut temp_lines: Lines = vec![];

        for row in 0..=last_line_index {
            let mut new_row: Vec<char> = vec![];
            for col in 0..=last_seat_index {
                let mut seen: Vec<char> = vec![];
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        let mut i: i32 = 1;

                        while ((row as i32) + i * x >= 0
                            && (row as i32) + i * x <= last_line_index as i32)
                            && ((col as i32) + i * y >= 0
                                && (col as i32) + i * y <= last_seat_index as i32)
                        {
                            let curr_row = (row as i32) + i * x;
                            let curr_col = (col as i32) + i * y;
                            let s = lines[curr_row as usize][curr_col as usize];
                            if s != FLOOR {
                                seen.push(s);
                                break;
                            }
                            i += 1;
                        }
                    }
                }
                let s = lines[row][col];

                if s == EMPTY && !seen.contains(&OCCUPIED) {
                    total_occ += 1;
                    new_row.push(OCCUPIED);
                } else if s == OCCUPIED
                    && seen.iter().cloned().filter(|x| x == &OCCUPIED).count() >= 5
                {
                    new_row.push(EMPTY);
                } else {
                    if s == OCCUPIED {
                        total_occ += 1;
                    }
                    new_row.push(s)
                }
            }
            temp_lines.push(new_row);
        }

        if previous_tot == total_occ {
            break;
        }

        lines = temp_lines;
        previous_tot = total_occ;
    }

    println!("{}", previous_tot);
}
