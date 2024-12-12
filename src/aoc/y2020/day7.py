# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

# dark orange bags contain 3 bright white bags, 4 muted yellow bags.
import re
from typing import Dict, Iterable, List, NamedTuple, Set, Tuple


class Rule(NamedTuple):
    count: int
    target: "Bag"


class Bag:
    name: str
    can_contain: Set[Rule]
    contained_by: Set["Bag"]

    def __init__(self, name: str):
        self.name = name
        self.can_contain = set()
        self.contained_by = set()

    def __repr__(self) -> str:
        return f"a {self.name} bag"


BAGPARSE_RE = re.compile(r"^(.+) bags contain (.+).$")
BAGBITS_RE = re.compile(r"(\d+) (.+?) bag")


def extract_bags(data: Iterable[str]) -> Iterable[Tuple[str, List[Tuple[int, str]]]]:
    for line in data:
        match = BAGPARSE_RE.match(line)
        assert match is not None, f"{line} didn't parse!"
        name = match[1]
        rules = [(int(count), name) for count, name in BAGBITS_RE.findall(match[2])]
        yield name, rules


def main(data: Iterable[str]):
    raw_rules = sorted(extract_bags(data), key=lambda r: r[0])

    bags: Dict[str, Bag] = {}

    for name, rules in raw_rules:
        bags[name] = Bag(name)

    for name, rules in raw_rules:
        bag = bags[name]
        for count, target_name in rules:
            target = bags[target_name]
            rule = Rule(count=count, target=target)
            bag.can_contain.add(rule)
            target.contained_by.add(bag)

    shiny_gold = bags["shiny gold"]

    total_bags = 0

    to_check = [rule for rule in shiny_gold.can_contain]
    while len(to_check) > 0:
        rule = to_check.pop(0)
        assert rule.target is not shiny_gold

        total_bags += rule.count
        for _ in range(rule.count):
            to_check.extend(rule.target.can_contain)

    print(total_bags)
