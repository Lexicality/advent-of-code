import heapq
import sys
from typing import (
    Generic,
    Iterable,
    Iterator,
    List,
    NamedTuple,
    Optional,
    Set,
    Tuple,
    TypeVar,
)

from termcolor import cprint

from aoc.utils import Coord2D, Grid

H = TypeVar(
    "H",
    # technically can be anything comparible but obviously python's type system
    # doesn't support that so let's go with numbers because that's all I'm using
    int,
    float,
)
V = TypeVar("V")


class WorkingSet(Generic[H, V]):
    heap: List[Tuple[H, V]]
    heapset: Set[V]

    def __init__(self) -> None:
        self.heap = []
        self.heapset = set()

    def add(self, value: V, heuristic: H) -> None:
        self.heapset.add(value)
        heapq.heappush(self.heap, (heuristic, value))

    def replace(self, value: V, heuristic: H) -> None:
        self.heap = [val for val in self.heap if val[1] != value]
        heapq.heappush(self.heap, (heuristic, value))

    def add_or_replace(self, value: V, heuristic: H) -> None:
        if value in self.heapset:
            self.replace(value, heuristic)
        else:
            self.add(value, heuristic)

    def pop(self) -> V:
        _, value = heapq.heappop(self.heap)
        self.heapset.remove(value)
        return value

    def __contains__(self, value: V) -> bool:
        return value in self.heapset

    def __len__(self) -> int:
        return len(self.heap)

    def __bool__(self) -> bool:
        return bool(self.heap)


class Chiton(NamedTuple):
    risk: int
    cheapest_cost: int
    cheapest_neighbour: Optional[Coord2D]


class ChitonCave(Grid[Chiton]):
    _goal: Coord2D

    @classmethod
    def parse(cls, data: Iterator[str]):
        ret = cls()
        ret._fill(
            (
                (
                    Chiton(
                        risk=int(risk),
                        cheapest_cost=sys.maxsize,
                        cheapest_neighbour=None,
                    )
                    for risk in line
                )
                for line in data
            )
        )
        for y in range(6):
            for x in range(6):
                if x == 0 and y == 0:
                    continue
                for coord in ret._iterate_grid():
                    newcoord = Coord2D(
                        x=coord.x + ret.width * x,
                        y=coord.y + ret.height * y,
                    )
                    existing = ret[coord]
                    ret[newcoord] = existing._replace(
                        risk=((existing.risk + x + y) % 9) or 9
                    )
        ret.width *= 5
        ret.height *= 5

        ret._goal = Coord2D(ret.width - 1, ret.height - 1)
        return ret

    def _heuristic(self, coord: Coord2D) -> int:
        return coord.distance(self._goal)

    def _get_path(self) -> List[Coord2D]:
        current = self._goal
        path: List[Coord2D] = []
        while current:
            path.append(current)
            _, _, current = self[current]
        return path

    def print(self, path: Optional[Iterable[Coord2D]] = None) -> None:
        if path:
            pathset = set(path)
        else:
            pathset = set((Coord2D(0, 0), self._goal))

        for y in range(self.height):
            for x in range(self.width):
                coord = Coord2D(x, y)
                risk, _, _ = self[coord]
                if coord in pathset:
                    cprint(str(risk), attrs=["bold"], end="")
                else:
                    cprint(str(risk), attrs=["dark"], end="")

            print()
        print()

    def a_star(self) -> List[Coord2D]:
        # self.print()

        start = Coord2D(0, 0)
        self[start] = self[start]._replace(cheapest_cost=0)

        working_set = WorkingSet[int, Coord2D]()
        working_set.add(start, self._heuristic(start))

        while working_set:
            coord = working_set.pop()
            if coord == self._goal:
                path = self._get_path()
                self.print(path)
                return path
            _, cost, _ = self[coord]

            for neighbour in self._get_neighbours(coord, diagonal=False):
                neighbour_risk, neighbour_cost, _ = self[neighbour]

                new_cost = cost + neighbour_risk
                if new_cost < neighbour_cost:
                    self[neighbour] = Chiton(neighbour_risk, new_cost, coord)
                    working_set.add_or_replace(
                        neighbour,
                        new_cost + self._heuristic(neighbour),
                    )

        raise RuntimeError("Could not find goal?!")


def main(data: Iterator[str]) -> None:
    grid = ChitonCave.parse(data)

    path = grid.a_star()

    total_risk = 0
    start = Coord2D(0, 0)
    for coord in path:
        if coord != start:
            risk, _, _ = grid[coord]
            total_risk += risk

    print(total_risk)
