from __future__ import annotations

from math import ceil, floor
from typing import Iterator, List, Literal, Optional, Tuple, Union

PairValue = Union[int, "Pair"]


class Pair:
    """
    It's a tree! It's a linked list! It's hell!
    """

    left: PairValue
    right: PairValue
    parent: Optional[Pair] = None

    def __init__(self, left: PairValue, right: PairValue) -> None:
        if isinstance(left, Pair):
            left.parent = self
        self.left = left
        if isinstance(right, Pair):
            right.parent = self
        self.right = right

    def __str__(self) -> str:
        return f"[{self.left},{self.right}]"

    def stringulate(self, depth=0) -> str:
        return str(self)
        if isinstance(self.left, Pair):
            leftval = self.left.stringulate(depth + 1)
        else:
            leftval = str(self.left)
        if isinstance(self.right, Pair):
            rightval = self.right.stringulate(depth + 1)
        else:
            rightval = str(self.right)
        colour = f"\N{CSI}{31 + depth}m"
        return colour + "[" + leftval + colour + "," + rightval + colour + "]\N{CSI}0m"

    def _split(self, num: int) -> Pair:
        return Pair(
            floor(num / 2),
            ceil(num / 2),
        )

    def _check_split(self, side: Literal["left", "right"]) -> bool:
        sideval = getattr(self, side)
        if isinstance(sideval, Pair):
            return sideval.check_split()
        elif sideval < 10:
            return False
        new_pair = self._split(sideval)
        new_pair.parent = self
        setattr(self, side, new_pair)
        return True

    def check_split(self) -> bool:
        return self._check_split("left") or self._check_split("right")

    def _check_explode(self, side: Literal["left", "right"], depth: int) -> bool:
        sideval = getattr(self, side)
        if isinstance(sideval, int):
            return False
        return sideval.check_explode(depth)

    def _explode_me(self, child: Pair) -> None:
        if self.left is child:
            self.left = 0
        elif self.right is child:
            self.right = 0
        else:
            raise ValueError("That's not a child of mine!")

    def check_explode(self, depth: int) -> bool:
        if depth < 4:
            depth += 1
            return (
                self._check_explode("left", depth)
                or
                #
                self._check_explode("right", depth)
            )

        assert self.parent is not None
        self.parent.explode_upwards(self, (self.left, self.right))  # type: ignore
        self.parent._explode_me(self)
        return True

    def _explode_down(self, value: int, side: Literal["left", "right"]) -> None:
        sideval: PairValue = getattr(self, side)
        if isinstance(sideval, int):
            setattr(self, side, sideval + value)
        else:
            sideval._explode_down(value, side)

    def _explode_up(
        self,
        not_this_one: Pair,
        value: int,
        side: Literal["left", "right"],
    ) -> bool:
        sideval: PairValue = getattr(self, side)
        if isinstance(sideval, int):
            setattr(self, side, sideval + value)
            return True
        elif sideval == not_this_one:
            return False
        sideval._explode_down(value, "left" if side == "right" else "right")
        return True

    def explode_upwards(
        self,
        not_this_one: Pair,
        values: Tuple[int, int],
        process_left: bool = True,
        process_right: bool = True,
    ) -> None:
        pass
        if process_left:
            if self._explode_up(not_this_one, values[0], "left"):
                process_left = False
        if process_right:
            if self._explode_up(not_this_one, values[1], "right"):
                process_right = False
        if self.parent and (process_left or process_right):
            self.parent.explode_upwards(self, values, process_left, process_right)

    def add(self, other: Pair) -> Pair:
        p = Pair(self, other)
        p.balance()
        return p

    def balance(self):
        while True:
            if self.check_explode(0):
                continue
            if self.check_split():
                continue
            break

    def _get_magnitude(self, side: Literal["left", "right"]) -> int:
        val: int
        sideval: PairValue = getattr(self, side)
        if isinstance(sideval, Pair):
            val = sideval.magnitude()
        else:
            val = sideval
        if side == "left":
            return val * 3
        else:
            return val * 2

    def magnitude(self) -> int:
        return self._get_magnitude("left") + self._get_magnitude("right")

    def copy(self) -> Pair:
        leftval = self.left
        if isinstance(leftval, Pair):
            leftval = leftval.copy()
        rightval = self.right
        if isinstance(rightval, Pair):
            rightval = rightval.copy()
        return Pair(leftval, rightval)

    @classmethod
    def _readside(cls, input: List[str]) -> PairValue:
        val = input.pop(0)
        if val == "[":
            pair = cls.parse(input)
            # Always going to be something after a pair, either a `]` or `,`
            input.pop(0)
            return pair
        ret = val
        while (val := input.pop(0)) not in (",", "]"):
            ret += val
        return int(ret)

    @classmethod
    def parse(cls, input: List[str]) -> Pair:
        return cls(
            cls._readside(input),
            cls._readside(input),
        )


def _get_pair(line: str) -> Pair:
    input = list(line)[1:]
    res = Pair.parse(input)
    if len(input) > 0:
        raise ValueError("Extra unparsed input: " + "".join(input))
    return res


def main(data: Iterator[str]):
    nums = [_get_pair(line) for line in data]

    maxmag = 0
    for num1 in nums:
        for num2 in nums:
            if num1 == num2:
                continue
            mag = num1.copy().add(num2.copy()).magnitude()
            if mag > maxmag:
                maxmag = mag
    print(maxmag)
