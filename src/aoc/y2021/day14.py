from typing import Dict, Iterable, Iterator, List, Tuple

import numpy as np

Pair = Tuple[int, int]
InsertionRules = Dict[Pair, int]

NUM_STEPS = 10


# strings are expensive and this is exponential
def destrify(string: str) -> Iterable[int]:
    for char in string:
        yield ord(char)


def restrify(polymer: Iterable[int]) -> str:
    return "".join(chr(i) for i in polymer)


def main(data: Iterator[str]) -> None:
    polymer = np.fromiter(destrify(next(data)), np.int8)

    rules: InsertionRules = {}

    next(data)  # blank line
    for rulestr in data:
        pair, result = rulestr.split(" -> ")
        rules[tuple(destrify(pair))] = ord(result)

    print(restrify(polymer))

    for i in range(NUM_STEPS):
        indexs: List[int] = []
        values: List[int] = []
        for j in range(len(polymer) - 1):
            k = j + 1
            res = rules.get((polymer[j], polymer[k]))
            if res:
                indexs.append(k)
                values.append(res)
        polymer = np.insert(polymer, indexs, values)

        # print(f"After step {i+ 1}:", restrify(polymer))
        print(f"After step {i+ 1}:", len(polymer))

    _, counts = np.unique(polymer, return_counts=True)

    print(max(counts) - min(counts))
