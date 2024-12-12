# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

from typing import Iterable, Optional


def main(data: Iterable[str]):
    nums = [int(line) for line in data]
    num_set = set(nums)

    def search(num: int) -> Optional[int]:
        target = 2020 - num
        if target in num_set:
            return target
        return None

    while nums:
        num1 = nums.pop()
        for num2 in nums:
            num3 = search(num1 + num2)
            if num3:
                print(num1 * num2 * num3)
                return
        # target = 2020 - num
        # if target in num_set:
        #     print(num * target)
        #     return

    print("None found?")
