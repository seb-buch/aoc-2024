use anyhow::*;
use aoc2024::*;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

const DAY: &str = "06";
const SOLUTION_PART_1: &str = "4559";
const SOLUTION_PART_2: &str = "1604";

#[derive(Debug, PartialEq, Eq, Hash, Default, Copy, Clone)]
struct Position {
    row: i32,
    col: i32,
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
enum Direction {
    #[default]
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
enum GuardPatrolStatus {
    #[default]
    NOTSTARTED,
    OnGoing,
    Finished,
    StuckInLoop,
}

#[derive(Default, Debug, Copy, Clone)]
enum GuardMoveOutcome {
    #[default]
    BLOCKED,
    Exited,
}

#[derive(Debug, PartialEq)]
struct GuardStop {
    pos: Position,
    direction: Direction,
}

#[derive(Debug, Default, PartialEq)]
struct Guard {
    position: Position,
    initial_position: Position,
    direction: Direction,
    trail: Vec<Position>,
}

#[derive(Debug, Default, PartialEq)]
struct Obstacle {
    position: Position,
}

#[derive(Debug, PartialEq)]
struct Map {
    height: usize,
    width: usize,
    obstacles: HashMap<usize, Obstacle>,
}

impl Map {
    fn is_position_an_obstacle(&self, position: &Position) -> bool {
        if self.is_position_outside(position) {
            return false;
        }
        self.obstacles
            .contains_key(&(self.width * position.row as usize + position.col as usize))
    }

    fn is_position_outside(&self, position: &Position) -> bool {
        match position.row {
            row if row >= self.height as i32 => return true,
            row if row < 0 => return true,
            _ => {}
        }

        match position.col {
            col if col >= self.width as i32 => return true,
            col if col < 0 => return true,
            _ => {}
        }

        false
    }

    fn add_obstacle(&mut self, position: &Position) {
        let key = self.width * position.row as usize + position.col as usize;
        self.obstacles.insert(
            key,
            Obstacle {
                position: Position {
                    row: position.row,
                    col: position.col,
                },
            },
        );
    }

    fn remove_obstacle(&mut self, position: &Position) {
        let key = self.width * position.row as usize + position.col as usize;
        self.obstacles.remove(&key);
    }
}

#[derive(Debug, PartialEq)]
struct GuardPatrol {
    guard: Guard,
    map: Map,
    guard_stops: Vec<GuardStop>,
    status: GuardPatrolStatus,
}

impl GuardPatrol {
    fn new(guard: Guard, map: Map) -> Self {
        Self {
            guard,
            map,
            guard_stops: Vec::new(),
            status: GuardPatrolStatus::NOTSTARTED,
        }
    }

    fn from_input(input: &str) -> Self {
        let mut height = 0;
        let mut width = 0;
        let mut obstacles: HashMap<usize, Obstacle> = HashMap::new();
        let mut guard: Option<Guard> = None;

        for (row, line) in input.lines().enumerate() {
            height += 1;
            width = line.len();

            for (col, char) in line.chars().enumerate() {
                match char {
                    '#' => {
                        obstacles.insert(
                            width * row + col,
                            Obstacle {
                                position: Position {
                                    row: row as i32,
                                    col: col as i32,
                                },
                            },
                        );
                    }
                    '^' => {
                        guard = Some(Guard::new(Position {
                            row: row as i32,
                            col: col as i32,
                        }));
                    }
                    _ => {}
                }
            }
        }

        GuardPatrol::new(
            guard.unwrap(),
            Map {
                height,
                width,
                obstacles,
            },
        )
    }

    fn reset(&mut self) {
        self.guard.reset();
        self.guard_stops.clear();
        self.status = GuardPatrolStatus::NOTSTARTED;
    }

    fn tick(&mut self) {
        if matches!(self.status, GuardPatrolStatus::Finished)
            || matches!(self.status, GuardPatrolStatus::StuckInLoop)
        {
            return;
        }

        if matches!(self.status, GuardPatrolStatus::NOTSTARTED) {
            self.save_guard_stop();
            self.status = GuardPatrolStatus::OnGoing;
        }

        let move_outcome = self.guard.move_on_map(&self.map);

        if self.guard_stops.contains(&GuardStop {
            pos: self.guard.position,
            direction: self.guard.direction,
        }) {
            self.status = GuardPatrolStatus::StuckInLoop;
            return;
        }
        self.save_guard_stop();

        if let GuardMoveOutcome::Exited = move_outcome {
            self.status = GuardPatrolStatus::Finished;
        }
    }

    fn run(&mut self) {
        self.reset();
        loop {
            self.tick();
            if self.status != GuardPatrolStatus::OnGoing {
                break;
            }
        }
    }

