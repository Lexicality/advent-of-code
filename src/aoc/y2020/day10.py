from collections import defaultdict
from typing import DefaultDict, Iterable


def main(data: Iterable[str]):
    joltrain = sorted(int(line) for line in data)
    diffs: DefaultDict[int] = defaultdict(lambda: 0)

    joltrain.insert(0, 0)

    for i in range(len(joltrain) - 1):
        diff = abs(joltrain[i] - joltrain[i + 1])
        diffs[diff] += 1

    diffs[3] += 1

    print(diffs)
    print(diffs[1] * diffs[3])
