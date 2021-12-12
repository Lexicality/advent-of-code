from typing import Iterable, List, Set, TypedDict

from termcolor import cprint

from aoc.utils import Coord2D, Grid

SQUID_STEPS = 100


class Octopod(TypedDict):
    energy: int
    flashed: bool


class SquidGrid(Grid[int]):
    @classmethod
    def parse(cls, data: Iterable[str]):
        ret = cls()

        ret._fill(
            (
                (
                    # {
                    #     "energy": int(char),
                    #     "flashed": False,
                    # }
                    int(char)
                    for char in line
                )
                for line in data
            )
        )

        return ret

    def _flash(self, coord: Coord2D) -> Iterable[Coord2D]:
        for neighbour in self._get_neighbours(coord, diagonal=True):
            energy = self[neighbour]
            energy += 1
            self[neighbour] = energy
            if energy > 9:
                yield neighbour

    def energize(self) -> int:
        flashed: Set[Coord2D] = set()
        flash_queue: List[Coord2D] = []
        for coord, energy in self.items():
            energy += 1
            self[coord] = energy
            if energy > 9:
                flash_queue.append(coord)

        while flash_queue:
            coord = flash_queue.pop()
            if coord in flashed:
                continue
            flashed.add(coord)
            flash_queue.extend(self._flash(coord))

        for coord, energy in self.items():
            if energy > 9:
                self[coord] = 0

        return len(flashed)

    def print(self) -> None:
        for y in range(self.height):
            for x in range(self.width):
                energy = self[Coord2D(x, y)]
                if energy == 0:
                    cprint(str(energy), attrs=["bold"], end=" ")
                else:
                    cprint(str(energy), attrs=["dark"], end=" ")
            print()
        print()


def main(data: Iterable[str]) -> None:
    grid = SquidGrid.parse(data)

    i = 0
    while True:
        i += 1
        print(i, end=" ", flush=i % 20 == 0)
        num_flashes = grid.energize()
        if num_flashes == 100:
            print("complete!", i)
            break
