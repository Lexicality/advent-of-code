# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

from __future__ import annotations

from typing import Iterable, Iterator, NamedTuple

import pandas as pd

from aoc.utils import Coord2D


def _steppe(start: int, end: int) -> int:
    if start > end:
        return -1
    elif start < end:
        return 1
    return 0


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
        step = Coord2D(_steppe(start.x, end.x), _steppe(start.y, end.y))
        pos = start
        yield pos
        # probably fine
        while True:
            pos += step
            yield pos
            if pos == end:
                break


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
        for coord in vent:
            grid[coord.x][coord.y] += 1

    # print(grid.replace(0, ".").to_string(header=False, index=False))

    overlaps = 0
    # gnee
    for col_id in grid.columns:
        for value in grid[col_id]:
            if value > 1:
                overlaps += 1

    print(overlaps)
