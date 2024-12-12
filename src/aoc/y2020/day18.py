# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

import re
from typing import Iterable

RE_PARENS = re.compile(r"\(([^())]+)\)")
RE_ADD = re.compile(r"(\d+) (\+) (\d+)")
RE_MUL = re.compile(r"(\d+) (\*) (\d+)")


def perform_operation(match: re.Match):
    n1 = int(match[1])
    n2 = int(match[3])
    opr = match[2]
    if opr == "*":
        return str(n1 * n2)
    else:
        return str(n1 + n2)


def process_group(group: str) -> str:
    # print("Processing group", group)

    while "+" in group:
        new_group = RE_ADD.sub(perform_operation, group, count=1)
        assert new_group != group, "didn't work :("
        group = new_group
        # print("ADDITION!", group)

    while "*" in group:
        new_group = RE_MUL.sub(perform_operation, group, count=1)
        assert new_group != group, "didn't work :("
        group = new_group
        # print("Multiplication!", group)

    return group


def do_expr(line: str) -> int:
    print("Input:", line)
    while "(" in line:
        line = RE_PARENS.sub(lambda m: process_group(m[1]), line, count=1)
        # print("Grouped!", line)

    line = process_group(line)
    print("Output:", line)
    return int(line)


def main(input: Iterable[str]):
    # EXAMPLE_VALUES = [10, 231, 51, 46, 1445, 669060, 23340]

    totes = 0
    for line in input:
        ans = do_expr(line)
        # expected = EXAMPLE_VALUES.pop(0)
        # if ans != expected:
        #     print("HECK, expected", expected)
        #     return
        totes += ans
    print(totes)
