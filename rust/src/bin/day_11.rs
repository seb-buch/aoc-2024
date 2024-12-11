use crate::stones::StoneSet;
use anyhow::*;
use aoc2024::*;
use std::str::FromStr;

const DAY: &str = "11";
const SOLUTION_PART_1: &str = "233875";
const SOLUTION_PART_2: &str = "TODO"; // TODO: Replace with actual value

mod stones {
    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    pub struct StoneSet {
        pub stones: Vec<usize>,
    }

    impl FromStr for StoneSet {
        type Err = anyhow::Error;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            let stones = input
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            Ok(Self { stones })
        }
    }

    impl StoneSet {
        pub fn blink(&mut self) {
            let mut new_stones: Vec<usize> = Vec::new();

            for num in self.stones.iter() {
                if *num == 0 {
                    new_stones.push(1);
                    continue;
                }

                let as_str = format!("{}", num);
                let n_digits = as_str.len();
                if n_digits % 2 == 0 {
                    new_stones.push(as_str[..n_digits / 2].parse().unwrap());
                    new_stones.push(as_str[n_digits / 2..].parse().unwrap());
                    continue;
                }

                new_stones.push(num * 2024);
            }

            self.stones = new_stones;
        }

        pub fn blinks(&mut self, n_blinks: usize) {
            for _ in 0..n_blinks {
                self.blink();
            }
        }
    }
}

//region Part 1

fn solve_part_1(input_data: &str) -> Result<String> {
    let mut stone_set = StoneSet::from_str(input_data)?;
    stone_set.blinks(25);
    Ok(format!("{}", stone_set.stones.len()))
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

#[cfg(test)]
mod tests {
    use crate::stones::StoneSet;
    #[test]
    fn should_load_from_str() {
        let expected = StoneSet {
            stones: vec![125, 17],
        };

        let input = "125 17\n";

        let actual = input.parse::<StoneSet>().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_blink_example1() {
        let expected = [253000usize, 1, 7];

        let mut stone_set = StoneSet {
            stones: vec![125, 17],
        };

        stone_set.blink();

        assert_eq!(expected, *stone_set.stones);
    }

    #[test]
    fn should_blink_example2() {
        let expected = [512, 72, 2024, 2, 0, 2, 4, 2867, 6032];

        let mut stone_set = StoneSet {
            stones: vec![512072, 1, 20, 24, 28676032],
        };

        stone_set.blink();

        assert_eq!(expected, *stone_set.stones);
    }

    #[test]
    fn should_blinks_example1() {
        let expected = 55312;

        let mut stone_set = StoneSet {
            stones: vec![125, 17],
        };

        stone_set.blinks(25);

        assert_eq!(expected, stone_set.stones.len());
    }
}
