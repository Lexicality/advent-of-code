from typing import Iterable


def main(data: Iterable[str]):
    res = 0

    for line in data:
        unique, output = line.split(" | ")
        for digit in output.split(" "):
            num_segs = len(digit)
            if num_segs == 2 or num_segs == 3 or num_segs == 4 or num_segs == 7:
                res += 1

    print(res)
