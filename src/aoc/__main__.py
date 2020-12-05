import argparse
import importlib
import inspect
import os
import pathlib


def main():
    filename = inspect.getframeinfo(inspect.currentframe()).filename
    path = os.path.dirname(os.path.abspath(filename))
    here = pathlib.Path(path)

    available_days = list(p.stem for p in here.glob("day*.py"))
    available_days.sort()

    parser = argparse.ArgumentParser()
    parser.add_argument(
        "day",
        type=int,
        choices=[int(day.replace("day", "")) for day in available_days],
        help="Which day's code to run",
    )
    args = parser.parse_args()

    day = importlib.import_module(f"aoc.day{args.day}")
    day.main()


main()
