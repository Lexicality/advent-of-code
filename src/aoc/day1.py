from typing import Optional

from .utils import read_file

DAY = 1


def main():
    nums = [int(line) for line in read_file(DAY)]
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


if __name__ == "__main__":
    main()
