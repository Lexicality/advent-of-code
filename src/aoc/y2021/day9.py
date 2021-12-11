from typing import Dict, Iterable, List

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

    def _get_neighbours(self, coord: Coord2D) -> Iterable[Coord2D]:
        for x in (-1, 1):
            yield coord + Coord2D(x, 0)

        for y in (-1, 1):
            yield coord + Coord2D(0, y)

    def check_lowspot(self, coord: Coord2D) -> bool:
        if not self._check_valid(coord):
            raise ValueError("Invalid coordinate!")

        pos = self[coord]

        for target in self._get_neighbours(coord):
            if self._check_valid(target) and self[target] <= pos:
                return False

        return True

    def count_floodfill(self, starting_coord: Coord2D) -> int:
        """
        the data appears to bound every basin with 9s so I don't need to worry
        about ridges to other low spots (to confirm tho)
        """
        visited = set((starting_coord,))
        queue = list(self._get_neighbours(starting_coord))
        size = 1
        while queue:
            target = queue.pop()
            if target in visited or not self._check_valid(target):
                continue
            visited.add(target)
            pos = self[target]
            if pos == 9:
                continue
            size += 1
            queue.extend(
                (
                    coord
                    for coord in self._get_neighbours(target)
                    if coord not in visited
                )
            )
        return size


def main(data: Iterable[str]):
    grid = Grid.parse(data)
    ret = 0

    lowspots: List[Coord2D] = []
    for y in range(grid.height):
        for x in range(grid.width):
            coord = Coord2D(x, y)
            pos = grid[coord]
            if grid.check_lowspot(coord):
                lowspots.append(coord)
                cprint(str(pos), color="red", attrs=["bold"], end="")
            elif pos < 9:
                cprint(str(pos), end="")
            else:
                cprint(str(pos), attrs=["concealed"], end="")
        print()

    basins = [grid.count_floodfill(coord) for coord in lowspots]
    biggest = sorted(basins, reverse=True)[:3]
    ret = biggest[0] * biggest[1] * biggest[2]

    print(ret)
