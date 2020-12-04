import itertools
from typing import Iterable, List

from .utils import read_file

DAY = "4"
FIELDS = {
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
    # "cid",
}


def pred(line: str) -> bool:
    return line != ""


def passport(iter: Iterable[str]) -> List[str]:
    pp = itertools.takewhile(pred, iter)
    return " ".join(pp).split(" ")


def passports() -> Iterable[List[str]]:
    data = read_file(DAY)
    while pp := passport(data):
        # can't work out how this is getting in
        if pp == [""]:
            break
        yield pp


def validate(passport: List[str]) -> bool:
    fields = set(field.split(":", maxsplit=1)[0] for field in passport)
    for field in FIELDS:
        if field not in fields:
            return False

    return True


def main():
    ret = 0
    for pp in passports():
        if validate(pp):
            ret += 1

    print(ret)


main()
