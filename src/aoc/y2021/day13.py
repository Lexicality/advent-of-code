from __future__ import annotations

from typing import Iterator, List

from aoc.utils import Coord2D, Grid


class Transparency(Grid[bool]):
    @classmethod
    def parse(cls, data: Iterator[str]) -> Transparency:
        ret = Transparency()

        for line in data:
            if not line:
                break
            coord = Coord2D.parse(line)
            ret.width = max(ret.width, coord.x + 1)
            ret.height = max(ret.height, coord.y + 1)
            ret[coord] = True

        return ret

    def _calc_size(self) -> None:
        self.width = 0
        self.height = 0
        for coord in self.keys():
            self.width = max(self.width, coord.x + 1)
            self.height = max(self.height, coord.y + 1)

    def fold(self, axis: str, pos: int) -> None:
        if axis != "x" and axis != "y":
            raise ValueError(f"Unknown axis {axis}")

        axis_attr = 0 if axis == "x" else 1

        to_fold: List[Coord2D] = []
        for coord in self.keys():
            if coord[axis_attr] > pos:
                to_fold.append(coord)

        for coord in to_fold:
            del self[coord]
            # mirror across the axis
            new_val = pos - (coord[axis_attr] - pos)
            self[coord._replace(**{axis: new_val})] = True

        self._calc_size()

    def __str__(self) -> str:
        ret = ""
        for y in range(self.height):
            for x in range(self.width):
                if self.get(Coord2D(x, y)):
                    ret += "#"
                else:
                    ret += "."
            ret += "\n"
        return ret


def main(data: Iterator[str]) -> None:
    page = Transparency.parse(data)
    print(page)

    for command in data:
        print(command)
        _, _, cmd = command.split(" ")
        axis, pos = cmd.split("=")
        page.fold(axis, int(pos))
        print(page)
