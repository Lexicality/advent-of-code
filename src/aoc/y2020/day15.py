from collections import defaultdict
from typing import DefaultDict, Iterable, List, Optional


class Listicle(List[Optional[int]]):
    def __init__(self, *args):
        super().__init__([None, None])

    def be_spoken(self, round: int) -> None:
        self.pop(0)
        self.append(round)

    def get_value(self) -> int:
        if self[0] is None:
            return 0
        return self[1] - self[0]  # type: ignore

    def get_logging(self) -> str:
        if self[0] is None:
            return "spoken for the first time"
        return f"spoken on turns {self[0]:04} and {self[1]:04}"


def run_game(nums: List[int]) -> int:
    last_heard: DefaultDict[int, Listicle] = defaultdict(Listicle)

    last_spoken: int = -1
    for i in range(len(nums)):
        last_spoken = nums[i]
        last_heard[last_spoken].be_spoken(i + 1)

    print(last_heard)

    for round in range(len(nums) + 1, 30000000 + 1):
        # for round in range(len(nums) + 1, 21):
        last = last_heard[last_spoken]
        value = last.get_value()
        # print(
        #     f"Turn {round:04}:",
        #     "Last number was",
        #     last_spoken,
        #     "which was",
        #     last.get_logging(),
        #     "giving us",
        #     value,
        # )

        last_heard[value].be_spoken(round)
        last_spoken = value

    return last_spoken


def main(data: Iterable[str]):
    # Multiple example support
    for line in data:
        nums = [int(i) for i in line.split(",")]
        result = run_game(nums)
        print(line, result)
        # break
