# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

from typing import Iterable, List, NamedTuple, Set, Tuple


class Opcode(NamedTuple):
    opcode: str
    value: int


def parse_opcode(input: str) -> Opcode:
    opcode, value = input.split(" ", maxsplit=1)
    return Opcode(opcode, int(value))


def compute(program: List[Opcode]) -> Tuple[bool, int]:
    visited_lines: Set[int] = set()

    pc = 0
    acc = 0

    while pc < len(program):
        if pc in visited_lines:
            return False, acc

        visited_lines.add(pc)

        opcode, value = program[pc]
        if opcode == "nop":
            pc += 1
        elif opcode == "acc":
            acc += value
            pc += 1
        elif opcode == "jmp":
            pc += value
            assert pc >= 0, "invalid jump backwards!"
        else:
            assert False, "invalid opcode!"

    return True, acc


def main(data: Iterable[str]):
    program = [parse_opcode(line) for line in data]

    for i in range(len(program)):
        opcode, value = program[i]
        if opcode == "acc":
            continue
        elif opcode == "jmp":
            opcode = "nop"
        elif opcode == "nop":
            opcode = "jmp"
        else:
            assert False, "invalid opcode!"

        new_program = program.copy()
        new_program[i] = Opcode(opcode, value)

        finished, acc = compute(new_program)
        if finished:
            print("success!")
            print(acc)
            return

    assert False, "Unsolvable?"
