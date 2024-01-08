import math
from typing import Iterable

from aoc.utils import Coord2D


def _get_coords(data: Iterable[str]) -> Iterable[Coord2D]:
    for line in data:
        x, y = line.split(",")
        yield Coord2D(int(x), int(y))


def main(data: Iterable[str]):
    coords = sorted(_get_coords(data), key=lambda c: c[0] * c[1])
    gridmin = Coord2D(math.inf, math.inf)  # type: ignore
    gridmax = Coord2D(-math.inf, -math.inf)  # type: ignore

    for coord in coords:
        gridmin = Coord2D(x=min(gridmin.x, coord.x), y=min(gridmin.y, coord.y))
        gridmax = Coord2D(x=max(gridmax.x, coord.x), y=max(gridmax.y, coord.y))

    print(gridmin.x, gridmin.y, gridmax.x, gridmax.y)

    MAX_DISTANCE = 10000

    size = 0

    for x in range(gridmin.x, gridmax.x + 1):
        for y in range(gridmin.y, gridmax.y + 1):
            to_check = Coord2D(x, y)
            distance = sum(coord.distance(to_check) for coord in coords)
            if distance < MAX_DISTANCE:
                size += 1

    print(size)
