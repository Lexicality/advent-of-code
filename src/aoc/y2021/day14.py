from collections import defaultdict
from pprint import pprint
from typing import Dict, Iterator, Tuple

InsertionRules = Dict[str, Tuple[str, Tuple[str, str]]]
Polymer = Dict[str, int]

NUM_STEPS = 40


def main(data: Iterator[str]) -> None:
    counts: Dict[str, int] = defaultdict(int)

    polymer: Polymer = defaultdict(int)
    polymer_str = next(data)

    for char in polymer_str:
        counts[char] += 1

    for i in range(len(polymer_str) - 1):
        polymer[polymer_str[i] + polymer_str[i + 1]] += 1

    pprint(counts)
    pprint(polymer)

    next(data)  # blank line

    rules: InsertionRules = {}

    for rulestr in data:
        pair, result = rulestr.split(" -> ")
        a, b = pair
        rules[pair] = (result, (a + result, result + b))

    # pprint(rules)

    for i in range(NUM_STEPS):
        new_polymer: Polymer = defaultdict(int)

        for pair, count in polymer.items():
            letter, (p1, p2) = rules[pair]
            counts[letter] += count
            new_polymer[p1] += count
            new_polymer[p2] += count

        print("Step", i + 1)
        # pprint(counts)
        # pprint(new_polymer)
        # print()
        polymer = new_polymer

    pprint(counts)
    pprint(polymer)

    print(max(counts.values()) - min(counts.values()))
