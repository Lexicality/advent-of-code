import itertools
from typing import Dict, Iterable, List, Optional


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

    _tree: Optional[List[int]] = None

    def get_tree(self) -> List[List[int]]:
        if self._tree is None:
            self._tree = self._get_tree()
        return self._tree

    def _get_tree(self):
        if self.permutations == 0:
            return [1]

        return [
            i + 1
            for i in itertools.chain.from_iterable(
                adapter.get_tree() for adapter in self.compatible2
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

    # pretree?
    for jolt in joltrain[::-1]:
        adapter = adapters[jolt]
        print("Doing tree for", adapter, ":", len(adapter.get_tree()))
    print("done!")

    # from pprint import pprint

    # pprint(adapters)
    # pprint(adapters[0].my_trees())
    print(len(adapters[0].get_tree()))
