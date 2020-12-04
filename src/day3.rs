use crate::utils::read_input;

pub fn part1() -> () {
    let lines = read_input("day3.txt");

    let line_len = lines[0].len();
    let mut counter: usize = 0;
    let mut tree_count: u16 = 0;

    for line in lines {
        if counter >= line_len {
            counter = counter - line_len;
        }

        if line.chars().nth(counter).unwrap().to_string() == "#" {
            tree_count += 1;
        }

        counter += 3;
    }

    println!("Part 1: {}", tree_count);
}

pub fn part2() -> () {
    let lines = read_input("day3.txt");

    let line_len = lines[0].len();

    let mut counter1: usize = 0;
    let mut counter2: usize = 0;
    let mut counter3: usize = 0;
    let mut counter4: usize = 0;
    let mut counter5: usize = 0;

    let mut line_num: u16 = 0;
    let mut tree_count1: u128 = 0;
    let mut tree_count2: u128 = 0;
    let mut tree_count3: u128 = 0;
    let mut tree_count4: u128 = 0;
    let mut tree_count5: u128 = 0;

    let overlap_counter = |cntr: &mut usize| -> () {
        let mut len: usize = line_len;
        if cntr >= &mut len {
            *cntr = *cntr - line_len;
        }
    };

    for line in lines {
        line_num += 1;
        let is_odd = line_num % 2 != 0;

        let check_and_inc =
            |cntr: Option<&mut usize>, tree_count: &mut u128, inc_by: usize| -> () {
                match cntr {
                    None => (),
                    Some(cntr) => {
                        if line.chars().nth(*cntr).unwrap().to_string() == "#" {
                            *tree_count += 1;
                        }
                        *cntr += inc_by;
                    }
                }
            };

        overlap_counter(&mut counter1);
        overlap_counter(&mut counter2);
        overlap_counter(&mut counter3);
        overlap_counter(&mut counter4);
        overlap_counter(&mut counter5);

        check_and_inc(Some(&mut counter1), &mut tree_count1, 1);
        check_and_inc(Some(&mut counter2), &mut tree_count2, 3);
        check_and_inc(Some(&mut counter3), &mut tree_count3, 5);
        check_and_inc(Some(&mut counter4), &mut tree_count4, 7);
        check_and_inc(
            if is_odd { Some(&mut counter5) } else { None },
            &mut tree_count5,
            1,
        );
    }

    println!(
        "Part 2: {}",
        tree_count1 * tree_count2 * tree_count3 * tree_count4 * tree_count5
    );
}
