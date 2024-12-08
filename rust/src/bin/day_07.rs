use anyhow::*;
use aoc2024::*;

const DAY: &str = "07";
const SOLUTION_PART_1: &str = "1298103531759";
const SOLUTION_PART_2: &str = "140575048428831";

struct CalibrationEntry {
    target: usize,
    numbers: Vec<usize>,
}

fn process_input(input: &str) -> Vec<CalibrationEntry> {
    let mut entries: Vec<CalibrationEntry> = Vec::new();

    for line in input.lines() {
        let mut splitted_target = line.split(":");
        let target: usize = splitted_target.next().unwrap().parse().unwrap();
        let numbers: Vec<usize> = splitted_target
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        entries.push(CalibrationEntry { target, numbers })
    }

    entries
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Plus,
    Mult,
    Concat,
}

fn get_possible_results(
    numbers: &[usize],
    left_operators: &[Operator],
    accepted_operators: &[Operator],
) -> Vec<usize> {
    let mut possible_results: Vec<usize> = Vec::new();
    if left_operators.len() < numbers.len() - 1 {
        for operator in accepted_operators {
            let mut left_operators = left_operators.to_vec();
            left_operators.push(*operator);
            possible_results.append(&mut get_possible_results(
                numbers,
                &left_operators,
                &accepted_operators,
            ));
        }

        return possible_results;
    }

    let mut result = numbers[0];
    for i in 0..numbers.len() - 1 {
        let right_operand = numbers[i + 1];
        match left_operators[i] {
            Operator::Plus => result += right_operand,
            Operator::Mult => result *= right_operand,
            Operator::Concat => {
                result = format!("{}{}", result, right_operand)
                    .parse::<usize>()
                    .unwrap()
            }
        }
    }
    possible_results.push(result);

    possible_results
}

fn compute_number_of_calibration(
    target: usize,
    numbers: &[usize],
    accepted_operators: &[Operator],
) -> usize {
    let possible_results = get_possible_results(numbers, &*vec![], accepted_operators);

    let mut target_ok = 0;
    for result_number in possible_results {
        if result_number == target {
            target_ok += 1;
        }
    }

    target_ok
}

fn compute_calibration_result_total(
    entries: &[CalibrationEntry],
    accepted_operators: &[Operator],
) -> usize {
    let mut total = 0;

    for entry in entries {
        if compute_number_of_calibration(entry.target, &entry.numbers, accepted_operators) > 0 {
            total += entry.target;
        }
    }

    total
}

//region Part 1

fn solve_part_1(entries: &[CalibrationEntry]) -> Result<String> {
    Ok(format!(
        "{}",
        compute_calibration_result_total(entries, &[Operator::Plus, Operator::Mult])
    ))
}
//endregion

//region Part 2

fn solve_part_2(entries: &[CalibrationEntry]) -> Result<String> {
    Ok(format!(
        "{}",
        compute_calibration_result_total(
            entries,
            &[Operator::Plus, Operator::Mult, Operator::Concat]
        )
    ))
}

//endregion

fn main() -> Result<()> {
    start_day(DAY);

    let (entries, duration) = time_function!(process_input(get_input_data(DAY)?.as_str()));
    println!("Input data loaded in {}", pretty_duration(duration));

    let (answer_part_1, part1_duration) = time_function!(solve_part_1(&entries)?);
    println!(
        "Part 1: {} (solved in {})",
        answer_part_1,
        pretty_duration(part1_duration)
    );
    check_result(&answer_part_1, SOLUTION_PART_1);

    let (answer_part_2, part2_duration) = time_function!(solve_part_2(&entries)?);
    println!(
        "Part 2: {} (solved in {})",
        answer_part_2,
        pretty_duration(part2_duration)
    );
    check_result(&answer_part_2, SOLUTION_PART_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work_with_one_operator() {
        // Test setup
        let expected_result = 1;

        // Given a target and some numbers
        let target = 190;
        let input = vec![10, 19];

        // When computing the number of possibilities
        let result =
            compute_number_of_calibration(target, &input, &[Operator::Plus, Operator::Mult]);

        // Then the result should be the expected one
        assert_eq!(
            result, expected_result,
            "Expected number of possibilities: {} (actual: {})",
            expected_result, result
        );
    }

    #[test]
    fn should_work_with_two_operators() {
        // Test setup
        let expected_result = 2;

        // Given a target and some numbers
        let target = 3267;
        let input = vec![81, 40, 27];

        // When computing the number of possibilities
        let result =
            compute_number_of_calibration(target, &input, &[Operator::Plus, Operator::Mult]);

        // Then the result should be the expected one
        assert_eq!(
            result, expected_result,
            "Expected number of possibilities: {} (actual: {})",
            expected_result, result
        );
    }

    #[test]
    fn should_get_the_total_calibration_result_for_part1() {
        // Test setup
        let expected_result = 3749;

        // Given some entries
        let entries = process_input(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );

        // When getting the calibration result
        let result = compute_calibration_result_total(&entries, &[Operator::Plus, Operator::Mult]);

        // Then it should be the one expected
        assert_eq!(
            result, expected_result,
            "Expected total calibration result: {} (actual:{})",
            expected_result, result
        );
    }

    #[test]
    fn should_get_the_total_calibration_result_for_part2() {
        // Test setup
        let expected_result = 11387;

        // Given some entries
        let entries = process_input(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );

        // When getting the calibration result
        let result = compute_calibration_result_total(
            &entries,
            &[Operator::Plus, Operator::Mult, Operator::Concat],
        );

        // Then it should be the one expected
        assert_eq!(
            result, expected_result,
            "Expected total calibration result: {} (actual:{})",
            expected_result, result
        );
    }
}
