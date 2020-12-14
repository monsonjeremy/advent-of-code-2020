use std::collections::HashMap;
use std::{fs, println};

type NumVec = Vec<i128>;

pub fn create_lines<'a>() -> NumVec {
    let input: String = fs::read_to_string("day10.txt").expect("Couldn't read file...");

    input
        .lines()
        .map(|line| {
            line.parse::<i128>()
                .expect(&format!("Could not parse number: {}", line))
        })
        .collect()
}

pub fn part1() -> () {
    let mut lines = create_lines();
    lines.sort();

    let adapter_rating = lines[lines.len() - 1] + 3;

    lines.push(adapter_rating);
    let mut curr_jolt = 0;
    let mut selected_adapters: Vec<(&i128, i128)> = vec![];

    loop {
        let viable_adapters = lines.iter().filter(|adapter| {
            let diff = adapter.to_owned() - curr_jolt;
            diff <= 3 && diff > 0
        });

        if viable_adapters.clone().count() == 0 {
            break;
        }

        let min_viable_adapter = viable_adapters.min().unwrap();

        selected_adapters.push((min_viable_adapter, min_viable_adapter - curr_jolt));

        curr_jolt = *min_viable_adapter;

        continue;
    }

    let mut one_count = 0;
    let mut three_count = 0;

    for adapter in selected_adapters {
        if adapter.1 == 1 {
            one_count += 1;
        }
        if adapter.1 == 3 {
            three_count += 1;
        }
    }

    println!("{:#?}", one_count * three_count);
}

pub fn part2() -> () {
    let mut lines = create_lines();
    lines.push(0);
    lines.sort();
    let adapter_rating = lines[lines.len() - 1] + 3;

    lines.push(adapter_rating);

    type MemoTable = HashMap<usize, i128>;
    let mut memo: MemoTable = HashMap::new();

    fn dp(i: usize, lines: &NumVec, memo: &mut MemoTable) -> i128 {
        if i == lines.len() - 1 {
            1
        } else if memo.contains_key(&i) {
            *memo.get(&i).unwrap()
        } else {
            let mut ans: i128 = 0;
            for j in i + 1..i + 4 {
                if j > lines.len() - 1 {
                    break;
                }

                if lines[j] - lines[i] <= 3 {
                    ans += dp(j, lines, memo);
                }
            }
            memo.insert(i, ans);
            ans
        }
    };

    let answer = dp(0, &lines, &mut memo);

    println!("{:#?}", answer);
}
