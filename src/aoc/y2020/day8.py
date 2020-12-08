from typing import Iterable, NamedTuple, Set

class Opcode(NamedTuple):
    opcode: str
    value: int

def parse_opcode(input: str) -> Opcode:
    opcode, value = input.split(" ", maxsplit=1)
    return Opcode(opcode, int(value))


def main(data: Iterable[str]):
    pc = 0
    acc = 0
    program = [parse_opcode(line) for line in data]

    visited_lines: Set[int] = set()

    while pc <= len(program):

        if pc in visited_lines:
            print(acc)
            return
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
            assert pc < len(program), "invalid jump forwards!"
        else:
            assert False, "invalid opcode!"

    assert False, "program halted?"
