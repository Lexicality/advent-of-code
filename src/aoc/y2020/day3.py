import math
from typing import Iterable, List

RIGHT_BIAS = 7
TREE = "#"
SLOPES = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]


def generate_map(data: Iterable[str]):
    map = list(data)
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


def main(data: Iterable[str]):
    map = generate_map(data)
    trees = 1
    for slope in SLOPES:
        trees *= check_slope(map, *slope)
    print(trees)
