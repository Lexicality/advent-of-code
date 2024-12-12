# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

from __future__ import annotations

from typing import Iterable, List


def main(data: Iterable[str]) -> None:
    elves: List[int] = []
    running_elf = 0
    for line in data:
        line = line.strip()
        if not line:
            elves.append(running_elf)
            running_elf = 0
            continue
        running_elf += int(line)

    if running_elf:
        elves.append(running_elf)

    elves.sort(reverse=True)

    print(sum(elves[:3]))
