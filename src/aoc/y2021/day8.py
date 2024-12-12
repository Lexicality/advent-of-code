# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

from typing import Iterable, Set

ALL_SEGS: set[str] = set(("a", "b", "c", "d", "e", "f", "g"))


class Display:
    """
      0:      1:      2:      3:      4:
     aaaa    ....    aaaa    aaaa    ....
    b    c  .    c  .    c  .    c  b    c
    b    c  .    c  .    c  .    c  b    c
     ....    ....    dddd    dddd    dddd
    e    f  .    f  e    .  .    f  .    f
    e    f  .    f  e    .  .    f  .    f
     gggg    ....    gggg    gggg    ....

      5:      6:      7:      8:      9:
     aaaa    aaaa    aaaa    aaaa    aaaa
    b    .  b    .  .    c  b    c  b    c
    b    .  b    .  .    c  b    c  b    c
     dddd    dddd    ....    dddd    dddd
    .    f  e    f  .    f  e    f  .    f
    .    f  e    f  .    f  e    f  .    f
     gggg    gggg    ....    gggg    gggg
    """

    a: Set[str]
    b: Set[str]
    c: Set[str]
    d: Set[str]
    e: Set[str]
    f: Set[str]
    g: Set[str]

    def __init__(self) -> None:
        self.a = set()
        self.b = set()
        self.c = set()
        self.d = set()
        self.e = set()
        self.f = set()
        self.g = set()

    def refine(self, digit: str) -> None:
        """
         aaaa
        b    c
        b    c
         dddd
        e    f
        e    f
         gggg
        """
        numseg = len(digit)
        if numseg == 5:
            return
        chars = sorted(digit)
        charset = set(chars)
        if numseg == 2:
            self.c |= charset
            self.f |= charset
        elif numseg == 3:
            for char in chars:
                if char not in self.c and char not in self.f:
                    self.a.add(char)
                    break
        elif numseg == 4:
            for char in chars:
                if char not in self.c and char not in self.f:
                    self.b.add(char)
                    self.d.add(char)
        elif numseg == 6:
            missing = (ALL_SEGS - charset).pop()
            if missing in self.c:
                # 6
                self.f.remove(missing)
                self.c -= self.f
            elif missing in self.d:
                # 0
                self.b.remove(missing)
                self.d -= self.b
            else:
                self.e.add(missing)
        elif numseg == 7:
            self.g = ALL_SEGS - (self.a | self.b | self.c | self.d | self.e | self.f)

    def classify(self, digit: str) -> int:
        numseg = len(digit)
        charset = set(digit)
        if numseg == 2:
            return 1
        elif numseg == 3:
            return 7
        elif numseg == 4:
            return 4
        elif numseg == 5:
            BASE = self.a | self.d | self.g
            TWO = BASE | self.c | self.e
            if charset == TWO:
                return 2
            THREE = BASE | self.c | self.f
            if charset == THREE:
                return 3
            FIVE = BASE | self.b | self.f
            if charset == FIVE:
                return 5
        elif numseg == 6:
            BASE = self.a | self.b | self.f | self.g
            ZERO = BASE | self.c | self.e
            if charset == ZERO:
                return 0
            SIX = BASE | self.d | self.e
            if charset == SIX:
                return 6
            NINE = BASE | self.c | self.d
            if charset == NINE:
                return 9
        else:
            return 8
        raise ValueError("oh no")

    def __str__(self) -> str:
        return (
            f"a: {self.a} b: {self.b} c: {self.c}"
            f" d: {self.d} e: {self.e} f: {self.f} g: {self.g}"
        )


def main(data: Iterable[str]):
    res = 0

    for line in data:
        display = Display()
        unique, output = line.split(" | ")
        for digit in sorted(unique.split(" "), key=lambda a: len(a)):
            display.refine(digit)
        out_digits = ""
        for digit in output.split(" "):
            out_digits += str(display.classify(digit))
        print(out_digits)
        res += int(out_digits)

    print(res)
