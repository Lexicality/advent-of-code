from enum import Enum
from typing import Dict, Iterable, NamedTuple


class Coord(NamedTuple):
    x: int
    y: int

    def __add__(self, other: "Coord") -> "Coord":
        return Coord(x=self.x + other.x, y=self.y + other.y)

    def __sub__(self, other: "Coord") -> "Coord":
        return Coord(x=self.x - other.x, y=self.y - other.y)

    def __str__(self):
        return f"[{self.x}, {self.y}]"


class GridCell(Enum):
    Floor = "."
    Chair = "L"
    OccupiedChair = "#"


DIRECTIONS = [
    Coord(-1, -1),
    Coord(0, -1),
    Coord(1, -1),
    Coord(-1, 0),
    Coord(1, 0),
    Coord(-1, 1),
    Coord(0, 1),
    Coord(1, 1),
]


class Grid(Dict[Coord, GridCell]):
    _max: Coord(-1, -1)

    def read_data(self, data: Iterable[str]):
        y = 0
        for line in data:
            for x in range(len(line)):
                self[Coord(x, y)] = GridCell(line[x])
            y += 1

        gridmax = Coord(-1, -1)

        for coord in self.keys():
            gridmax = Coord(x=max(gridmax.x, coord.x), y=max(gridmax.y, coord.y))

        self._max = Coord(gridmax.x + 1, gridmax.y + 1)

    def get_neighbours(self, coord: Coord) -> Iterable[GridCell]:
        for dir in DIRECTIONS:
            try:
                yield self[coord + dir]
            except KeyError:
                pass

    def get_num_occupied_neighbours(self, coord: Coord) -> int:
        return sum(
            1 if cell == GridCell.OccupiedChair else 0
            for cell in self.get_neighbours(coord)
        )

    def __str__(self):
        ret = ""
        for y in range(self._max.y):
            for x in range(self._max.x):
                ret += self[Coord(x, y)].value
            ret += "\n"
        return ret.strip()


def simulate(grid: Grid) -> bool:
    changed = False
    # We have to precalculate everything to update simultaneously
    neighbours = {
        coord: grid.get_num_occupied_neighbours(coord) for coord in grid.keys()
    }

    for coord, cell in grid.items():
        if cell == GridCell.Chair:
            if neighbours[coord] == 0:
                changed = True
                grid[coord] = GridCell.OccupiedChair
        elif cell == GridCell.OccupiedChair:
            if neighbours[coord] >= 4:
                changed = True
                grid[coord] = GridCell.Chair

    return changed


def main(data: Iterable[str]):
    grid = Grid()
    grid.read_data(data)

    # print(str(grid))
    # print()

    # print(list(grid.get_neighbours(Coord(9, 0))))

    # for _ in range(10):
    #     changed = simulate(grid)
    #     print(str(grid))
    #     print(changed)
    #     print()

    while simulate(grid):
        pass
        # print(str(grid))
        # print()

    print(sum(1 if cell == GridCell.OccupiedChair else 0 for cell in grid.values()))
