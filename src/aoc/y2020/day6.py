import itertools
from typing import Iterable, List


def pred(line: str) -> bool:
    return line != ""


PersonAnswer = List[str]
GroupAnswer = List[PersonAnswer]


def extract_group(data: Iterable[str]) -> GroupAnswer:
    group = itertools.takewhile(pred, data)
    return [list(person) for person in group]


def groups(data: Iterable[str]) -> Iterable[GroupAnswer]:
    while group := extract_group(data):
        yield group


def main(data: Iterable[str]):
    total = sum(
        # amazing
        len(set(itertools.chain.from_iterable(group)))
        for group in groups(data)
    )
    print(total)
