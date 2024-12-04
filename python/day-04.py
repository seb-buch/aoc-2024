#!/usr/bin/env/python3
import numpy as np
from numpy.typing import NDArray

from common import get_input_data, pretty_duration_ns, time_function, check_result

DAY = "04"
SOLUTION_PART_1 = "2536"
SOLUTION_PART_2 = "1875"


def input_to_2darray(input_data: str) -> NDArray[[str, str]]:
    converted = []
    for line in input_data.splitlines():
        converted.append([char for char in line])

    return np.array(converted)


def check_xmas_direction_east(
    position: tuple[int, int], grid: NDArray[[str, str]]
) -> bool:
    start_row, start_col = position

    if start_col > grid.shape[1] - 4:
        return False

    if grid[start_row, start_col + 1] != "M":
        return False

    if grid[start_row, start_col + 2] != "A":
        return False

    if grid[start_row, start_col + 3] != "S":
        return False

    return True


def check_xmas_direction_north(
    position: tuple[int, int], grid: NDArray[[str, str]]
) -> bool:
    start_row, start_col = position

    if start_row < 3:
        return False

    if grid[start_row - 1, start_col] != "M":
        return False

    if grid[start_row - 2, start_col] != "A":
        return False

    if grid[start_row - 3, start_col] != "S":
        return False

    return True


def check_xmas_direction_southeast(
    position: tuple[int, int], grid: NDArray[[str, str]]
) -> bool:
    start_row, start_col = position

    if start_row > grid.shape[0] - 4 or start_col > grid.shape[1] - 4:
        return False

    if grid[start_row + 1, start_col + 1] != "M":
        return False

    if grid[start_row + 2, start_col + 2] != "A":
        return False

    if grid[start_row + 3, start_col + 3] != "S":
        return False

    return True


def check_xmas_direction_southwest(
    position: tuple[int, int], grid: NDArray[[str, str]]
) -> bool:
    start_row, start_col = position

    if start_row > grid.shape[0] - 4 or start_col < 3:
        return False

    if grid[start_row + 1, start_col - 1] != "M":
        return False

    if grid[start_row + 2, start_col - 2] != "A":
        return False

    if grid[start_row + 3, start_col - 3] != "S":
        return False

    return True


def check_xmas_direction_west(
    position: tuple[int, int], grid: NDArray[[str, str]]
) -> bool:
    start_row, start_col = position

    if start_col < 3:
        return False

    if grid[start_row, start_col - 1] != "M":
        return False

    if grid[start_row, start_col - 2] != "A":
        return False

    if grid[start_row, start_col - 3] != "S":
        return False

    return True


def check_xmas_direction_northeast(
    position: tuple[int, int], grid: NDArray[[str, str]]
) -> bool:
    start_row, start_col = position

    if start_row < 3 or start_col > grid.shape[1] - 4:
        return False

    if grid[start_row - 1, start_col + 1] != "M":
        return False

    if grid[start_row - 2, start_col + 2] != "A":
        return False

    if grid[start_row - 3, start_col + 3] != "S":
        return False

    return True


def check_xmas_direction_northwest(
    position: tuple[int, int], grid: NDArray[[str, str]]
) -> bool:
    start_row, start_col = position

    if start_row < 3 or start_col < 3:
        return False

    if grid[start_row - 1, start_col - 1] != "M":
        return False

    if grid[start_row - 2, start_col - 2] != "A":
        return False

    if grid[start_row - 3, start_col - 3] != "S":
        return False

    return True


def check_xmas_direction_south(
    position: tuple[int, int], grid: NDArray[[str, str]]
) -> bool:
    start_row, start_col = position

    if start_row > grid.shape[0] - 4:
        return False

    if grid[start_row + 1, start_col] != "M":
        return False

    if grid[start_row + 2, start_col] != "A":
        return False

    if grid[start_row + 3, start_col] != "S":
        return False

    return True


def n_xmas_starting_from_position(
    position: tuple[int, int], grid: NDArray[[str, str]]
) -> int:
    n_found = 0
    for func in [
        check_xmas_direction_east,
        check_xmas_direction_west,
        check_xmas_direction_north,
        check_xmas_direction_south,
        check_xmas_direction_southeast,
        check_xmas_direction_southwest,
        check_xmas_direction_northeast,
        check_xmas_direction_northwest,
    ]:
        if func(position=position, grid=grid):
            n_found += 1

    return n_found


# region Part 1
def solve_part_1(input_data: str) -> str:
    grid = input_to_2darray(input_data=input_data)

    n_xmas_found = 0
    for xi in range(grid.shape[0]):
        for yi in range(grid.shape[1]):
            if grid[xi, yi] == "X":
                n_xmas_found += n_xmas_starting_from_position(
                    position=(xi, yi), grid=grid
                )
    return f"{n_xmas_found}"


# endregion


# region Part 2
def solve_part_2(input_data: str) -> str:
    grid = input_to_2darray(input_data=input_data)

    n_found = 0
    for row_i in range(1, grid.shape[0] - 1):
        for col_i in range(1, grid.shape[1] - 1):
            if grid[row_i, col_i] == "A":
                if check_cross(position=(row_i, col_i), grid=grid):
                    n_found += 1

    return f"{n_found}"


def check_cross(position: tuple[int, int], grid: NDArray[[str, str]]) -> bool:
    start_row, start_col = position

    if grid[start_row - 1, start_col - 1] not in ["M", "S"]:
        return False

    if grid[start_row + 1, start_col + 1] not in ["M", "S"]:
        return False

    if grid[start_row - 1, start_col + 1] not in ["M", "S"]:
        return False

    if grid[start_row + 1, start_col - 1] not in ["M", "S"]:
        return False

    if (
        grid[start_row - 1, start_col - 1] == "M"
        and grid[start_row + 1, start_col + 1] != "S"
    ):
        return False

    if (
        grid[start_row - 1, start_col - 1] == "S"
        and grid[start_row + 1, start_col + 1] != "M"
    ):
        return False

    if (
        grid[start_row - 1, start_col + 1] == "M"
        and grid[start_row + 1, start_col - 1] != "S"
    ):
        return False

    if (
        grid[start_row - 1, start_col + 1] == "S"
        and grid[start_row + 1, start_col - 1] != "M"
    ):
        return False

    return True


# endregion


# region Main function
def main():
    input_data, io_duration = time_function(get_input_data, day=DAY)
    print(f"Input data loaded in {pretty_duration_ns(io_duration)}")

    answer_part_1, part1_duration = time_function(solve_part_1, input_data=input_data)
    print(f"Part 1: {answer_part_1} (solved in {pretty_duration_ns(part1_duration)})")
    check_result(answer_part_1, SOLUTION_PART_1)

    answer_part_2, part2_duration = time_function(solve_part_2, input_data=input_data)
    print(f"Part 2: {answer_part_2} (solved in {pretty_duration_ns(part2_duration)})")
    check_result(answer_part_2, SOLUTION_PART_2)


# endregion

if __name__ == "__main__":
    main()
