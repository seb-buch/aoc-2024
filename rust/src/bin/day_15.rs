use crate::warehouse::Warehouse;
use crate::widewarehouse::WideWarehouse;
use anyhow::*;
use aoc2024::*;
use std::str::FromStr;

const DAY: &str = "15";
const SOLUTION_PART_1: &str = "1463512";
const SOLUTION_PART_2: &str = "1486520";

mod warehouse {
    use crate::warehouse::MoveError::{BlockedBywall, ImpossibleToMoveCrate, NoMoreMoves};
    use std::collections::VecDeque;
    use std::fmt::Display;
    use std::str::FromStr;

    pub struct Warehouse {
        height: usize,
        width: usize,
        tiles: Vec<char>,
        moves: VecDeque<char>,
        robot_position: (usize, usize),
    }

    enum MoveError {
        NoMoreMoves,
        BlockedBywall,
        ImpossibleToMoveCrate,
    }

    fn compute_new_position(position: &(usize, usize), increment: &(i32, i32)) -> (usize, usize) {
        let new_row = (position.0 as i32 + increment.0) as usize;
        let new_col = (position.1 as i32 + increment.1) as usize;
        (new_row, new_col)
    }

    impl Warehouse {
        pub fn new() -> Warehouse {
            Warehouse {
                height: 0,
                width: 0,
                tiles: Vec::new(),
                moves: VecDeque::new(),
                robot_position: (0, 0),
            }
        }

        fn position_to_index(&self, position: &(usize, usize)) -> usize {
            position.0 * self.width + position.1
        }

        fn get_tile_from_position(&self, position: &(usize, usize)) -> char {
            let index = self.position_to_index(position);

            self.tiles[index]
        }

        fn push_crate(
            &mut self,
            position: &(usize, usize),
            increment: &(i32, i32),
        ) -> Result<(), MoveError> {
            let next_position = &compute_new_position(position, &increment);

            match self.get_tile_from_position(next_position) {
                '.' => self.unsafe_move(position, next_position),
                'O' => match self.push_crate(next_position, increment) {
                    Ok(_) => self.unsafe_move(position, next_position),
                    Err(_) => Err(ImpossibleToMoveCrate),
                },
                '#' => Err(BlockedBywall),
                value => unreachable!("Invalid tile : {}", value),
            }
        }

        fn unsafe_move(
            &mut self,
            old_position: &(usize, usize),
            new_position: &(usize, usize),
        ) -> Result<(), MoveError> {
            let char = self.get_tile_from_position(old_position);

            let new_index = self.position_to_index(new_position);
            self.tiles[new_index] = char;

            let old_index = self.position_to_index(old_position);
            self.tiles[old_index] = '.';

            Ok(())
        }

        fn unsafe_robot_move(
            &mut self,
            old_position: &(usize, usize),
            new_position: &(usize, usize),
        ) -> Result<(), MoveError> {
            self.robot_position = *new_position;
            self.unsafe_move(old_position, new_position)
        }

        fn move_robot_once(&mut self) -> Result<(), MoveError> {
            let increment = match self.moves.pop_front() {
                Some('^') => (-1, 0),
                Some('>') => (0, 1),
                Some('v') => (1, 0),
                Some('<') => (0, -1),
                Some(_) => unreachable!("Invalid move"),
                None => return Err(NoMoreMoves),
            };

            let current_position = &self.robot_position.clone();
            let next_position = &compute_new_position(current_position, &increment);

            match self.get_tile_from_position(next_position) {
                '#' => Err(BlockedBywall),
                '.' => self.unsafe_robot_move(current_position, next_position),
                'O' => match self.push_crate(next_position, &increment) {
                    Err(_) => Err(ImpossibleToMoveCrate),
                    Ok(()) => self.unsafe_robot_move(current_position, next_position),
                },
                value => unreachable!("Invalid tile : {}", value),
            }
        }

        pub fn move_robot_until_done(&mut self) {
            loop {
                match self.move_robot_once() {
                    Ok(_) => {}
                    Err(value) => {
                        if let NoMoreMoves = value {
                            break;
                        }
                    }
                }
            }
        }

        pub fn calculate_gps_coordinates(&mut self) -> Vec<usize> {
            let mut gps_coords: Vec<usize> = Vec::new();

            for row in 0..self.height {
                for col in 0..self.width {
                    let char = self.get_tile_from_position(&(row, col));
                    if char == 'O' {
                        gps_coords.push(100 * row + col);
                    }
                }
            }
            gps_coords
        }
    }

