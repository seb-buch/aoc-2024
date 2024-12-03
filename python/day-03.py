#!/usr/bin/env/python3

from common import get_input_data, pretty_duration_ns, time_function, check_result

DAY = "03"
SOLUTION_PART_1 = "182780583"
SOLUTION_PART_2 = "90772405"


def parse_multiplication(input_data: str, start: int) -> tuple[int, int] | None:
    # Parsing left number
    comma_position = input_data.find(",", start)

    if comma_position == -1:
        return None

    left = input_data[start + 4 : comma_position]

    # Parsing right number
    end = input_data.find(")", comma_position)
    if end == -1:
        return None

    right = input_data[comma_position + 1 : end]

    # Parsing result
    try:
        result = int(left) * int(right)
    except ValueError:
        return None

    return result, end - start


def parse_multiplications(input_data: str) -> list[int]:
    multiplications = []
    start = 0
    while start != -1:
        position = input_data.find("mul(", start)
        if position == -1:
            break

        # parse multiplication
        multiplication = parse_multiplication(input_data, position)
        offset = 4
        if multiplication is not None:
            result, offset = multiplication
            multiplications.append(result)

        start = position + offset

    return multiplications


# region Part 1
def solve_part_1(input_data: str) -> str:
    return f"{sum(parse_multiplications(input_data))}"


# endregion


# region Part 2
def solve_part_2(input_data: str) -> str:
    # identify region where the multiplication is enabled
    start = 0
    is_enabled = True
    multiplications = []
    while True:
        if is_enabled:
            anchor = "don't()"
        else:
            anchor = "do()"

        next_instruction_position = input_data.find(anchor, start)

        if is_enabled:
            multiplications.extend(
                parse_multiplications(input_data[start:next_instruction_position])
            )

        if next_instruction_position == -1:
            break

        is_enabled = not is_enabled
        start = next_instruction_position + len(anchor)

    return f"{sum(multiplications)}"


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
