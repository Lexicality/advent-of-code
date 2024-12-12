# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

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

    _perms: Optional[int] = None

    def get_total_permutations(self) -> int:
        if self._perms is None:
            self._perms = self._get_perms()
        return self._perms

    def _get_perms(self) -> int:
        if self.permutations == 0:
            return 1

        return sum(adapter.get_total_permutations() for adapter in self.compatible2)

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
        print("Doing tree for", adapter, ":", adapter.get_total_permutations())
    print("done!")

    # from pprint import pprint

    # pprint(adapters)
    # pprint(adapters[0].my_trees())
    print(adapters[0].get_total_permutations())
