from __future__ import annotations

from enum import Enum
from typing import NamedTuple


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