    impl Display for Warehouse {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut output = String::new();

            for row in 0..self.height {
                for col in 0..self.width {
                    output.push(self.tiles[row * self.width + col]);
                }
                output.push('\n');
            }
            write!(f, "{}", output)
        }
    }

    impl FromStr for Warehouse {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut warehouse = Warehouse::new();

            let lines = s.lines();

            let mut in_map_section = true;
            for (row, line) in lines.enumerate() {
                if in_map_section {
                    if line.trim().is_empty() {
                        in_map_section = false;
                        continue;
                    }
                    warehouse.height += 1;

                    for (col, char) in line.trim().chars().enumerate() {
                        if row == 0 {
                            warehouse.width += 1;
                        }
                        warehouse.tiles.push(char);
                        if char == '@' {
                            warehouse.robot_position = (row, col);
                        }
                    }
                } else {
                    for char in line.trim().chars() {
                        warehouse.moves.push_back(char);
                    }
                }
            }

            Ok(warehouse)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::warehouse::Warehouse;
        use std::collections::VecDeque;
        use std::str::FromStr;
        const EXAMPLE_DATA: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
        const SIMPLE_EXAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        #[test]
        fn should_load_from_str() {
            let expected_height = 8;
            let expected_width = 8;
            let expected_robot_position = (2, 2);
            let expected_map = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
";
            let expected_moves = "<^^>>>vv<v>>v<<".chars().collect::<VecDeque<_>>();

            let warehouse = Warehouse::from_str(SIMPLE_EXAMPLE).unwrap();

            assert_eq!(expected_height, warehouse.height);
            assert_eq!(expected_width, warehouse.width);
            assert_eq!(expected_map, format!("{}", warehouse));
            assert_eq!(expected_robot_position, warehouse.robot_position);
            assert_eq!(expected_moves, warehouse.moves);
        }

        #[test]
        fn should_move_robot() {
            let expected_output = "########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
";

            let mut warehouse = Warehouse::from_str(SIMPLE_EXAMPLE).unwrap();

            warehouse.move_robot_until_done();

            assert_eq!(expected_output, format!("{}", warehouse));
        }

        #[test]
        fn should_move_robot_bigger_example() {
            let expected_output = "##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
";

            let mut warehouse = Warehouse::from_str(EXAMPLE_DATA).unwrap();
            warehouse.move_robot_until_done();
            assert_eq!(expected_output, format!("{}", warehouse));
        }

        #[test]
        fn should_calculate_gps_coordinates() {
            let expected_sum = 10092usize;

            let mut warehouse = Warehouse::from_str(EXAMPLE_DATA).unwrap();
            warehouse.move_robot_until_done();

            assert_eq!(
                expected_sum,
                warehouse.calculate_gps_coordinates().iter().sum()
            );
        }
    }
}

mod widewarehouse {
    use crate::widewarehouse::MoveDirection::{Down, Left, Right, Up};
    use crate::widewarehouse::MoveError::{BlockedBywall, ImpossibleToMoveCrate};
    use std::collections::VecDeque;
    use std::fmt::Display;
    use std::str::FromStr;

    pub struct WideWarehouse {
        height: usize,
        width: usize,
        tiles: Vec<char>,
        moves: VecDeque<char>,
        robot_position: (usize, usize),
    }

    #[derive(Debug)]
    enum MoveError {
        BlockedBywall,
        ImpossibleToMoveCrate,
    }

    #[derive(Clone, Copy)]
    enum MoveDirection {
        Up,
        Down,
        Left,
        Right,
    }

    impl WideWarehouse {
        pub fn new() -> WideWarehouse {
            WideWarehouse {
                height: 0,
                width: 0,
                tiles: Vec::new(),
                moves: VecDeque::new(),
                robot_position: (0, 0),
            }
        }

        fn position_to_index(&self, position: &(usize, usize)) -> usize {
            position.0 * self.width + position.1
        }

        fn get_tile_from_position(&self, position: &(usize, usize)) -> char {
            let index = self.position_to_index(position);

            self.tiles[index]
        }

        fn unsafe_move_robot(&mut self, new_position: (usize, usize)) -> Result<(), MoveError> {
            let old_position = &self.robot_position;
            let old_index = self.position_to_index(old_position);
            self.tiles[old_index] = '.';

            let new_index = self.position_to_index(&new_position);
            self.tiles[new_index] = '@';
            self.robot_position = new_position;

            Ok(())
        }

