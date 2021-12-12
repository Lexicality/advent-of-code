from __future__ import annotations

from typing import DefaultDict, Iterator, List, NamedTuple, Set


class Cave:
    key: str
    smol: bool
    links: Set[Cave]

    def __init__(self, key: str) -> None:
        self.key = key
        self.smol = key.lower() == key
        self.links = set()

    def __hash__(self) -> int:
        return hash(self.key)

    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Cave) and __o.key == self.key

    @property
    def prunable(self):
        key = self.key
        return (
            key != "start"
            and key != "end"
            and len(self.links) == 1
            and next(iter(self.links)).smol
        )

    def __str__(self) -> str:
        links = [cave.key for cave in self.links]
        return f"{self.key}: {links}"


class CaveDict(DefaultDict[str, Cave]):
    def __missing__(self, __key: str) -> Cave:
        cave = Cave(__key)
        self[__key] = cave
        return cave


class CavePath(NamedTuple):
    cave: Cave
    path: List[Cave]
    seen: Set[Cave]
    part2: bool

    def __str__(self) -> str:
        return "->".join(cave.key for cave in self.path) + "->" + self.cave.key


def _travel(cave_path: CavePath) -> Iterator[CavePath]:
    cave, path, seen, part2 = cave_path
    if cave.smol:
        seen = seen | set((cave,))
    path = path + [cave]
    for link in cave.links:
        if link.key == "start":
            continue
        if link in seen:
            if not part2:
                yield CavePath(link, path, seen, True)
            continue
        yield CavePath(link, path, seen, part2)


def main(data: Iterator[str]) -> None:
    caves = CaveDict()

    for line in data:
        c1, c2 = line.split("-")
        cave1 = caves[c1]
        cave2 = caves[c2]
        cave1.links.add(cave2)
        cave2.links.add(cave1)

    donepaths: List[CavePath] = []
    paths: List[CavePath] = [CavePath(caves["start"], [], set(), False)]

    while paths:
        path = paths.pop()
        for newpath in _travel(path):
            if newpath.cave.key == "end":
                donepaths.append(newpath)
            else:
                paths.append(newpath)

    # for path in donepaths:
    #     print(path)

    print(len(donepaths))
