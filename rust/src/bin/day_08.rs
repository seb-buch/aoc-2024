use anyhow::*;
use aoc2024::*;
use std::collections::{HashMap, HashSet};
use std::result::Result::Ok;

const DAY: &str = "08";
const SOLUTION_PART_1: &str = "341";
const SOLUTION_PART_2: &str = "1134";

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
struct Position {
    row: isize,
    col: isize,
}

#[derive(PartialEq, Debug, Clone)]
struct AntennaNetwork {
    width: usize,
    height: usize,
    frequency: char,
    antennas: Vec<Position>,
}

impl AntennaNetwork {
    fn new(frequency: char, height: usize, width: usize) -> AntennaNetwork {
        Self {
            width,
            height,
            frequency,
            antennas: vec![],
        }
    }

    fn add_antenna(&mut self, antenna: Position) {
        self.antennas.push(antenna);
    }

    fn compute_antinode_for_antenna_couple(
        &self,
        reference: &Position,
        other: &Position,
    ) -> Option<Position> {
        let delta_row = reference.row - other.row;
        let row = reference.row + delta_row;

        if row < 0 || row >= self.height as isize {
            return None;
        }

        let delta_col = reference.col - other.col;
        let col = reference.col + delta_col;

        if col < 0 || col >= self.width as isize {
            return None;
        }

        Some(Position { row: row, col: col })
    }

    fn is_inside_network(&self, position: &Position) -> bool {
        if position.row < 0 || position.row >= self.height as isize {
            return false;
        }
        if position.col < 0 || position.col >= self.width as isize {
            return false;
        }
        true
    }

    fn compute_antinode_harmonics_for_antenna_couple(
        &self,
        reference: &Position,
        other: &Position,
    ) -> Vec<Position> {
        let mut harmonics: Vec<Position> = vec![];

        let delta_row = reference.row - other.row;
        let delta_col = reference.col - other.col;

        let mut resonance = 0;

        loop {
            let mut added = 0;

            let first_antinode = Position {
                row: reference.row + delta_row * resonance,
                col: reference.col + delta_col * resonance,
            };
            if self.is_inside_network(&first_antinode) {
                added += 1;
                harmonics.push(first_antinode);
            }
            let second_antinode = Position {
                row: other.row - delta_row * resonance,
                col: other.col - delta_col * resonance,
            };
            if self.is_inside_network(&second_antinode) {
                added += 1;
                harmonics.push(second_antinode);
            }

            if added == 0 {
                break;
            }
            resonance += 1;
        }

        harmonics
    }

    fn compute_antinodes(&self) -> Vec<Position> {
        let mut antinodes: Vec<Position> = vec![];

        for i in 0..self.antennas.len() {
            let antenna_1 = &self.antennas[i];
            for j in i + 1..self.antennas.len() {
                let antenna_2 = &self.antennas[j];

                if let Some(position) =
                    self.compute_antinode_for_antenna_couple(antenna_1, antenna_2)
                {
                    antinodes.push(position);
                }

                if let Some(position) =
                    self.compute_antinode_for_antenna_couple(antenna_2, antenna_1)
                {
                    antinodes.push(position);
                }
            }
        }
        antinodes
    }

    fn compute_antinodes_with_harmonics(&self) -> Vec<Position> {
        let mut antinodes: Vec<Position> = vec![];

        for i in 0..self.antennas.len() {
            let antenna_1 = &self.antennas[i];
            for j in i + 1..self.antennas.len() {
                let antenna_2 = &self.antennas[j];

                for position in
                    self.compute_antinode_harmonics_for_antenna_couple(antenna_1, antenna_2)
                {
                    if !antinodes.contains(&position) {
                        antinodes.push(position);
                    }
                }
            }
        }

        antinodes
    }
}

