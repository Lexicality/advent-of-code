from enum import Enum
from typing import Iterable, NamedTuple, Tuple


class Command(Enum):
    North = "N"
    South = "S"
    East = "E"
    West = "W"
    Left = "L"
    Right = "R"
    Forward = "F"


class Coord(NamedTuple):
    x: int
    y: int

    def __add__(self, other: "Coord") -> "Coord":
        return Coord(x=self.x + other.x, y=self.y + other.y)

    def __sub__(self, other: "Coord") -> "Coord":
        return Coord(x=self.x - other.x, y=self.y - other.y)

    def __mul__(self, other: int) -> "Coord":
        return Coord(x=self.x * other, y=self.y * other)

    def __str__(self):
        return f"[{self.x}, {self.y}]"


class Direction(Enum):
    North = Coord(x=0, y=1)
    South = Coord(x=0, y=-1)
    East = Coord(x=1, y=0)
    West = Coord(x=-1, y=0)


def rotate(what: Direction, how: Command, amount: int):
    assert amount % 90 == 0
    amount //= 90
    assert amount <= 4

    # Always rotate left
    if how == Command.Right:
        amount = 4 - amount

    for _ in range(amount):
        if what == Direction.North:
            what = Direction.West
        elif what == Direction.West:
            what = Direction.South
        elif what == Direction.South:
            what = Direction.East
        elif what == Direction.East:
            what = Direction.North

    return what


def comandorate(data: Iterable[str]) -> Iterable[Tuple[Command, int]]:
    for line in data:
        yield Command(line[0]), int(line[1:])


def main(data: Iterable[str]):
    dir = Direction.East
    pos = Coord(0, 0)
    for cmd, amt in comandorate(data):
        print("Currently at", pos, "facing", dir)
        if cmd == Command.Left or cmd == Command.Right:
            print("Turning", cmd.name, amt, "degrees")
            dir = rotate(dir, cmd, amt)
            continue
        if cmd == Command.North:
            move_dir = Direction.North
        elif cmd == Command.South:
            move_dir = Direction.South
        elif cmd == Command.East:
            move_dir = Direction.East
        elif cmd == Command.West:
            move_dir = Direction.West
        else:
            print("(fwd)")
            move_dir = dir

        print("Moving", move_dir.name, amt, "units")
        pos += move_dir.value * amt

    print("end pos", pos)
    print(abs(pos.x) + abs(pos.y))
