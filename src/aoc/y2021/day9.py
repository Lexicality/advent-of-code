from typing import Dict, Iterable
from termcolor import cprint

from aoc.utils import Coord2D


class Grid(Dict[Coord2D, int]):
    width: int = 0
    height: int = 0

    @classmethod
    def parse(cls, data: Iterable[str]):
        ret = cls()

        y = 0
        for line in data:
            x = 0
            for char in line:
                ret[Coord2D(x, y)] = int(char)
                x += 1
            # all lines should be the same width
            ret.width = x
            y += 1

        ret.height = y

        return ret

    def _check_valid(self, coord: Coord2D) -> bool:
        # yay python bullshit
        return (0 <= coord.x < self.width) and (0 <= coord.y < self.height)

    def check_lowspot(self, coord: Coord2D) -> bool:
        if not self._check_valid(coord):
            raise ValueError("Invalid coordinate!")

        pos = self[coord]

        for x in (-1, 1):
            target = coord + Coord2D(x, 0)
            if self._check_valid(target) and self[target] <= pos:
                return False

        for y in (-1, 1):
            target = coord + Coord2D(0, y)
            if self._check_valid(target) and self[target] <= pos:
                return False

        return True


def main(data: Iterable[str]):
    grid = Grid.parse(data)
    ret = 0

    for y in range(grid.height):
        for x in range(grid.width):
            coord = Coord2D(x, y)
            pos = grid[coord]
            if grid.check_lowspot(coord):
                ret += 1 + pos
                cprint(str(pos), attrs=["bold"], end="")
            else:
                cprint(str(pos), attrs=["dark"], end="")
        print()

    print(ret)
