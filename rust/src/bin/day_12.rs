use crate::garden::Garden;
use anyhow::*;
use aoc2024::*;
use std::str::FromStr;

const DAY: &str = "12";
const SOLUTION_PART_1: &str = "1450816";
const SOLUTION_PART_2: &str = "865662";

mod garden {
    use std::collections::HashMap;
    use std::fmt::Display;
    use std::str::FromStr;

    pub struct Garden {
        height: usize,
        width: usize,
        tiles: Vec<char>,
        tiles_regions: Vec<Option<usize>>,
        regions: HashMap<usize, Vec<(usize, usize)>>,
    }

    impl Garden {
        pub fn new() -> Garden {
            Garden {
                height: 0,
                width: 0,
                tiles: Vec::new(),
                regions: HashMap::new(),
                tiles_regions: Vec::new(),
            }
        }

        fn get_index_from_position(&self, position: &(usize, usize)) -> usize {
            self.width * position.0 + position.1
        }

        fn get_neighbors_from_position(
            &self,
            position: &(usize, usize),
        ) -> [Option<(usize, usize)>; 4] {
            let (row, col) = *position;

            let north_neighbour = match row {
                0 => None,
                _ => Some((row - 1, col)),
            };

            let south_neighbour = match row {
                value if value == self.height - 1 => None,
                _ => Some((row + 1, col)),
            };

            let west_neighbour = match col {
                0 => None,
                _ => Some((row, col - 1)),
            };

            let east_neighbour = match col {
                value if value == self.width - 1 => None,
                _ => Some((row, col + 1)),
            };

            [
                north_neighbour,
                south_neighbour,
                west_neighbour,
                east_neighbour,
            ]
        }

        fn get_neighbour_around_position(
            &self,
            position: &(usize, usize),
        ) -> [Option<(usize, usize)>; 8] {
            let (row, col) = *position;

            let north_neighbour = match row {
                0 => None,
                _ => Some((row - 1, col)),
            };

            let northeast_neighbour = match row {
                0 => None,
                _ => match col {
                    value if value == self.width - 1 => None,
                    _ => Some((row - 1, col + 1)),
                },
            };

            let east_neighbour = match col {
                value if value == self.width - 1 => None,
                _ => Some((row, col + 1)),
            };

            let southeast_neighbour = match row {
                value if value == self.height - 1 => None,
                _ => match col {
                    value if value == self.width - 1 => None,
                    _ => Some((row + 1, col + 1)),
                },
            };

            let south_neighbour = match row {
                value if value == self.height - 1 => None,
                _ => Some((row + 1, col)),
            };

            let southwest_neighbour = match row {
                value if value == self.height - 1 => None,
                _ => match col {
                    0 => None,
                    _ => Some((row + 1, col - 1)),
                },
            };

            let west_neighbour = match col {
                0 => None,
                _ => Some((row, col - 1)),
            };

            let northwest_neighbour = match row {
                0 => None,
                _ => match col {
                    0 => None,
                    _ => Some((row - 1, col - 1)),
                },
            };

            [
                north_neighbour,
                northeast_neighbour,
                east_neighbour,
                southeast_neighbour,
                south_neighbour,
                southwest_neighbour,
                west_neighbour,
                northwest_neighbour,
            ]
        }

        fn populate_region(&mut self, region_id: usize, seed_position: &(usize, usize)) {
            let seed_index = self.get_index_from_position(seed_position);
            let seed_char = self.tiles[seed_index];

            let mut buffer = vec![*seed_position];
            while !buffer.is_empty() {
                let position = buffer.remove(0);
                let index = self.get_index_from_position(&position);
                let char = self.tiles[index];

                if char == seed_char && self.tiles_regions[index].is_none() {
                    self.tiles_regions[index] = Some(region_id);
                    self.regions.entry(region_id).or_default().push(position);

                    for neighbour in self
                        .get_neighbors_from_position(&position)
                        .into_iter()
                        .flatten()
                    {
                        let neighbour_index = self.get_index_from_position(&neighbour);
                        if self.tiles_regions[neighbour_index].is_none() {
                            buffer.push(neighbour);
                        }
                    }
                }
            }
        }

