from __future__ import annotations

from enum import Enum
from typing import Dict, Iterable, Iterator, NamedTuple, TypeVar


class Coord2D(NamedTuple):
    x: int
    y: int

    @classmethod
    def parse(cls, data: str) -> Coord2D:
        x, y = data.split(",")
        return Coord2D(x=int(x), y=int(y))

    def __add__(self, other: Coord2D) -> Coord2D:
        return Coord2D(x=self.x + other.x, y=self.y + other.y)

    def __sub__(self, other: Coord2D) -> Coord2D:
        return Coord2D(x=self.x - other.x, y=self.y - other.y)

    def __mul__(self, other: int) -> Coord2D:
        return Coord2D(x=self.x * other, y=self.y * other)

    def __str__(self):
        return f"[{self.x}, {self.y}]"

    def distance(self, other: Coord2D) -> int:
        return abs(self.x - other.x) + abs(self.y - other.y)

    def get_max(self, other: Coord2D) -> Coord2D:
        return Coord2D(max(self.x, other.x), max(self.y, other.y))

    def get_min(self, other: Coord2D) -> Coord2D:
        return Coord2D(min(self.x, other.x), min(self.y, other.y))


class Direction(Enum):
    North = Coord2D(x=0, y=1)
    South = Coord2D(x=0, y=-1)
    East = Coord2D(x=1, y=0)
    West = Coord2D(x=-1, y=0)


class Coord3D(NamedTuple):
    x: int
    y: int
    z: int

    @classmethod
    def parse(cls, data: str) -> Coord3D:
        x, y, z = data.split(",")
        return Coord3D(x=int(x), y=int(y), z=int(z))

    def __add__(self, other: Coord3D) -> Coord3D:
        return Coord3D(x=self.x + other.x, y=self.y + other.y, z=self.z + other.z)

    def __sub__(self, other: Coord3D) -> Coord3D:
        return Coord3D(x=self.x - other.x, y=self.y - other.y, z=self.z - other.z)

    def __mul__(self, other: int) -> "Coord3D":
        return Coord3D(x=self.x * other, y=self.y * other, z=self.z * other)

    def __str__(self):
        return f"[{self.x}, {self.y}, {self.z}]"

    def distance(self, other: Coord3D) -> int:
        return abs(self.x - other.x) + abs(self.y - other.y) + abs(self.z - other.z)

    def get_max(self, other: Coord3D) -> Coord3D:
        return Coord3D(max(self.x, other.x), max(self.y, other.y), max(self.z, other.z))

    def get_min(self, other: Coord3D) -> Coord3D:
        return Coord3D(min(self.x, other.x), min(self.y, other.y), min(self.z, other.z))


T = TypeVar("T")


class Grid(Dict[Coord2D, T]):
    width: int = 0
    height: int = 0

    def _fill(self, data: Iterable[Iterable[T]]) -> None:
        """
        Fills the grid with data, expecting it to come in like a standard AOC
        data dump, eg the first iterable is rows and the second is columns

        eg (x, y):
        [[(0,0), (1, 0), (2, 0)], [(0, 1), (1, 1), ...], ...]

        also this doesn't support sparse data so make sure all rows are the same
        length or you're going to have a very bad time
        """

        y = 0
        for row in data:
            x = 0
            for item in row:
                self[Coord2D(x, y)] = item
                x += 1
            y += 1

        self.width = x
        self.height = y

    def _check_valid(self, coord: Coord2D) -> bool:
        # yay python bullshit
        return (0 <= coord.x < self.width) and (0 <= coord.y < self.height)

    def _get_neighbours(self, coord: Coord2D, diagonal: bool) -> Iterator[Coord2D]:
        for x in (-1, 0, 1):
            for y in (-1, 0, 1):
                if x == 0 and y == 0:
                    continue
                elif not diagonal and not (x == 0 or y == 0):
                    continue
                neighbour = coord + Coord2D(x, y)
                if not self._check_valid(neighbour):
                    continue
                yield neighbour

    def _iterate_grid(self) -> Iterator[Coord2D]:
        for y in range(self.height):
            for x in range(self.width):
                yield Coord2D(x, y)
