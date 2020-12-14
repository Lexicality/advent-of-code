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


def rotate(pos: Coord, direction: Command, amount: int):
    assert amount % 90 == 0
    amount //= 90
    assert amount <= 4

    # Always rotate right
    if direction == Command.Left:
        amount = 4 - amount

    for _ in range(amount):
        x, y = pos
        pos = Coord(x=y, y=-x)

    return pos


def comandorate(data: Iterable[str]) -> Iterable[Tuple[Command, int]]:
    for line in data:
        yield Command(line[0]), int(line[1:])


def main(data: Iterable[str]):
    ship_pos = Coord(0, 0)
    waypoint = Coord(10, 1)
    for cmd, amt in comandorate(data):
        print("Currently at", ship_pos, "waypoint is at", waypoint)

        if cmd == Command.Forward:
            print("Moving to waypoint", amt, "times")
            ship_pos += waypoint * amt
            continue
        elif cmd == Command.Left or cmd == Command.Right:
            print("Rotating waypoint", cmd.name, amt, "degrees")
            waypoint = rotate(waypoint, cmd, amt)
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
            assert False

        print("Moving waypoint", move_dir.name, amt, "units")
        waypoint += move_dir.value * amt

    print("end pos", ship_pos)
    print(abs(ship_pos.x) + abs(ship_pos.y))
