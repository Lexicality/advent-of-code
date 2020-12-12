import itertools
from typing import Dict, Iterable, List


class Adapter:
    jolts: int
    compatible: List[int]
    compatible2: List["Adapter"]

    def __init__(self, jolts: int, adapters: List[int]):
        self.jolts = jolts
        self.compatible = list(
            itertools.takewhile(
                lambda adapter: adapter <= jolts + 3,
                (adapter for adapter in adapters if adapter > jolts),
            )
        )

    @property
    def permutations(self):
        return len(self.compatible)

    def my_trees(self):
        me_tree = [self.jolts]

        if self.permutations == 0:
            return [me_tree]

        return [
            me_tree + tree
            for tree in itertools.chain.from_iterable(
                adapter.my_trees() for adapter in self.compatible2
            )
        ]

    def __str__(self):
        compatible = ", ".join(str(adapter) for adapter in self.compatible)
        return f"Adapter {self.jolts}" f" - {self.permutations}: " f"[{compatible}]"

    def __repr__(self):
        return str(self)


def main(data: Iterable[str]):
    joltrain = sorted(int(line) for line in data)

    joltrain.insert(0, 0)
    LAST = joltrain[-1] + 3
    joltrain.append(LAST)

    adapters: Dict[int, Adapter] = {
        jolts: Adapter(jolts, joltrain) for jolts in joltrain
    }
    for adapter in adapters.values():
        adapter.compatible2 = [adapters[jolts] for jolts in adapter.compatible]

    # from pprint import pprint

    # pprint(adapters)
    # pprint(adapters[0].my_trees())
    print(len(adapters[0].my_trees()))
