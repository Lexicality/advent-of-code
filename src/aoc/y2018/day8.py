import itertools
import re
from typing import Iterator, List

A = ord("A")
namer = itertools.cycle(chr(A + i) for i in range(26))


def spliter(data: Iterator[str]) -> Iterator[str]:
    """https://stackoverflow.com/a/9770397/823542"""
    pattern = re.compile(r"[^ ]+")
    return (match[0] for line in data for match in pattern.finditer(line))


class Node:
    name: str
    children: List["Node"]
    metadata: List[int]

    def __init__(self, input: Iterator[int]):
        self.name = next(namer)
        num_children = next(input)
        num_metadata = next(input)
        self.children = [Node(input) for _ in range(num_children)]
        self.metadata = [next(input) for _ in range(num_metadata)]

    @property
    def all_metadata(self) -> List[int]:
        return self.metadata + list(
            itertools.chain.from_iterable(c.all_metadata for c in self.children)
        )

    def __str__(self) -> str:
        kids = "\n\t".join(str(c) for c in self.children)
        return (
            f"Node {self.name}"
            f" [\n\t{kids}\n]"
            f" [{','.join(str (m) for m in self.metadata)}]"
        )


def main(data: Iterator[str]):
    numbers = (int(n) for n in spliter(data))

    root = Node(numbers)
    print(root)
    print(root.all_metadata)
    print(sum(root.all_metadata))
