from typing import Iterable, List

from termcolor import cprint

from aoc.utils import Coord2D, Grid


class HeightGrid(Grid[int]):
    @classmethod
    def parse(cls, data: Iterable[str]):
        ret = cls()

        ret._fill(((int(char) for char in line) for line in data))

        return ret

    def check_lowspot(self, coord: Coord2D) -> bool:
        if not self._check_valid(coord):
            raise ValueError("Invalid coordinate!")

        pos = self[coord]

        for target in self._get_neighbours(coord, diagonal=False):
            if self._check_valid(target) and self[target] <= pos:
                return False

        return True

    def count_floodfill(self, starting_coord: Coord2D) -> int:
        """
        the data appears to bound every basin with 9s so I don't need to worry
        about ridges to other low spots (to confirm tho)
        """
        visited = set((starting_coord,))
        queue = list(self._get_neighbours(starting_coord, diagonal=False))
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
                    for coord in self._get_neighbours(target, diagonal=False)
                    if coord not in visited
                )
            )
        return size


def main(data: Iterable[str]):
    grid = HeightGrid.parse(data)
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