fn process_input(input_data: &str) -> Vec<AntennaNetwork> {
    let mut grid: HashMap<char, AntennaNetwork> = HashMap::new();

    let lines: Vec<&str> = input_data.lines().collect();
    let height = lines.len();
    for (row, line) in lines.iter().enumerate() {
        let width = line.len();

        for (col, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }

            let position = Position {
                row: row as isize,
                col: col as isize,
            };

            match grid.get_mut(&char) {
                None => {
                    let mut network = AntennaNetwork::new(char, height, width);
                    network.add_antenna(position);
                    grid.insert(char, network);
                }
                Some(network) => {
                    network.add_antenna(position);
                }
            };
        }
    }
    Vec::from_iter(grid.values().cloned())
}

fn compute_number_of_antinodes_for_networks(networks: &[AntennaNetwork]) -> usize {
    let mut antinodes: HashSet<Position> = HashSet::new();

    for network in networks {
        for position in &network.compute_antinodes() {
            antinodes.insert(*position);
        }
    }
    antinodes.len()
}

fn compute_number_of_harmonical_antinodes_for_networks(networks: &[AntennaNetwork]) -> usize {
    let mut antinodes: HashSet<Position> = HashSet::new();
    for network in networks {
        for position in &network.compute_antinodes_with_harmonics() {
            antinodes.insert(*position);
        }
    }
    antinodes.len()
}

//region Part 1

fn solve_part_1(networks: &[AntennaNetwork]) -> Result<String> {
    Ok(format!(
        "{}",
        compute_number_of_antinodes_for_networks(networks)
    ))
}
//endregion

//region Part 2

fn solve_part_2(networks: &[AntennaNetwork]) -> Result<String> {
    Ok(format!(
        "{}",
        compute_number_of_harmonical_antinodes_for_networks(networks)
    ))
}
//endregion

