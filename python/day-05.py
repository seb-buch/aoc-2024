#!/usr/bin/env/python3
from collections import defaultdict
from typing import DefaultDict

from common import get_input_data, pretty_duration_ns, time_function, check_result

DAY = "05"
SOLUTION_PART_1 = "6041"
SOLUTION_PART_2 = "4884"


def input_to_order_and_updates(
    input_data: str,
) -> tuple[DefaultDict[int, list[int]], list[list[int]]]:
    in_update_section = False

    orders: DefaultDict[int, list[int]] = defaultdict(list)
    updates = []
    for line in input_data.splitlines():
        if not in_update_section:
            order = line.split("|")
            if len(order) != 2:
                in_update_section = True
                continue

            left, right = [int(num) for num in order]

            orders[left].append(right)

        else:
            updates.append([int(num) for num in line.split(",")])

    return orders, updates


def is_order_correct(update: list[int], orders: DefaultDict[int, list[int]]) -> bool:
    preceding_nums = []
    for num in update:
        forbidden_nums = orders[num]
        for preceding_num in preceding_nums:
            if preceding_num in forbidden_nums:
                return False
        preceding_nums.append(num)
    return True


# region Part 1
def solve_part_1(input_data: str) -> str:
    orders, updates = input_to_order_and_updates(input_data=input_data)

    total = 0
    for update in updates:
        if is_order_correct(update=update, orders=orders):
            total += update[len(update) // 2]

    return f"{total}"


# endregion


def reorder_update(update: list[int], orders: DefaultDict[int, list[int]]) -> list[int]:
    reordered_update = update[:]
    while not is_order_correct(update=reordered_update, orders=orders):
        preceding_nums = []
        for num in reordered_update:
            forbidden_nums = orders[num]
            reordered = False
            for preceding_num in preceding_nums:
                if preceding_num in forbidden_nums:
                    i = reordered_update.index(num)
                    j = reordered_update.index(preceding_num)
                    reordered_update[i], reordered_update[j] = (
                        reordered_update[j],
                        reordered_update[i],
                    )
                    reordered = True

                    break

            if reordered:
                break
            preceding_nums.append(num)

    return reordered_update


# region Part 2
def solve_part_2(input_data: str) -> str:
    orders, updates = input_to_order_and_updates(input_data=input_data)

    total = 0
    for update in updates:
        if not is_order_correct(update=update, orders=orders):
            reordered_update = reorder_update(update=update, orders=orders)
            total += reordered_update[len(reordered_update) // 2]

    return f"{total}"


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
