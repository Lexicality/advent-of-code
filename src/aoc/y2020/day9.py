# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

from typing import Iterable, List, Optional

PREAMBLE = 25


def has_sum(window: List[int], target: int) -> bool:
    count = len(window)
    for i in range(count):
        for j in range(i, count):
            if window[i] + window[j] == target:
                return True

    return False


def find_weakness(nums: List[int], target: int) -> Optional[List[int]]:
    count = len(nums)
    running_total = 0
    for i in range(count - 1):
        running_total = nums[i]
        for j in range(i + 1, count):
            running_total += nums[j]
            if running_total > target:
                break
            elif running_total < target:
                continue
            return nums[i : j + 1]

    return None


def main(data: Iterable[str]):
    nums = [int(line) for line in data]

    window = nums[:PREAMBLE]

    target_num = -1

    for num in nums[PREAMBLE:]:
        if not has_sum(window, num):
            target_num = num
            break
        window.pop(0)
        window.append(num)
    else:
        print("failed?")
        return

    weakness = find_weakness(nums, target_num)
    assert weakness is not None, "No weakness?"
    weakness.sort()
    print(weakness[0] + weakness.pop())
