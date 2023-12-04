from __future__ import annotations

from typing import Iterable, List
from dataclasses import dataclass
import re

CARD_REGEX = re.compile(r"^Card\s+(\d+): (.+) \| (.+)$")


@dataclass
class Card:
    id: int
    winning_numbers: set[int]
    card_numbers: set[int]

    def parse(line: str) -> Card:
        matches = CARD_REGEX.match(line)
        assert matches is not None

        return Card(
            int(matches[1]),
            set((int(num) for num in matches[2].split())),
            set((int(num) for num in matches[3].split())),
        )

    def num_winning_nums(self) -> int:
        return len(self.winning_numbers.intersection(self.card_numbers))


class CardQueue:
    queue: List[int]

    def __init__(self) -> None:
        self.queue = []

    def pop(self) -> int:
        try:
            return self.queue.pop(0)
        except IndexError:
            return 0

    def incr(self, amt: int):
        for index in range(amt):
            try:
                self.queue[index] += 1
            except IndexError:
                self.queue.append(1)


def main(data: Iterable[str]) -> None:
    ret = 0
    queue = CardQueue()
    for line in data:
        card = Card.parse(line)
        points = card.num_winning_nums()
        count = queue.pop() + 1
        if points > 0:
            for _ in range(count):
                queue.incr(points)
        ret += count

    print(ret)
