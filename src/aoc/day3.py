import math
from typing import List

from .utils import read_file

DAY = "3"
RIGHT_BIAS = 7
TREE = "#"
SLOPES = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]


def generate_map():
    map = list(read_file(DAY))
    height = len(map)
    width = len(map[0])

    num_per = width // RIGHT_BIAS

    mul = math.ceil(height / num_per)
    return [row * mul for row in map]


def check_slope(
    map: List[str],
    right: int,
    down: int,
) -> int:
    xpos = 0
    ypos = 0
    trees = 0
    while xpos < len(map):
        row = map[xpos]
        if row[ypos] == TREE:
            trees += 1
        xpos += down
        ypos += right
    return trees


def main():
    map = generate_map()
    trees = 1
    for slope in SLOPES:
        trees *= check_slope(map, *slope)
    print(trees)


if __name__ == "__main__":
    main()
