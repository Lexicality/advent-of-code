from typing import Iterable


def main(data: Iterable[str]) -> None:
    nums = [int(line) for line in data]

    sinking = 0
    last = nums[0]

    for num in nums:
        if num > last:
            sinking += 1
        last = num

    print(sinking)
