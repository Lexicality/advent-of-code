import math

from .utils import read_file

DAY = "3"
RIGHT_BIAS = 3
TREE = "#"


def generate_map():
    map = list(read_file(DAY))
    height = len(map)
    width = len(map[0])

    num_per = width // RIGHT_BIAS

    mul = math.ceil(height / num_per)
    return [row * mul for row in map]


def main():
    map = generate_map()
    pos = 0
    trees = 0
    for row in map:
        if row[pos] == TREE:
            trees += 1
        pos += RIGHT_BIAS
    print(trees)


if __name__ == "__main__":
    main()
