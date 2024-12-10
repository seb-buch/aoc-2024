use crate::topo::Map;
use anyhow::*;
use aoc2024::*;
use std::str::FromStr;

const DAY: &str = "10";
const SOLUTION_PART_1: &str = "629";
const SOLUTION_PART_2: &str = "1242";

mod topo {
    use std::str::FromStr;
    const IMPASSABLE_TILE: i8 = -1;
    const MAX_HEIGHT: i8 = 9;

    #[derive(Debug, PartialEq)]
    pub struct Map {
        pub heights: Vec<Vec<i8>>,
    }

    impl FromStr for Map {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut heights: Vec<Vec<i8>> = Vec::new();

            for line in s.lines() {
                heights.push(
                    line.trim()
                        .chars()
                        .map(|c| c.to_digit(10).unwrap_or(IMPASSABLE_TILE as u32) as i8)
                        .collect(),
                );
            }

            Ok(Map { heights })
        }
    }

    impl Map {
        pub fn get_trailheads(&self) -> Vec<(usize, usize)> {
            let mut trailheads: Vec<(usize, usize)> = Vec::new();

            for row in 0..self.heights.len() {
                for col in 0..self.heights[row].len() {
                    let height = self.heights[row][col];
                    if height == 0 {
                        trailheads.push((row, col));
                    }
                }
            }

            trailheads
        }

        fn find_trail_targets(
            &self,
            current_row: usize,
            current_col: usize,
        ) -> Vec<(usize, usize)> {
            let current_height = self.heights[current_row][current_col];

            if current_height == MAX_HEIGHT {
                return vec![(current_row, current_col)];
            }

            let mut targets: Vec<(usize, usize)> = Vec::new();

            if current_row > 0 {
                // Check north tile
                let tile_height = self.heights[current_row - 1][current_col];
                if tile_height == current_height + 1 {
                    for target in self.find_trail_targets(current_row - 1, current_col) {
                        if !targets.contains(&target) {
                            targets.push(target);
                        }
                    }
                }
            }

            if current_row < self.heights.len() - 1 {
                // Check south tile
                let tile_height = self.heights[current_row + 1][current_col];
                if tile_height == current_height + 1 {
                    for target in self.find_trail_targets(current_row + 1, current_col) {
                        if !targets.contains(&target) {
                            targets.push(target);
                        }
                    }
                }
            }

            if current_col > 0 {
                // Check west tile
                let tile_height = self.heights[current_row][current_col - 1];
                if tile_height == current_height + 1 {
                    for target in self.find_trail_targets(current_row, current_col - 1) {
                        if !targets.contains(&target) {
                            targets.push(target);
                        }
                    }
                }
            }

            if current_col < self.heights[current_row].len() - 1 {
                // Check west tile
                let tile_height = self.heights[current_row][current_col + 1];
                if tile_height == current_height + 1 {
                    for target in self.find_trail_targets(current_row, current_col + 1) {
                        if !targets.contains(&target) {
                            targets.push(target);
                        }
                    }
                }
            }

            targets
        }

        pub fn get_trailhead_score(&self, row: usize, col: usize) -> usize {
            if row >= self.heights.len() || col >= self.heights[row].len() {
                return 0;
            }

            if self.heights[row][col] > 0 {
                return 0;
            }

            self.find_trail_targets(row, col).len()
        }

        fn find_trail_rating(&self, current_row: usize, current_col: usize) -> usize {
            let current_height = self.heights[current_row][current_col];

            if current_height == MAX_HEIGHT {
                return 1;
            }

            let mut rating = 0usize;

            if current_row > 0 {
                // Check north tile
                let tile_row = current_row - 1;
                let tile_col = current_col;

                let tile_height = self.heights[tile_row][tile_col];
                if tile_height == current_height + 1 {
                    rating += self.find_trail_rating(tile_row, tile_col);
                }
            }

            if current_row < self.heights.len() - 1 {
                // Check south tile
                let tile_row = current_row + 1;
                let tile_col = current_col;

                let tile_height = self.heights[tile_row][tile_col];
                if tile_height == current_height + 1 {
                    rating += self.find_trail_rating(tile_row, tile_col);
                }
            }

            if current_col > 0 {
                // Check west tile
                let tile_row = current_row;
                let tile_col = current_col - 1;

                let tile_height = self.heights[tile_row][tile_col];
                if tile_height == current_height + 1 {
                    rating += self.find_trail_rating(tile_row, tile_col);
                }
            }

            if current_col < self.heights[current_row].len() - 1 {
                // Check west tile
                let tile_row = current_row;
                let tile_col = current_col + 1;

                let tile_height = self.heights[tile_row][tile_col];
                if tile_height == current_height + 1 {
                    rating += self.find_trail_rating(tile_row, tile_col);
                }
            }

            rating
        }

        pub fn get_trailhead_rating(&self, row: usize, col: usize) -> usize {
            if row >= self.heights.len() || col >= self.heights[row].len() {
                return 0;
            }

            if self.heights[row][col] > 0 {
                return 0;
            }

            self.find_trail_rating(row, col)
        }
    }
}

