import itertools
from typing import Iterable, List


def permutaturize(all_nums: List[int], current: List[int]) -> List[List[int]]:
    latest = current[-1]
    assert latest != all_nums[-1]

    i = all_nums.index(latest)

    ret = []
    for optn in itertools.takewhile(lambda a: a <= latest + 3, all_nums[i + 1 :]):
        permutation = current.copy()
        permutation.append(optn)

        ret.append(permutation)

    return ret


def main(data: Iterable[str]):
    joltrain = sorted(int(line) for line in data)

    joltrain.insert(0, 0)
    LAST = joltrain[-1] + 3
    joltrain.append(LAST)

    permutations: List[List[int]] = [[0]]
    complete: List[List[int]] = []

    while len(permutations) > 0:
        current = permutations.pop(0)
        results = permutaturize(joltrain, current)
        for result in results:
            if result[-1] == LAST:
                complete.append(result)
            else:
                permutations.append(result)

    print(len(complete))