fn main() -> Result<()> {
    start_day(DAY);

    let (networks, duration) = time_function!(process_input(&*get_input_data(DAY)?));
    println!("Input data loaded in {}", pretty_duration(duration));

    let (answer_part_1, part1_duration) = time_function!(solve_part_1(&networks)?);
    println!(
        "Part 1: {} (solved in {})",
        answer_part_1,
        pretty_duration(part1_duration)
    );
    check_result(&answer_part_1, SOLUTION_PART_1);

    let (answer_part_2, part2_duration) = time_function!(solve_part_2(&networks)?);
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
    fn should_process_input() {
        // Test setup
        let expected_networks = vec![
            AntennaNetwork {
                frequency: '0',
                height: 12,
                width: 12,
                antennas: vec![
                    Position { row: 1, col: 8 },
                    Position { row: 2, col: 5 },
                    Position { row: 3, col: 7 },
                    Position { row: 4, col: 4 },
                ],
            },
            AntennaNetwork {
                frequency: 'A',
                height: 12,
                width: 12,
                antennas: vec![
                    Position { row: 5, col: 6 },
                    Position { row: 8, col: 8 },
                    Position { row: 9, col: 9 },
                ],
            },
        ];

        // Given an input
        let input_data = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        // When processing it
        let networks = process_input(input_data);

        // Then we should obtain the expected networks
        for expected_network in expected_networks {
            for network in networks.iter() {
                if expected_network.frequency == network.frequency {
                    assert_eq!(expected_network, *network);
                }
            }
        }
    }

    #[test]
    fn should_compute_antinode_simple() {
        // Test setup
        let expected_positions = vec![Position { row: 1, col: 3 }, Position { row: 7, col: 6 }];

        // Given an antenna network
        let mut network = AntennaNetwork::new('a', 10, 10);
        network.add_antenna(Position { row: 3, col: 4 });
        network.add_antenna(Position { row: 5, col: 5 });

        // When computing the antinode positions
        let positions = network.compute_antinodes();

        // Then they should correspond to the expected ones
        assert_eq!(
            positions, expected_positions,
            "Expected antinodes: {:?} (actual: {:?})",
            expected_positions, positions
        );
    }

    #[test]
    fn should_compute_antinode_with_invalid_antinodes() {
        // Test setup
        let expected_positions = vec![
            Position { row: 1, col: 3 },
            Position { row: 7, col: 6 },
            Position { row: 2, col: 0 },
            Position { row: 6, col: 2 },
        ];

        // Given an antenna network
        let mut network = AntennaNetwork::new('a', 10, 10);
        network.add_antenna(Position { row: 3, col: 4 });
        network.add_antenna(Position { row: 5, col: 5 });
        network.add_antenna(Position { row: 4, col: 8 });

        // When computing the antinode positions
        let positions = network.compute_antinodes();

        // Then they should correspond to the expected ones
        assert_eq!(
            positions, expected_positions,
            "Expected antinodes: {:?} (actual: {:?})",
            expected_positions, positions
        );
    }

    #[test]
    fn should_compute_antinodes_for_part1_example() {
        // Test setup
        let expected_number = 14;

        // Given the example for part 1
        let networks = vec![
            AntennaNetwork {
                frequency: '0',
                height: 12,
                width: 12,
                antennas: vec![
                    Position { row: 1, col: 8 },
                    Position { row: 2, col: 5 },
                    Position { row: 3, col: 7 },
                    Position { row: 4, col: 4 },
                ],
            },
            AntennaNetwork {
                frequency: 'A',
                height: 12,
                width: 12,
                antennas: vec![
                    Position { row: 5, col: 6 },
                    Position { row: 8, col: 8 },
                    Position { row: 9, col: 9 },
                ],
            },
        ];

        // When computing the total number of antinodes
        let result = compute_number_of_antinodes_for_networks(&networks);

        // Then we should get the expected result
        assert_eq!(
            expected_number, result,
            "Number of antinodes: {} (expected: {}",
            result, expected_number
        );
    }

    #[test]
    fn should_compute_antinodes_with_harmonics() {
        // Test setup
        let expected_positions = [
            Position { row: 0, col: 0 },
            Position { row: 1, col: 3 },
            Position { row: 2, col: 6 },
            Position { row: 3, col: 9 },
            Position { row: 2, col: 1 },
            Position { row: 4, col: 2 },
            Position { row: 6, col: 3 },
            Position { row: 8, col: 4 },
            Position { row: 0, col: 5 },
        ];

        // Given a network
        let network = AntennaNetwork {
            width: 10,
            height: 10,
            frequency: 'T',
            antennas: Vec::from([
                Position { row: 0, col: 0 },
                Position { row: 1, col: 3 },
                Position { row: 2, col: 1 },
            ]),
        };

        // When computing antinodes with harmonics
        let positions = network.compute_antinodes_with_harmonics();

        // Then the positions of the antinodes should be the one expected
        assert_eq!(expected_positions.len(), positions.len());
        for (i, expected_position) in expected_positions.iter().enumerate() {
            let position = positions.get(i).unwrap();
            assert_eq!(
                *position, *expected_position,
                "Expected antinode #{:?}: {:?} (actual: {:?})",
                i, expected_position, position
            );
        }
    }

    #[test]
    fn should_compute_harmonical_antinodes_for_part2_example() {
        // Test setup
        let expected_number = 34;

        // Given the example for part 2
        let networks = vec![
            AntennaNetwork {
                frequency: '0',
                height: 12,
                width: 12,
                antennas: vec![
                    Position { row: 1, col: 8 },
                    Position { row: 2, col: 5 },
                    Position { row: 3, col: 7 },
                    Position { row: 4, col: 4 },
                ],
            },
            AntennaNetwork {
                frequency: 'A',
                height: 12,
                width: 12,
                antennas: vec![
                    Position { row: 5, col: 6 },
                    Position { row: 8, col: 8 },
                    Position { row: 9, col: 9 },
                ],
            },
        ];

        // When computing the total number of antinodes
        let result = compute_number_of_harmonical_antinodes_for_networks(&networks);

        // Then we should get the expected result
        assert_eq!(
            expected_number, result,
            "Number of antinodes: {} (expected: {}",
            result, expected_number
        );
    }
}