        fn unsafe_move_crate(&mut self, old_position: &(usize, usize), direction: MoveDirection) {
            let old_positions = [
                (old_position.0, old_position.1),
                (old_position.0, old_position.1 + 1),
            ];

            let new_positions = match direction {
                Up => vec![
                    (old_position.0 - 1, old_position.1),
                    (old_position.0 - 1, old_position.1 + 1),
                ],
                Down => vec![
                    (old_position.0 + 1, old_position.1),
                    (old_position.0 + 1, old_position.1 + 1),
                ],
                Left => vec![
                    (old_position.0, old_position.1 - 1),
                    (old_position.0, old_position.1),
                ],
                Right => vec![
                    (old_position.0, old_position.1 + 1),
                    (old_position.0, old_position.1 + 2),
                ],
            };

            for (i, old_position) in old_positions.iter().enumerate() {
                let index = self.position_to_index(old_position);
                let mut expected_old_char = '[';
                if i == 1 {
                    expected_old_char = ']';
                }

                let actual_char = self.get_tile_from_position(old_position);

                if expected_old_char == actual_char {
                    self.tiles[index] = '.';
                }
            }

            for (i, new_position) in new_positions.into_iter().enumerate() {
                let index = self.position_to_index(&new_position);
                if i == 0 {
                    self.tiles[index] = '[';
                } else {
                    self.tiles[index] = ']';
                }
            }
        }

        fn get_crate_neighbours_on_move(
            &self,
            position: &(usize, usize),
            direction: MoveDirection,
        ) -> Vec<(usize, usize)> {
            let actual_position = match self.get_tile_from_position(position) {
                '[' => position,
                ']' => &(position.0, position.1 - 1),
                _ => {
                    return Vec::new();
                }
            };

            match direction {
                Up => vec![
                    (actual_position.0 - 1, actual_position.1),
                    (actual_position.0 - 1, actual_position.1 + 1),
                ],
                Down => vec![
                    (actual_position.0 + 1, actual_position.1),
                    (actual_position.0 + 1, actual_position.1 + 1),
                ],
                Left => vec![(actual_position.0, actual_position.1 - 1)],
                Right => vec![(actual_position.0, actual_position.1 + 2)],
            }
        }

        fn get_crates_to_be_moved(
            &self,
            starting_position: &(usize, usize),
            direction: MoveDirection,
        ) -> Vec<(usize, usize)> {
            let mut crates = Vec::new();

            let actual_position = match self.get_tile_from_position(starting_position) {
                '[' => starting_position,
                ']' => &(starting_position.0, starting_position.1 - 1),
                _ => {
                    return Vec::new();
                }
            };

            crates.push(*actual_position);

            let neighbours = self.get_crate_neighbours_on_move(&actual_position, direction);

            for neighbour_position in neighbours {
                crates.append(&mut self.get_crates_to_be_moved(&neighbour_position, direction));
            }

            crates
        }

        fn can_crates_be_moved(
            &self,
            crates: &Vec<(usize, usize)>,
            direction: MoveDirection,
        ) -> bool {
            for position in crates {
                let neighbours = self.get_crate_neighbours_on_move(position, direction);
                for neighbour_position in neighbours {
                    let tile = self.get_tile_from_position(&neighbour_position);
                    if tile == '#' {
                        return false;
                    }
                }
            }
            true
        }

        fn move_robot_once(&mut self, direction: MoveDirection) -> Result<(), MoveError> {
            let current_position = self.robot_position;
            let next_position = match direction {
                Up => (current_position.0 - 1, current_position.1),
                Down => (current_position.0 + 1, current_position.1),
                Left => (current_position.0, current_position.1 - 1),
                Right => (current_position.0, current_position.1 + 1),
            };

            let neighbour = self.get_tile_from_position(&next_position);

            match neighbour {
                '.' => self.unsafe_move_robot(next_position),
                '[' | ']' => {
                    let mut crates = self.get_crates_to_be_moved(&next_position, direction);
                    crates.reverse();
                    if self.can_crates_be_moved(&crates, direction) {
                        for crate_pos in crates {
                            self.unsafe_move_crate(&crate_pos, direction);
                        }
                        self.unsafe_move_robot(next_position)
                    } else {
                        Err(ImpossibleToMoveCrate)
                    }
                }
                '#' => Err(BlockedBywall),
                _ => unreachable!("Invalid neighbour: {:?}", neighbour),
            }
        }

