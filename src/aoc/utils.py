from typing import Iterable


def read_file(day: str) -> Iterable[str]:
    with open(f"data/{day}.txt", "r") as f:
        for line in f:
            yield line.strip()
