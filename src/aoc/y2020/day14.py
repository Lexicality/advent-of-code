import re
from typing import Dict, Iterable

MEM_RE = re.compile(r"mem\[(\d+)\]")


def maskify(input: int, mask: str) -> int:
    binput = list(f"{input:036b}")

    for i in range(36):
        mask_val = mask[i]
        if mask_val == "1" or mask_val == "0":
            binput[i] = mask_val

    return int("".join(binput), 2)


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
        memory[mem_addr] = maskify(new_value, mask)

    print(memory)
    print(sum(memory.values()))
