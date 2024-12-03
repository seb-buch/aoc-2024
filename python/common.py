"""Common utility fonctions."""

from pathlib import Path
from time import perf_counter_ns
from typing import Callable


def get_input_data(day: str = "01") -> str:
    input_path = Path(__file__).parent.parent / f"data/input-{day}.txt"
    return input_path.read_text("utf-8")


def time_function[T](func: Callable[..., T], *args, **kwargs) -> tuple[T, float]:
    start_time = perf_counter_ns()
    result = func(*args, **kwargs)
    return result, perf_counter_ns() - start_time


def pretty_duration_ns(duration_ns: float) -> str:
    units = ["ns", "μs", "ms", "s"]

    while duration_ns >= 1000:
        duration_ns /= 1000
        units.pop(0)
        if len(units) == 0:
            break

    return f"{duration_ns:.3f} {units[0]}"


def check_result(actual: str, expected: str) -> None:
    if actual != expected:
        if expected == "TODO":
            print("⚠️ No known solution... Can not check")
            return

        print("❌ Solution is invalid!")
        return
    print("✅ Solution is valid!")
