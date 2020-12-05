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
    all_passes = list(process_pass(bp) for bp in read_file(DAY))

    all_passes.sort()

    for i in range(len(all_passes)):
        this = all_passes[i]
        next = all_passes[i + 1]
        if next > this + 1:
            print(this + 1)
            return

    print("how did you get here without raising?")

    print(all_passes)
