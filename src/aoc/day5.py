from .utils import read_file

DAY = "5"

ROWS = list(range(128))
COLS = list(range(8))


def process_pass(bpass: str) -> int:
    my_rows = ROWS
    my_cols = COLS
    for char in bpass:
        if char == "F":
            my_rows = my_rows[: len(my_rows) // 2]
        elif char == "B":
            my_rows = my_rows[len(my_rows) // 2 :]
        elif char == "L":
            my_cols = my_cols[: len(my_cols) // 2]
        elif char == "R":
            my_cols = my_cols[len(my_cols) // 2 :]

    print(bpass, my_rows, my_cols)

    return my_rows[0] * 8 + my_cols[0]


def main():
    highest = -1

    for bpass in read_file(DAY):
        highest = max(highest, process_pass(bpass))

    print(highest)


main()
