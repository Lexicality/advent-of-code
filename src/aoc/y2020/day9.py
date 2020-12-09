import itertools
from typing import Iterable, List

PREAMBLE = 25


def has_sum(window: List[int], target: int) -> bool:
    count = len(window)
    for i in range(count):
        for j in range(i, count):
            if window[i] + window[j] == target:
                return True

    return False


def main(data: Iterable[str]):
    nums = (int(line) for line in data)

    window = list(itertools.islice(nums, PREAMBLE))
    for num in nums:
        if not has_sum(window, num):
            print(num)
            return
        window.pop(0)
        window.append(num)

    print("failed?")
