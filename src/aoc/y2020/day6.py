# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

import itertools
from collections import defaultdict
from typing import Iterable, List


def pred(line: str) -> bool:
    return line != ""


PersonAnswer = List[str]
GroupAnswer = List[PersonAnswer]


def extract_group(data: Iterable[str]) -> GroupAnswer:
    group = itertools.takewhile(pred, data)
    return [list(person) for person in group]


def groups(data: Iterable[str]) -> Iterable[GroupAnswer]:
    while group := extract_group(data):
        yield group


def get_qs(group: GroupAnswer) -> List[str]:
    num_group = len(group)
    if num_group == 1:
        return group[0]

    qs = defaultdict(lambda: 0)
    for per in group:
        for q in per:
            qs[q] += 1

    return [q for q, c in qs.items() if c == num_group]


def main(data: Iterable[str]):
    total = sum(len(get_qs(group)) for group in groups(data))
    print(total)
