#!/usr/bin/env/python3
import numpy as np

from common import get_input_data, pretty_duration_ns, time_function, check_result

DAY = "01"
SOLUTION_PART_1 = "2344935"
SOLUTION_PART_2 = "27647262"


# region Part 1
def solve_part_1(input_data: str) -> str:
    data = np.array([list(map(int, line.split())) for line in input_data.splitlines()])

    left = np.sort(data[:, 0])
    right = np.sort(data[:, 1])

    result = np.sum(np.abs(left - right))
    return f"{result}"


# enregion


# region Part 2
def solve_part_2(input_data: str) -> str:
    data = np.array([list(map(int, line.split())) for line in input_data.splitlines()])

    left = np.sort(data[:, 0])
    right = np.sort(data[:, 1])

    unique, counts = np.unique(right, return_counts=True)
    occurences = dict(zip(unique.tolist(), counts.tolist()))

    result = sum(num * occurences.setdefault(num, 0) for num in left)

    return f"{result}"


# endregion


# region Main
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