fn compute_trailhead_scores_sum(map: &Map) -> usize {
    map.get_trailheads()
        .iter()
        .map(|(r, c)| {
            let score = map.get_trailhead_score(*r, *c);
            score
        })
        .sum()
}

fn compute_trailhead_rating_sum(map: &Map) -> usize {
    let sum: usize = map
        .get_trailheads()
        .iter()
        .map(|(r, c)| map.get_trailhead_rating(*r, *c))
        .sum();

    sum
}

//region Part 1

fn solve_part_1(input_data: &str) -> Result<String> {
    let map = Map::from_str(input_data)?;
    Ok(format!("{}", compute_trailhead_scores_sum(&map)))
}
//endregion

//region Part 2

fn solve_part_2(input_data: &str) -> Result<String> {
    let map = Map::from_str(input_data)?;
    Ok(format!("{}", compute_trailhead_rating_sum(&map)))
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
    use crate::topo::Map;
    use crate::{compute_trailhead_rating_sum, compute_trailhead_scores_sum};
    use std::str::FromStr;

    #[test]
    fn should_build_map() {
        let expected = Map {
            heights: [
                [-1, -1, -1, 0, -1, -1, -1].to_vec(),
                [-1, -1, -1, 1, -1, -1, -1].to_vec(),
                [-1, -1, -1, 2, -1, -1, -1].to_vec(),
                [6, 5, 4, 3, 4, 5, 6].to_vec(),
                [7, -1, -1, -1, -1, -1, 7].to_vec(),
                [8, -1, -1, -1, -1, -1, 8].to_vec(),
                [9, -1, -1, -1, -1, -1, 9].to_vec(),
            ]
            .to_vec(),
        };

        let input_data = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

        let map = Map::from_str(input_data).unwrap();

        assert_eq!(expected, map);
    }

    #[test]
    fn should_get_trailheads() {
        let expected = [(0, 1), (6, 5)];

        let map = Map::from_str(
            "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
        )
        .unwrap();

        assert_eq!(expected, *map.get_trailheads())
    }

    #[test]
    fn should_get_trailhead_score() {
        let expected = 2;

        let map = Map::from_str(
            "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        )
        .unwrap();

        assert_eq!(expected, map.get_trailhead_score(0, 3));
    }

    #[test]
    fn should_get_trailhead_score_multiple() {
        let expected = 4;

        let map = Map::from_str(
            "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        )
        .unwrap();

        assert_eq!(expected, map.get_trailhead_score(0, 3));
    }

    #[test]
    fn should_get_trailhead_scores_one_trailhead() {
        let expected = 2;
        let map = Map::from_str(
            "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        )
        .unwrap();
        assert_eq!(expected, compute_trailhead_scores_sum(&map));
    }

    #[test]
    fn should_get_trailhead_scores_two_trailheads() {
        let expected = 3;
        let map = Map::from_str(
            "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
        )
        .unwrap();
        assert_eq!(expected, compute_trailhead_scores_sum(&map));
    }

    #[test]
    fn should_get_trailhead_scores() {
        let expected = 36;
        let map = Map::from_str(
            "89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732",
        )
        .unwrap();
        assert_eq!(expected, compute_trailhead_scores_sum(&map));
    }

    #[test]
    fn should_get_trailhead_rating_simple() {
        let expected = 3;
        let map = Map::from_str(
            ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....",
        )
        .unwrap();

        assert_eq!(expected, map.get_trailhead_rating(0, 5));
    }

    #[test]
    fn should_get_trailhead_rating_medium() {
        let expected = 13;
        let map = Map::from_str(
            "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        )
        .unwrap();

        assert_eq!(expected, map.get_trailhead_rating(0, 3));
    }

    #[test]
    fn should_get_trailhead_rating_complex() {
        let expected = 227;
        let map = Map::from_str(
            "012345
123456
234567
345678
4.6789
56789.",
        )
        .unwrap();

        assert_eq!(expected, map.get_trailhead_rating(0, 0));
    }

    #[test]
    fn should_get_trailhead_ratings_example() {
        let expected = 81;
        let map = Map::from_str(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        )
        .unwrap();

        assert_eq!(expected, compute_trailhead_rating_sum(&map));
    }
}