        pub fn move_robot_until_done(&mut self) {
            loop {
                let robot_move_opt = self.moves.pop_front();

                if robot_move_opt.is_none() {
                    break;
                }

                let robot_move = robot_move_opt.unwrap();

                if (match robot_move {
                    '^' => self.move_robot_once(Up),
                    '>' => self.move_robot_once(Right),
                    'v' => self.move_robot_once(Down),
                    '<' => self.move_robot_once(Left),
                    _ => unreachable!("Unknown move: {}", robot_move),
                })
                .is_err()
                {}
            }
        }

        pub fn calculate_gps_coordinates(&mut self) -> Vec<usize> {
            let mut gps_coords: Vec<usize> = Vec::new();

            for row in 0..self.height {
                for col in 0..self.width {
                    let char = self.get_tile_from_position(&(row, col));
                    if char == '[' {
                        gps_coords.push(100 * row + col);
                    }
                }
            }
            gps_coords
        }
    }

    impl FromStr for WideWarehouse {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut warehouse = WideWarehouse::new();

            let lines = s.lines();

            let mut in_map_section = true;
            for (row, line) in lines.enumerate() {
                if in_map_section {
                    if line.trim().is_empty() {
                        in_map_section = false;
                        continue;
                    }
                    warehouse.height += 1;

                    for (col, char) in line.trim().chars().enumerate() {
                        if row == 0 {
                            warehouse.width += 2;
                        }

                        match char {
                            '.' => {
                                warehouse.tiles.append(&mut vec!['.', '.']);
                            }
                            '#' => {
                                warehouse.tiles.append(&mut vec!['#', '#']);
                            }
                            'O' => {
                                warehouse.tiles.append(&mut vec!['[', ']']);
                            }
                            '@' => {
                                warehouse.tiles.append(&mut vec!['@', '.']);
                                warehouse.robot_position = (row, col * 2usize);
                            }
                            _ => unreachable!("unexpected char in row {}: {}", row, char),
                        }
                    }
                } else {
                    for char in line.trim().chars() {
                        warehouse.moves.push_back(char);
                    }
                }
            }

            Ok(warehouse)
        }
    }

    impl Display for WideWarehouse {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut output = String::new();

            for row in 0..self.height {
                for col in 0..self.width {
                    output.push(self.tiles[row * self.width + col]);
                }
                output.push('\n');
            }
            write!(f, "{}", output)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        const EXAMPLE_DATA: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
        const SIMPLE_EXAMPLE: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

        #[test]
        fn should_load_from_str() {
            let expected_height = 7;
            let expected_width = 14;
            let expected_robot_position = (3, 10);
            let expected_map = "##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############
";
            let expected_moves = "<vv<<^^<<^^".chars().collect::<VecDeque<_>>();

            let warehouse = WideWarehouse::from_str(SIMPLE_EXAMPLE).unwrap();

            assert_eq!(expected_height, warehouse.height);
            assert_eq!(expected_width, warehouse.width);
            assert_eq!(expected_map, format!("{}", warehouse));
            assert_eq!(expected_robot_position, warehouse.robot_position);
            assert_eq!(expected_moves, warehouse.moves);
        }

        #[test]
        fn should_move_robot_until_done() {
            let expected_output = "##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############
";
            let mut warehouse = WideWarehouse::from_str(SIMPLE_EXAMPLE).unwrap();
            warehouse.move_robot_until_done();
            assert_eq!(expected_output, format!("{}", warehouse));
        }

        #[test]
        fn sould_move_bigger_example() {
            let expected_output = "####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################
";

            let mut warehouse = WideWarehouse::from_str(EXAMPLE_DATA).unwrap();
            warehouse.move_robot_until_done();
            assert_eq!(expected_output, format!("{}", warehouse));
        }

        #[test]
        fn should_calculate_gps_coordinates() {
            let expected_sum = 9021usize;

            let mut warehouse = WideWarehouse::from_str(EXAMPLE_DATA).unwrap();
            warehouse.move_robot_until_done();

            assert_eq!(
                expected_sum,
                warehouse.calculate_gps_coordinates().iter().sum()
            );
        }
    }
}

//region Part 1

fn solve_part_1(input_data: &str) -> Result<String> {
    let mut warehouse = Warehouse::from_str(input_data)?;
    warehouse.move_robot_until_done();

    Ok(format!(
        "{}",
        warehouse.calculate_gps_coordinates().iter().sum::<usize>()
    ))
}
//endregion

//region Part 2

fn solve_part_2(input_data: &str) -> Result<String> {
    let mut warehouse = WideWarehouse::from_str(input_data)?;
    warehouse.move_robot_until_done();

    Ok(format!(
        "{}",
        warehouse.calculate_gps_coordinates().iter().sum::<usize>()
    ))
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
