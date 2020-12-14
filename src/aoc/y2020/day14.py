import re
from typing import Dict, Iterable, NamedTuple


class BitMasks(NamedTuple):
    and_mask: int
    or_mask: int


def mit_basks(input: str) -> BitMasks:
    return BitMasks(
        and_mask=int(input.replace("X", "1"), 2),
        or_mask=int(input.replace("X", "0"), 2),
    )


MEM_RE = re.compile(r"mem\[(\d+)\]")


def main(data: Iterable[str]):
    masks = BitMasks(and_mask=(2 ** 36) - 1, or_mask=0)
    memory: Dict[str, int] = {}
    for line in data:
        print(f"Current mask: {masks.and_mask:036b}/{masks.or_mask:036b}")
        action, value = line.split(" = ", maxsplit=1)
        if action == "mask":
            print("Setting mask to", value)
            masks = mit_basks(value)
            continue

        m = MEM_RE.match(action)
        assert m
        intval = int(value)
        memory[m[1]] = (intval & masks.and_mask) | masks.or_mask

    print(memory)
    print(sum(memory.values()))