    fn save_guard_stop(&mut self) {
        self.guard_stops.push(GuardStop {
            pos: self.guard.position,
            direction: self.guard.direction,
        });
    }

    fn get_number_of_distinct_positions(&self) -> usize {
        let distinct_positions = self.guard.trail.iter().collect::<HashSet<_>>();
        distinct_positions.len()
    }

    fn find_positions_for_infinite_loop(&mut self) -> Vec<Position> {
        // Run patrol to get the initial trail
        self.run();

        let initial_trail = &self.guard.trail.clone();

        let mut triggering_positions: Vec<Position> = Vec::new();
        for position in initial_trail.iter().collect::<HashSet<_>>() {
            self.map.add_obstacle(position);
            self.reset();
            self.run();

            if self.status == GuardPatrolStatus::StuckInLoop {
                triggering_positions.push(*position);
            }

            self.map.remove_obstacle(position);
        }

        triggering_positions
    }
}

impl Guard {
    fn new(position: Position) -> Self {
        Self {
            position,
            initial_position: position,
            direction: Default::default(),
            trail: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.position = self.initial_position;
        self.direction = Default::default();
        self.trail.clear();
    }

    fn move_on_map(&mut self, map: &Map) -> GuardMoveOutcome {
        if self.trail.is_empty() {
            self.trail.push(self.position);
        }

        loop {
            let new_position = match self.direction {
                Direction::UP => Position {
                    row: self.position.row - 1,
                    col: self.position.col,
                },
                Direction::RIGHT => Position {
                    row: self.position.row,
                    col: self.position.col + 1,
                },
                Direction::DOWN => Position {
                    row: self.position.row + 1,
                    col: self.position.col,
                },
                Direction::LEFT => Position {
                    row: self.position.row,
                    col: self.position.col - 1,
                },
            };

            if map.is_position_an_obstacle(&new_position) {
                self.turn();
                return GuardMoveOutcome::BLOCKED;
            }

            if map.is_position_outside(&new_position) {
                return GuardMoveOutcome::Exited;
            }

            self.position = new_position;
            self.trail.push(new_position);
        }
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

//region Part 1

fn solve_part_1(input_data: &str) -> Result<String> {
    let mut patrol = GuardPatrol::from_input(input_data);
    patrol.run();
    Ok(format!("{}", patrol.get_number_of_distinct_positions()))
}
//endregion

//region Part 2

fn solve_part_2(input_data: &str) -> Result<String> {
    let mut patrol = GuardPatrol::from_input(input_data);
    let positions = patrol.find_positions_for_infinite_loop();

    Ok(format!("{}", positions.len()))
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
    use crate::{Direction, GuardPatrol, GuardPatrolStatus, GuardStop, Position};

    fn get_input_data() -> String {
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .into()
    }

    #[test]
    fn should_create_patrol_from_data() {
        // Test setup
        let input_data = get_input_data();
        let expected_width: usize = 10;
        let expected_height: usize = 10;
        let expected_nobstacles: usize = 8;
        let expected_position = Position { row: 6, col: 4 };

        // Given a patrol created for an input
        let patrol = GuardPatrol::from_input(&input_data);

        // then the map should be created correctly
        let map = patrol.map;
        assert_eq!(
            map.width, expected_width,
            "map width is expected to be {} (actual: {})",
            expected_width, map.width
        );
        assert_eq!(
            map.height, expected_height,
            "map height is expected to be {} (actual: {})",
            expected_height, map.height
        );
        assert_eq!(
            map.obstacles.len(),
            expected_nobstacles,
            "number or obstacles is expected to be {} (actual: {})",
            expected_nobstacles,
            map.obstacles.len()
        );

        // And the quard should be also created correctly
        let guard = patrol.guard;
        assert_eq!(
            guard.position, expected_position,
            "guard is expected to be at position {:?} (actual: {:?}",
            expected_position, guard.position
        );
        assert_eq!(guard.direction, Direction::UP);
    }

    #[test]
    fn should_have_guard_to_first_obstacle_on_first_tick() {
        // Test setup
        let input_data = get_input_data();
        let expected_position = Position { row: 1, col: 4 };
        let expected_direction = Direction::RIGHT;
        let expected_trail = [
            Position { row: 6, col: 4 },
            Position { row: 5, col: 4 },
            Position { row: 4, col: 4 },
            Position { row: 3, col: 4 },
            Position { row: 2, col: 4 },
            Position { row: 1, col: 4 },
        ];

        // Given a new patrol
        let mut patrol = GuardPatrol::from_input(&input_data);

        // When the first tick is called
        patrol.tick();

        // Then the guard should be positioned and oriented correctly
        let guard = patrol.guard;
        assert_eq!(
            guard.position, expected_position,
            "Guard's position is expected to be at position {:?} (actual: {:?}",
            expected_position, guard.position
        );
        assert_eq!(
            guard.direction, expected_direction,
            "Guard's direction is expected to be {:?} (actual: {:?})",
            expected_direction, guard.direction
        );

        // And the trail should be the one expcted
        assert_eq!(
            guard.trail, expected_trail,
            "Guard's trail differs from expected trail"
        );
    }

    #[test]
    fn should_finish_on_expected_ending_position_on_run() {
        // Test setup
        let input_data = get_input_data();
        let expected_position = Position { row: 9, col: 7 };
        let expected_stops = [
            GuardStop {
                pos: Position { row: 6, col: 4 },
                direction: Direction::UP,
            },
            GuardStop {
                pos: Position { row: 1, col: 4 },
                direction: Direction::RIGHT,
            },
            GuardStop {
                pos: Position { row: 1, col: 8 },
                direction: Direction::DOWN,
            },
            GuardStop {
                pos: Position { row: 6, col: 8 },
                direction: Direction::LEFT,
            },
            GuardStop {
                pos: Position { row: 6, col: 2 },
                direction: Direction::UP,
            },
            GuardStop {
                pos: Position { row: 4, col: 2 },
                direction: Direction::RIGHT,
            },
            GuardStop {
                pos: Position { row: 4, col: 6 },
                direction: Direction::DOWN,
            },
            GuardStop {
                pos: Position { row: 8, col: 6 },
                direction: Direction::LEFT,
            },
            GuardStop {
                pos: Position { row: 8, col: 1 },
                direction: Direction::UP,
            },
            GuardStop {
                pos: Position { row: 7, col: 1 },
                direction: Direction::RIGHT,
            },
            GuardStop {
                pos: Position { row: 7, col: 7 },
                direction: Direction::DOWN,
            },
            GuardStop {
                pos: Position { row: 9, col: 7 },
                direction: Direction::DOWN,
            },
        ];
        let expected_outcome = GuardPatrolStatus::Finished;

        // Given a new patrol
        let mut patrol = GuardPatrol::from_input(&input_data);

        // When run is called
        patrol.run();

        // Then it should end as expected
        assert_eq!(
            patrol.status, expected_outcome,
            "Expected outcome: {:?} (actual: {:?}",
            expected_outcome, patrol.status
        );

        // And the guard should be at the expected position
        assert_eq!(
            patrol.guard.position, expected_position,
            "Expected guard's position: {:?} (actual: {:?})",
            expected_position, patrol.guard.position
        );

        // And the guard should have stopped to all the expected stops
        for (i, expected_stop) in expected_stops.iter().enumerate() {
            let guard_stop = patrol.guard_stops.get(i).unwrap();
            assert_eq!(
                guard_stop, expected_stop,
                "Guard's stop #{} differs from expected one (expected: {:?}, actual: {:?})",
                i, expected_stop, guard_stop
            )
        }
    }

    #[test]
    fn should_compute_number_of_distinct_positions() {
        // Test setup
        let input_data = get_input_data();
        let expected_n_distinct_positions = 41;

        // Given a new patrol
        let mut patrol = GuardPatrol::from_input(&input_data);

        // When run is called and the patrol is finished
        patrol.run();
        assert_eq!(patrol.status, GuardPatrolStatus::Finished);

        // Then the number of distinct position should be the have expected
        let n_positions = patrol.get_number_of_distinct_positions();
        assert_eq!(
            n_positions, expected_n_distinct_positions,
            "Expected number of distinct positions: {} (actual: {})",
            n_positions, expected_n_distinct_positions
        );
    }

    #[test]
    fn should_detect_infinite_loop() {
        // Test setup
        let input_data = get_input_data();
        let obstacle_position = Position { row: 6, col: 3 };
        let expected_outcome = GuardPatrolStatus::StuckInLoop;

        // Given a new patrol that will triggered an infinite loop
        let mut patrol = GuardPatrol::from_input(&input_data);
        patrol.map.add_obstacle(&obstacle_position);

        // When we run the patrol
        patrol.run();

        // Then the outcome should reflects the infinite loop
        assert_eq!(
            patrol.status, expected_outcome,
            "Expected outcome: {:?} (actual: {:?})",
            expected_outcome, patrol.status
        );
    }

    #[test]
    fn should_find_positions_for_infinite_loop() {
        // Test setup
        let input_data = get_input_data();
        let expected_number = 6;

        // Given a new patrol that will triggered an infinite loop
        let mut patrol = GuardPatrol::from_input(&input_data);

        // When we find the positions that triggers an infinite loop
        let positions = patrol.find_positions_for_infinite_loop();

        assert_eq!(
            positions.len(),
            expected_number,
            "Expected number of positions: {} (actual: {})",
            expected_number,
            positions.len()
        );
    }
}
