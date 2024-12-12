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
from typing import Callable, Dict, Iterable

YEAR = re.compile(r"^[12]\d{3}$")


def _year(min: int, max: int):
    def year_checker(field: str) -> bool:
        if not YEAR.match(field):
            return False
        year = int(field)
        return min <= year <= max

    return year_checker


def height(field: str) -> bool:
    match = re.match(r"^(\d+)(cm|in)$", field)
    if not match:
        return False
    height = int(match[1])
    units = match[2]
    if units == "cm":
        return 150 <= height <= 193
    else:
        return 59 <= height <= 76


FIELDS: Dict[str, Callable[[str], bool]] = {
    "byr": _year(1920, 2002),
    "iyr": _year(2010, 2020),
    "eyr": _year(2020, 2030),
    "hgt": height,
    "hcl": lambda f: re.match(r"^#[a-f0-9]{6}$", f) is not None,
    "ecl": lambda f: re.match(r"^(amb|blu|brn|gry|grn|hzl|oth)$", f) is not None,
    "pid": lambda f: re.match(r"^\d{9}$", f) is not None,
    # "cid",
}


def pred(line: str) -> bool:
    return line != ""


Passport = Dict[str, str]


def passport(iter: Iterable[str]) -> Passport:
    pp = itertools.takewhile(pred, iter)
    return dict(
        field.split(":", maxsplit=1) for field in " ".join(pp).split(" ") if field
    )


def passports(data: Iterable[str]) -> Iterable[Passport]:
    while pp := passport(data):
        yield pp


def validate(passport: Passport) -> bool:
    for field, func in FIELDS.items():
        if field not in passport:
            return False
        if not func(passport[field]):
            return False

    return True


def main(data: Iterable[str]):
    ret = 0

    for pp in passports(data):
        if validate(pp):
            ret += 1

    print(ret)
