from __future__ import annotations

from typing import Iterable, Iterator, NamedTuple

import pandas as pd

from aoc.utils import Coord2D


class Vent(NamedTuple):
    start: Coord2D
    end: Coord2D

    @classmethod
    def parse(cls, data: str) -> Vent:
        start, end = data.split(" -> ")
        return Vent(start=Coord2D.parse(start), end=Coord2D.parse(end))

    def part1filter(self) -> bool:
        return self.start.x == self.end.x or self.start.y == self.end.y

    def get_max(self) -> Coord2D:
        return self.start.get_max(self.end)

    def __str__(self) -> str:
        return f"{self.start} -> {self.end}"

    def __iter__(self) -> Iterator[Coord2D]:
        start = self.start
        end = self.end
        if start > end:
            end, start = start, end

        for x in range(start.x, end.x + 1):
            for y in range(start.y, end.y + 1):
                yield Coord2D(x, y)


def main(data: Iterable[str]) -> None:
    vents = [Vent.parse(line) for line in data]

    gridmax = Coord2D(0, 0)
    for vent in vents:
        gridmax = gridmax.get_max(vent.get_max())

    grid = pd.DataFrame(
        [
            [0 for x in range(gridmax.x + 1)]
            #
            for y in range(gridmax.y + 1)
        ]
    )

    for vent in vents:
        print(vent, end=" ")
        if not vent.part1filter():
            print("discarded")
            continue
        print()
        print("  ", end="")
        for coord in vent:
            print(coord, end=", ")
            grid[coord.x][coord.y] += 1
        print()

    print(grid)

    overlaps = 0
    # gnee
    for col_id in grid.columns:
        for value in grid[col_id]:
            if value > 1:
                overlaps += 1

    print(overlaps)
