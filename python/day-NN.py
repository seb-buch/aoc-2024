#!/usr/bin/env/python3

from common import get_input_data, pretty_duration_ns, time_function, check_result

DAY = "NN"  # TODO: Replace with actual day number
SOLUTION_PART_1 = "TODO"  # TODO: Replace with actual value
SOLUTION_PART_2 = "TODO"  # TODO: Replace with actual value


# region Part 1
def solve_part_1(input_data: str) -> str:
    return ""


# endregion


# region Part 2
def solve_part_2(input_data: str) -> str:
    return ""


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
