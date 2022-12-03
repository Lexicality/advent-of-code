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

    parser = argparse.ArgumentParser(prog="python -m aoc")
    parser.add_argument("--example", default=False, action="store_true")

    year_args = parser.add_subparsers(dest="year")

    for year in here.glob("y*"):
        available_days = list(p.stem for p in year.glob("day*.py"))
        available_days.sort()

        year_parser = year_args.add_parser(year.stem.replace("y", ""))
        year_parser.add_argument(
            "day",
            type=int,
            choices=[int(day.replace("day", "")) for day in available_days],
            help="Which day's code to run",
        )

    args = parser.parse_args()

    day_data_file = pathlib.Path(
        "data",
        args.year,
        "example" if args.example else ".",
        f"{args.day:02}.txt",
    ).resolve()
    day_data = read_file(day_data_file)

    day = importlib.import_module(f"aoc.y{args.year}.day{args.day}")
    day.main(day_data)


main()
