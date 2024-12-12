# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

from typing import Iterable

ROWS = list(range(128))
COLS = list(range(8))


def process_pass(bpass: str) -> int:
    my_rows = ROWS
    my_cols = COLS
    for char in bpass:
        if char == "F":
            my_rows = my_rows[: len(my_rows) // 2]
        elif char == "B":
            my_rows = my_rows[len(my_rows) // 2 :]
        elif char == "L":
            my_cols = my_cols[: len(my_cols) // 2]
        elif char == "R":
            my_cols = my_cols[len(my_cols) // 2 :]

    print(bpass, my_rows, my_cols)

    return my_rows[0] * 8 + my_cols[0]


def main(data: Iterable[str]):
    all_passes = list(process_pass(bp) for bp in data)

    all_passes.sort()

    for i in range(len(all_passes)):
        this = all_passes[i]
        next = all_passes[i + 1]
        if next > this + 1:
            print(this + 1)
            return

    print("how did you get here without raising?")

    print(all_passes)
