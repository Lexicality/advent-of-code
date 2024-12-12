# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

import itertools
import re
from typing import Dict, Iterable

MEM_RE = re.compile(r"mem\[(\d+)\]")


def maskify(input: int, mask: str) -> Iterable[int]:
    binput = list(f"{input:036b}")

    x_count = 0
    for i in range(36):
        mask_val = mask[i]
        if mask_val == "1":
            binput[i] = mask_val
        elif mask_val == "X":
            binput[i] = "{}"
            x_count += 1

    binput_str = "".join(binput)

    for iteration in set(itertools.combinations((1, 0) * x_count, x_count)):
        yield int(binput_str.format(*iteration), 2)


def main(data: Iterable[str]):
    mask = "0" * 36
    memory: Dict[int, int] = {}
    for line in data:
        print(f"Current mask: {mask}")
        action, value = line.split(" = ", maxsplit=1)
        if action == "mask":
            print("Setting mask to", value)
            mask = value
            continue

        m = MEM_RE.match(action)
        assert m
        mem_addr = int(m[1])
        new_value = int(value)
        for real_mem_addr in maskify(mem_addr, mask):
            memory[real_mem_addr] = new_value

    print(memory)
    print(sum(memory.values()))
