use crate::stone::Stoneset;
use anyhow::*;
use aoc2024::*;
use std::str::FromStr;

const DAY: &str = "11";
const SOLUTION_PART_1: &str = "233875";
const SOLUTION_PART_2: &str = "277444936413293";

mod stone {
    use std::collections::HashMap;
    use std::str::FromStr;

    pub struct Stoneset {
        pub stones: HashMap<usize, usize>,
    }

    impl FromStr for Stoneset {
        type Err = anyhow::Error;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            let mut stones: HashMap<usize, usize> = HashMap::new();

            for stone in input.split_whitespace() {
                stones.insert(stone.parse()?, 1);
            }

            Ok(Self { stones })
        }
    }

    impl Stoneset {
        pub fn blink(&mut self) {
            let mut new_stones: HashMap<usize, usize> = HashMap::new();

            for (stone, count) in &self.stones {
                if *stone == 0 {
                    new_stones
                        .entry(1)
                        .and_modify(|value| *value += count)
                        .or_insert(*count);
                    continue;
                }

                let as_str = format!("{}", stone);
                let n_digits = as_str.len();
                if n_digits % 2 == 0 {
                    new_stones
                        .entry(as_str[..n_digits / 2].parse().unwrap())
                        .and_modify(|value| *value += count)
                        .or_insert(*count);
                    new_stones
                        .entry(as_str[n_digits / 2..].parse().unwrap())
                        .and_modify(|value| *value += count)
                        .or_insert(*count);
                    continue;
                }

                new_stones
                    .entry(stone * 2024)
                    .and_modify(|value| *value += count)
                    .or_insert(*count);
            }

            self.stones = new_stones;
        }
    }
}

//region Part 1

fn solve_part_1(input_data: &str) -> Result<String> {
    let mut stoneset = Stoneset::from_str(input_data)?;

    for _ in 0..25 {
        stoneset.blink();
    }

    let n_stones = stoneset.stones.values().sum::<usize>();
    Ok(format!("{}", n_stones))
}
//endregion

//region Part 2

fn solve_part_2(input_data: &str) -> Result<String> {
    let mut stoneset = Stoneset::from_str(input_data)?;

    for _ in 0..75 {
        stoneset.blink();
    }

    let n_stones = stoneset.stones.values().sum::<usize>();
    Ok(format!("{}", n_stones))
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
    use crate::stone::Stoneset;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn should_load_from_str() {
        // Test setup
        let expected = HashMap::from([(125, 1usize), (17, 1)]);

        // Given a string
        let input = "125 17";

        // When we load a stoneset from it
        let stones = Stoneset::from_str(input).unwrap();

        // Then it should correspond to what is expected
        for (stone, count) in expected {
            assert_eq!(count, *stones.stones.get(&stone).unwrap());
        }
    }

    #[test]
    fn should_blink() {
        // Test setup
        let expected = HashMap::from([(253000, 1usize), (1, 1), (7, 1)]);

        // Given a stoneset
        let mut stoneset = Stoneset::from_str("125 17").unwrap();

        // When we blink
        stoneset.blink();

        // Then it should correspond to what is expected
        for (stone, count) in expected {
            assert_eq!(count, *stoneset.stones.get(&stone).unwrap());
        }
    }

    #[test]
    fn should_pass_example() {
        // Test setup
        let expected = 55312;
        let n_blinks = 25;

        // Given a stoneset
        let mut stoneset = Stoneset::from_str("125 17").unwrap();

        // When we blink a few times
        for _ in 0..n_blinks {
            stoneset.blink();
        }

        // Then we whould get the expected total
        assert_eq!(expected, stoneset.stones.values().sum::<usize>());
    }
}
