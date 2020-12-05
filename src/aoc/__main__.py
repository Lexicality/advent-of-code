import argparse
import importlib
import inspect
import os
import pathlib
from typing import Iterable


def read_file(path: pathlib.Path) -> Iterable[str]:
    with open(path, "r") as f:
        for line in f:
            yield line.strip()


def main():
    filename = inspect.getframeinfo(inspect.currentframe()).filename
    path = os.path.dirname(os.path.abspath(filename))
    here = pathlib.Path(path)

    available_days = list(p.stem for p in here.glob("day*.py"))
    available_days.sort()

    parser = argparse.ArgumentParser()
    parser.add_argument("--example", default=False, action="store_true")
    parser.add_argument(
        "day",
        type=int,
        choices=[int(day.replace("day", "")) for day in available_days],
        help="Which day's code to run",
    )
    args = parser.parse_args()

    day_data_file = pathlib.Path(
        "data", "example" if args.example else ".", f"{args.day}.txt"
    ).resolve()
    day_data = read_file(day_data_file)

    day = importlib.import_module(f"aoc.day{args.day}")
    day.main(day_data)


main()
