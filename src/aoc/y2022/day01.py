from __future__ import annotations

from typing import Iterable, List


def main(data: Iterable[str]) -> None:
    elves: List[str] = []
    running_elf = 0
    for line in data:
        line = line.strip()
        if not line:
            elves.append(running_elf)
            running_elf = 0
            continue
        running_elf += int(line)

    if running_elf:
        elves.append(running_elf)

    elves.sort(reverse=True)

    print(sum(elves[:3]))
