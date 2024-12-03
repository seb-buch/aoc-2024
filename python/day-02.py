#!/usr/bin/env/python3
from itertools import pairwise

from common import get_input_data, pretty_duration_ns, time_function, check_result

DAY = "02"
SOLUTION_PART_1 = "213"
SOLUTION_PART_2 = "285"


def is_report_safe(report: list[int]) -> bool:
    ASCENDING_INCREMENTS = {1, 2, 3}
    DESCENDING_INCREMENTS = {-1, -2, -3}

    increment_set = ASCENDING_INCREMENTS
    if report[0] > report[-1]:
        increment_set = DESCENDING_INCREMENTS

    for a, b in pairwise(report):
        increment = b - a
        if increment not in increment_set:
            return False
    return True


def is_fixable_report(report: list[int]) -> bool:
    for i in range(len(report)):
        truncated_report = report[:i] + report[i + 1 :]
        if is_report_safe(truncated_report):
            return True
    return False


# region Part 1
def solve_part_1(input_data: str) -> str:
    n_safe = 0
    for report in input_data.splitlines():
        if is_report_safe([int(n) for n in report.split()]):
            n_safe += 1
    return f"{n_safe}"


# endregion


# region Part 2
def solve_part_2(input_data: str) -> str:
    n_safe = 0
    for report in input_data.splitlines():
        numbers = [int(n) for n in report.split()]

        if is_report_safe(numbers):
            n_safe += 1

        elif is_fixable_report(numbers):
            n_safe += 1

    return f"{n_safe}"


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