        pub fn identify_regions(&mut self) {
            self.tiles_regions = vec![None; self.tiles.len()];
            self.regions = HashMap::new();

            let height = self.height;
            let width = self.width;

            let mut current_region_id = 0usize;
            for row in 0..height {
                for col in 0..width {
                    let index = self.get_index_from_position(&(row, col));
                    if self.tiles_regions[index].is_none() {
                        self.populate_region(current_region_id, &(row, col));
                        current_region_id += 1;
                    }
                }
            }
        }

        fn compute_price_for_region(&self, region_id: usize) -> usize {
            let mut perimeter = 0usize;
            let member_positions = self.regions.get(&region_id).unwrap();
            let area = member_positions.len();
            let region_char =
                self.tiles[self.get_index_from_position(member_positions.first().unwrap())];

            for position in member_positions {
                let neighbours = self.get_neighbors_from_position(position);

                for neighbour in neighbours {
                    if let Some(neighbour_position) = neighbour {
                        let index = self.get_index_from_position(&neighbour_position);
                        let char = self.tiles[index];
                        if char != region_char {
                            perimeter += 1;
                        }
                    } else {
                        perimeter += 1;
                    }
                }
            }

            perimeter * area
        }

        pub fn compute_fence_price_per_region(&mut self) -> HashMap<usize, usize> {
            if self.regions.is_empty() {
                self.identify_regions();
            }

            self.regions
                .keys()
                .map(|region_id| (*region_id, self.compute_price_for_region(*region_id)))
                .collect()
        }

        fn compute_discounted_price_for_region(&self, region_id: usize) -> usize {
            let mut total_edges = 0usize;

            let member_positions = self.regions.get(&region_id).unwrap();
            let area = member_positions.len();
            let region_char =
                self.tiles[self.get_index_from_position(member_positions.first().unwrap())];

            let is_from_same_region = |position: Option<(usize, usize)>| {
                if let Some(position) = position {
                    let index = self.get_index_from_position(&position);
                    let char = self.tiles[index];
                    if char != region_char {
                        return false;
                    }
                    return true;
                } else {
                    return false;
                }
            };

            for position in member_positions {
                let neighbours = self.get_neighbour_around_position(position);

                let mut n_edges = 0usize;
                for edge in 0..4 {
                    let edge_neighbour_1 = neighbours[edge * 2];
                    let edge_neighbour_2 = neighbours[edge * 2 + 1];
                    let edge_neighbour_3 = neighbours[(edge * 2 + 2) % 8];

                    let mut n_consecutive_foreign_neighbours = 0usize;
                    if !is_from_same_region(edge_neighbour_1) {
                        n_consecutive_foreign_neighbours += 1;
                    }
                    if !is_from_same_region(edge_neighbour_2) {
                        n_consecutive_foreign_neighbours += 1;
                    } else {
                        n_consecutive_foreign_neighbours = 0;
                    }
                    if !is_from_same_region(edge_neighbour_3) {
                        n_consecutive_foreign_neighbours += 1;
                    } else {
                        n_consecutive_foreign_neighbours = 0;
                    }

                    if n_consecutive_foreign_neighbours % 2 == 1 {
                        n_edges += 1;
                    }
                }

                total_edges += n_edges;
            }

            area * total_edges
        }

        pub fn compute_discounted_price_per_region(&mut self) -> HashMap<usize, usize> {
            if self.regions.is_empty() {
                self.identify_regions();
            }

            self.regions
                .keys()
                .map(|region_id| {
                    (
                        *region_id,
                        self.compute_discounted_price_for_region(*region_id),
                    )
                })
                .collect()
        }
    }

    impl FromStr for Garden {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut garden = Garden::new();

            for line in s.lines() {
                garden.height += 1;
                for ch in line.trim().chars() {
                    if garden.height == 1 {
                        garden.width += 1;
                    }
                    garden.tiles.push(ch);
                }
            }

