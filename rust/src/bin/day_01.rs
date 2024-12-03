use anyhow::*;
use aoc2024::*;
use std::collections::HashMap;

const DAY: &str = "01";
const SOLUTION_PART_1: &str = "2344935";
const SOLUTION_PART_2: &str = "27647262";

fn input_to_vecs(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::<u32>::new();
    let mut right = Vec::<u32>::new();

    for line in input.lines() {
        if !line.is_empty() {
            let mut elems = line.split_whitespace();
            left.push(elems.next().unwrap().parse::<u32>().unwrap());
            right.push(elems.next().unwrap().parse::<u32>().unwrap());
        }
    }

    left.sort();
    right.sort();

    (left, right)
}

//region Part 1

fn solve_part_1(input_data: &str) -> Result<String> {
    let (left, right) = input_to_vecs(input_data);

    let mut total = 0;
    for i in 0..left.len() {
        total += left[i].abs_diff(right[i]);
    }
    Ok(format!("{}", total))
}
//endregion

//region Part 2

fn solve_part_2(input_data: &str) -> Result<String> {
    let (left, right) = input_to_vecs(input_data);

    let mut presents_in_right = HashMap::<u32, u32>::new();
    for elem in right.iter() {
        let tmp = presents_in_right.get(elem).unwrap_or(&0);
        presents_in_right.insert(*elem, *tmp + 1);
    }

    let mut result = 0;
    for elem in left {
        result += elem * presents_in_right.get(&elem).unwrap_or(&0);
    }
    Ok(format!("{}", result))
}
//endregion

fn main() -> Result<()> {
    start_day(DAY);

    let (input_data, duration) = time_function!(get_input_data(DAY)?);
    println!("Input data loaded in {}", pretty_duration(duration));

    let (answer_part_1, part1_duration) = time_function!(solve_part_1(&input_data)?);
    println!(
        "Part 1: {} (solved in {})",
        answer_part_1,
        pretty_duration(part1_duration)
    );
    check_result(&answer_part_1, SOLUTION_PART_1);

    let (answer_part_2, part2_duration) = time_function!(solve_part_2(&input_data)?);
    println!(
        "Part 2: {} (solved in {})",
        answer_part_2,
        pretty_duration(part2_duration)
    );
    check_result(&answer_part_2, SOLUTION_PART_2);

    Ok(())
}
