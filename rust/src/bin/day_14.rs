use crate::bathroom::Bathroom;
use anyhow::*;
use aoc2024::*;

const DAY: &str = "14";
const SOLUTION_PART_1: &str = "229868730";
const SOLUTION_PART_2: &str = "7861";

mod bathroom {

    pub struct Bathroom {
        pub robot_positions: Vec<(i32, i32)>,
        pub robot_velocities: Vec<(i32, i32)>,
        pub size_x: i32,
        pub size_y: i32,
        starting_positions: Vec<(i32, i32)>,
    }

    fn parse_coordinates(coordinates: &str) -> (i32, i32) {
        let raw_coordinates: Vec<&str> = coordinates.split('=').collect::<Vec<_>>()[1]
            .split(",")
            .collect();

        (
            raw_coordinates[0].parse().unwrap(),
            raw_coordinates[1].parse().unwrap(),
        )
    }

    impl Bathroom {
        pub fn new(size_x: i32, size_y: i32) -> Bathroom {
            Bathroom {
                size_x,
                size_y,
                robot_velocities: Vec::new(),
                robot_positions: Vec::new(),
                starting_positions: Vec::new(),
            }
        }

        fn xy_to_index(&self, x: i32, y: i32) -> usize {
            x as usize + y as usize * self.size_x as usize
        }

        pub fn load_robots_from_str(&mut self, input: &str) {
            for line in input.lines() {
                let split = line.split_whitespace().collect::<Vec<&str>>();
                let position = parse_coordinates(split[0]);
                let velocity = parse_coordinates(split[1]);
                self.robot_positions.push(position);
                self.robot_velocities.push(velocity);
                self.xy_to_index(position.0, position.1);
            }
            self.starting_positions = self.robot_positions.clone();
        }

        pub fn tick(&mut self) {
            for (i, position) in self.robot_positions.iter_mut().enumerate() {
                position.0 += self.robot_velocities[i].0;
                position.1 += self.robot_velocities[i].1;

                if position.0 >= self.size_x {
                    position.0 -= self.size_x;
                }
                if position.0 < 0 {
                    position.0 += self.size_x;
                }

                if position.1 >= self.size_y {
                    position.1 -= self.size_y;
                }
                if position.1 < 0 {
                    position.1 += self.size_y;
                }
            }
        }

        fn get_quadrant_population(&self) -> [usize; 4] {
            let mut n_in_first_quadrant = 0;
            let mut n_in_second_quadrant = 0;
            let mut n_in_third_quadrant = 0;
            let mut n_in_fourth_quadrant = 0;

            let middle_x = self.size_x / 2;
            let middle_y = self.size_y / 2;

            for (x, y) in &self.robot_positions {
                if *x < middle_x && *y < middle_y {
                    n_in_first_quadrant += 1;
                }

                if *x > middle_x && *y < middle_y {
                    n_in_second_quadrant += 1;
                }

                if *x < middle_x && *y > middle_y {
                    n_in_third_quadrant += 1;
                }

                if *x > middle_x && *y > middle_y {
                    n_in_fourth_quadrant += 1;
                }
            }

            [
                n_in_first_quadrant,
                n_in_second_quadrant,
                n_in_third_quadrant,
                n_in_fourth_quadrant,
            ]
        }

        pub fn get_safety_factor(&self) -> usize {
            let mut factor = 1;
            for n in self.get_quadrant_population() {
                factor *= n
            }
            factor
        }

        pub fn is_same_as_start(&self) -> bool {
            for (i, (x, y)) in self.robot_positions.iter().enumerate() {
                let (start_x, start_y) = &self.starting_positions[i];

                if !(start_x == x && start_y == y) {
                    return false;
                }
            }

            true
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::bathroom::Bathroom;

        #[test]
        fn should_load_robots() {
            let expected_positions = vec![
                (0, 4),
                (6, 3),
                (10, 3),
                (2, 0),
                (0, 0),
                (3, 0),
                (7, 6),
                (3, 0),
                (9, 3),
                (7, 3),
                (2, 4),
                (9, 5),
            ];
            let expected_velocities = vec![
                (3, -3),
                (-1, -3),
                (-1, 2),
                (2, -1),
                (1, 3),
                (-2, -2),
                (-1, -3),
                (-1, -2),
                (2, 3),
                (-1, 2),
                (2, -3),
                (-3, -3),
            ];

            let mut bathroom = Bathroom::new(11, 7);

            let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

            bathroom.load_robots_from_str(input);
            assert_eq!(expected_positions, bathroom.robot_positions);
            assert_eq!(expected_velocities, bathroom.robot_velocities);
        }

        #[test]
        fn should_tick_simple() {
            let expected_positions = vec![(4, 1)];

            let mut bathroom = Bathroom::new(11, 7);

            let input = "p=2,4 v=2,-3";

            bathroom.load_robots_from_str(input);

            bathroom.tick();

            assert_eq!(expected_positions, bathroom.robot_positions);
        }

        #[test]
        fn should_tick_wrap_x() {
            let expected_positions = vec![(10, 0), (0, 4)];

            let mut bathroom = Bathroom::new(11, 7);

            let input = "p=0,0 v=-1,0
p=10,4 v=1,0";

            bathroom.load_robots_from_str(input);

            bathroom.tick();

            assert_eq!(expected_positions, bathroom.robot_positions);
        }

        #[test]
        fn should_tick_wrap_y() {
            let expected_positions = vec![(0, 6), (4, 0)];

            let mut bathroom = Bathroom::new(11, 7);

            let input = "p=0,0 v=0,-1
p=4,6 v=0,1";

            bathroom.load_robots_from_str(input);

            bathroom.tick();

            assert_eq!(expected_positions, bathroom.robot_positions);
        }

        #[test]
        fn should_compute_safety_factor() {
            let expected = 0;
            let mut bathroom = Bathroom::new(11, 7);
            bathroom.load_robots_from_str(
                "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
            );

            let safety_factor = bathroom.get_safety_factor();
            assert_eq!(expected, safety_factor);
        }
    }
}
//region Part 1

fn solve_part_1(input_data: &str) -> Result<String> {
    let mut bathroom = Bathroom::new(101, 103);
    bathroom.load_robots_from_str(input_data);

    for _ in 0..100 {
        bathroom.tick();
    }

    Ok(format!("{}", bathroom.get_safety_factor()))
}
//endregion

//region Part 2

fn solve_part_2(input_data: &str) -> Result<String> {
    let mut bathroom = Bathroom::new(101, 103);
    bathroom.load_robots_from_str(input_data);

    let mut lowest_safety_score = (0usize, usize::MAX);
    let mut tick_count = 0usize;
    loop {
        tick_count += 1;
        bathroom.tick();

        let safety_score = bathroom.get_safety_factor();

        if safety_score < lowest_safety_score.1 {
            lowest_safety_score = (tick_count, safety_score);
        }

        if bathroom.is_same_as_start() {
            break;
        }
    }

    Ok(format!("{}", lowest_safety_score.0))
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
mod day_14_tests {
    use crate::bathroom::Bathroom;

    #[test]
    fn should_solve_part_1_example() {
        let expected = 12;

        let mut bathroom = Bathroom::new(11, 7);
        bathroom.load_robots_from_str(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
        );

        for _ in 0..100 {
            bathroom.tick();
        }

        assert_eq!(expected, bathroom.get_safety_factor());
    }
}
