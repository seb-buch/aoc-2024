use crate::clawmachine::{load_clawmachines, ClawMachine};
use anyhow::*;
use aoc2024::*;

const DAY: &str = "13";
const SOLUTION_PART_1: &str = "25629";
const SOLUTION_PART_2: &str = "107487112929999";

mod clawmachine {
    use itertools::Itertools;
    const BIAS: isize = 10000000000000isize;

    #[derive(Debug, PartialEq)]
    pub struct ClawMachine {
        pub input_a: (isize, isize),
        pub input_b: (isize, isize),
        pub target: (isize, isize),
    }

    #[derive(Debug, PartialEq)]
    pub struct ClawMachineSolution {
        pub nmoves_a: isize,
        pub nmoves_b: isize,
    }

    impl ClawMachine {
        pub fn solve(&self, biased: bool) -> Option<ClawMachineSolution> {
            let coef_det = self.input_a.0 * self.input_b.1 - self.input_a.1 * self.input_b.0;
            if coef_det == 0 {
                return None;
            }

            let det_x = self.target.0 * self.input_b.1 - self.target.1 * self.input_b.0;
            let det_y = self.target.1 * self.input_a.0 - self.target.0 * self.input_a.1;

            let nmoves_a = det_x / coef_det;
            let remainder_a = det_x % coef_det;
            if remainder_a != 0 {
                return None;
            }

            if nmoves_a < 0 {
                return None;
            }
            if !biased && nmoves_a > 100 {
                return None;
            }
            let nmoves_b = det_y / coef_det;
            let remainder_b = det_y % coef_det;
            if remainder_b != 0 {
                return None;
            }
            if nmoves_b < 0 {
                return None;
            }
            if !biased && nmoves_b > 100 {
                return None;
            }

            Some(ClawMachineSolution { nmoves_a, nmoves_b })
        }

        pub fn compute_cost(&self) -> Option<isize> {
            let solution = self.solve(false)?;

            Some(solution.nmoves_a * 3 + solution.nmoves_b)
        }

        pub fn compute_biased_cost(&self) -> Option<isize> {
            let solution = self.solve(true)?;

            Some(solution.nmoves_a * 3 + solution.nmoves_b)
        }
    }

    fn parse_button(line: &str) -> (isize, isize) {
        let split_1: Vec<&str> = line.split(",").collect::<Vec<_>>();
        let left = split_1[0].split("+").collect::<Vec<_>>()[1];

        let right = &split_1[1].trim()[2..];

        (left.parse().unwrap(), right.parse().unwrap())
    }

    fn parse_target(line: &str) -> (isize, isize) {
        let split_1: Vec<&str> = line.split(",").collect::<Vec<_>>();
        let left = split_1[0].split("=").collect::<Vec<_>>()[1];
        let right = split_1[1].split("=").collect::<Vec<_>>()[1];

        (left.parse().unwrap(), right.parse().unwrap())
    }
    pub fn load_clawmachines(input: &str, biased: bool) -> Vec<ClawMachine> {
        let mut clawmachines = Vec::new();

        for lines in &input.lines().chunks(4) {
            let mut input_a = (0, 0);
            let mut input_b = (0, 0);
            let mut target = (0, 0);
            for (num, line) in lines.enumerate() {
                match num {
                    0 => input_a = parse_button(line),
                    1 => input_b = parse_button(line),
                    2 => target = parse_target(line),
                    _ => {}
                }
            }

            if biased {
                target = (target.0 + BIAS, target.1 + BIAS);
            }

            clawmachines.push(ClawMachine {
                input_a,
                input_b,
                target,
            });
        }
        clawmachines
    }

    #[cfg(test)]
    mod tests_clawmachine {
        use super::*;

        #[test]
        fn should_solve_single_ok() {
            let expected = ClawMachineSolution {
                nmoves_a: 80,
                nmoves_b: 40,
            };

            let machine = ClawMachine {
                input_a: (94, 34),
                input_b: (22, 67),
                target: (8400, 5400),
            };

            let result = machine.solve(false);
            assert!(result.is_some());
            assert_eq!(expected, result.unwrap());
        }

        #[test]
        fn should_solve_single_notok() {
            let machine = ClawMachine {
                input_a: (26, 66),
                input_b: (67, 21),
                target: (12748, 2176),
            };

            let result = machine.solve(false);
            assert!(result.is_none());
        }

        #[test]
        fn should_compute_cost_ok() {
            let expected = 280;

            let machine = ClawMachine {
                input_a: (94, 34),
                input_b: (22, 67),
                target: (8400, 5400),
            };

            let result = machine.compute_cost();
            assert!(result.is_some());
            assert_eq!(expected, result.unwrap());
        }

        #[test]
        fn should_compute_cost_not_ok() {
            let machine = ClawMachine {
                input_a: (26, 66),
                input_b: (67, 21),
                target: (12748, 2176),
            };

            let result = machine.compute_cost();
            assert!(result.is_none());
        }

        #[test]
        fn should_load_clawmachines() {
            let expected = vec![
                ClawMachine {
                    input_a: (94, 34),
                    input_b: (22, 67),
                    target: (8400, 5400),
                },
                ClawMachine {
                    input_a: (26, 66),
                    input_b: (67, 21),
                    target: (12748, 12176),
                },
            ];

            let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";
            let result = load_clawmachines(input, false);

            assert_eq!(expected, result);
        }
    }
}

//region Part 1

fn solve_part_1(input_data: &str) -> Result<String> {
    let clawmachines = load_clawmachines(input_data, false);
    let result: isize = clawmachines
        .iter()
        .map(ClawMachine::compute_cost)
        .map(|solution| solution.unwrap_or(0))
        .sum();

    Ok(format!("{}", result))
}
//endregion

//region Part 2

fn solve_part_2(input_data: &str) -> Result<String> {
    let clawmachines = load_clawmachines(input_data, true);
    let result: isize = clawmachines
        .iter()
        .map(ClawMachine::compute_biased_cost)
        .map(|solution| solution.unwrap_or(0))
        .sum();

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

#[cfg(test)]
mod test_day_13 {
    use crate::clawmachine::{load_clawmachines, ClawMachine};
    #[test]
    fn should_solve_example() {
        let expected = 480;

        let clawmachines = load_clawmachines(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
            false,
        );

        let result: isize = clawmachines
            .iter()
            .map(ClawMachine::compute_cost)
            .map(|solution| solution.unwrap_or(0))
            .sum();

        assert_eq!(expected, result);
    }
}
