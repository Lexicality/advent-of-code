# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

from collections import defaultdict
from typing import DefaultDict, Iterable


def _windows(index: int, max: int) -> Iterable[int]:
    for i in range(index - 2, index + 1):
        if i >= 0 and i < max:
            yield i


def main(data: Iterable[str]) -> None:
    nums = [int(line) for line in data]
    max_window = len(nums) - 2

    windows: DefaultDict[int, int] = defaultdict(int)
    for i, num in enumerate(nums):
        print(i, num, end=" ")
        for win in _windows(i, max_window):
            windows[win] += num
            print(win, end=" ")
        print()

    sinking = 0
    last = windows[0]

    for num in windows.values():
        if num > last:
            sinking += 1
        last = num
        print(num)

    print(sinking)