            Ok(garden)
        }
    }

    impl Display for Garden {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
            let mut output = String::new();

            for row in 0..self.height {
                for col in 0..self.width {
                    output.push(self.tiles[row * self.width + col]);
                }
                if row != self.height - 1 {
                    output.push('\n');
                }
            }
            write!(f, "{}", output)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        const SMALL_EXAMPLE: &str = "AAAA
BBCD
BBCC
EEEC";
        const BIGGER_EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        #[test]
        fn should_load_from_str() {
            let expected_width = 4usize;
            let expected_height = 4usize;
            let expected_tiles: Vec<char> = "AAAABBCDBBCCEEEC".chars().collect();

            let garden = Garden::from_str(SMALL_EXAMPLE).unwrap();

            assert_eq!(expected_width, garden.width);
            assert_eq!(expected_height, garden.height);
            assert_eq!(expected_tiles, garden.tiles);
            assert_eq!(SMALL_EXAMPLE, format!("{}", garden));
        }

        #[test]
        fn should_identify_regions() {
            let expected_regions: HashMap<usize, Vec<(usize, usize)>> = HashMap::from([
                (0usize, vec![(0usize, 0usize), (0, 1), (0, 2), (0, 3)]),
                (1, vec![(1, 0), (1, 1), (2, 0), (2, 1)]),
                (2, vec![(1, 2), (2, 2), (2, 3), (3, 3)]),
                (3, vec![(1, 3)]),
                (4, vec![(3, 0), (3, 1), (3, 2)]),
            ]);

            let mut garden = Garden::from_str(SMALL_EXAMPLE).unwrap();
            garden.identify_regions();

            for (key, expected_value) in expected_regions.iter() {
                let region = &garden.regions[key];
                assert_eq!(
                    expected_value.len(),
                    region.len(),
                    "region #{} has size {} (expected: {})",
                    key,
                    region.len(),
                    expected_value.len()
                );

                for value in expected_value.iter() {
                    assert!(
                        region.contains(value),
                        "region #{:?} does not contain {:?}",
                        key,
                        value
                    );
                }
            }
        }

        #[test]
        fn should_compute_fence_price_per_region() {
            let expected_prices: HashMap<usize, usize> =
                HashMap::from([(0, 40), (1, 32), (2, 40), (3, 4), (4, 24)]);

            let mut garden = Garden::from_str(SMALL_EXAMPLE).unwrap();
            garden.identify_regions();

            let prices = garden.compute_fence_price_per_region();

            for (key, expected_value) in expected_prices.iter() {
                let price = prices[key];

                assert_eq!(
                    expected_value, &price,
                    "price #{} does not match expected price {} (actual: {})",
                    key, expected_value, price
                );
            }
        }

        #[test]
        fn should_compute_fence_price_bigger_example() {
            let expected_sum = 1930;

            let prices = Garden::from_str(BIGGER_EXAMPLE)
                .unwrap()
                .compute_fence_price_per_region();
            let sum = prices.values().sum::<usize>();
            assert_eq!(expected_sum, sum);
        }

        #[test]
        fn should_compute_discounted_price_for_small_example() {
            let expected_sum = 80usize;
            let prices = Garden::from_str(SMALL_EXAMPLE)
                .unwrap()
                .compute_discounted_price_per_region();
            let sum = prices.values().sum::<usize>();
            assert_eq!(expected_sum, sum);
        }

        #[test]
        fn should_compute_discounted_price_for_bigger_example() {
            let expected_sum = 1206usize;
            let prices = Garden::from_str(BIGGER_EXAMPLE)
                .unwrap()
                .compute_discounted_price_per_region();
            let sum = prices.values().sum::<usize>();
            assert_eq!(expected_sum, sum);
        }
    }
}
//region Part 1

fn solve_part_1(input_data: &str) -> Result<String> {
    let mut garden = Garden::from_str(input_data)?;
    let prices = garden.compute_fence_price_per_region();
    Ok(format!("{}", prices.values().sum::<usize>()))
}
//endregion

//region Part 2

fn solve_part_2(input_data: &str) -> Result<String> {
    let mut garden = Garden::from_str(input_data)?;
    let prices = garden.compute_discounted_price_per_region();
    Ok(format!("{}", prices.values().sum::<usize>()))
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
