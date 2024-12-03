use anyhow::*;
use aoc2024::*;

const DAY: &str = "NN"; // TODO: Replace with actual day number
const SOLUTION_PART_1: &str = "TODO"; // TODO: Replace with actual value
const SOLUTION_PART_2: &str = "TODO"; // TODO: Replace with actual value

//region Part 1

fn solve_part_1(input_data: &str) -> Result<String> {
    let lines = input_data.lines();
    Ok(format!("{}", lines.count()))
}
//endregion

//region Part 2

fn solve_part_2(input_data: &str) -> Result<String> {
    let lines = input_data.lines();
    Ok(format!("{}", lines.count()))
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
