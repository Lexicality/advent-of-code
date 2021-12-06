import itertools
from typing import Iterator, List, Optional

import pandas as pd


def _read_board(data: Iterator[str]) -> Optional[pd.DataFrame]:
    try:
        blank_line = next(data)
        assert blank_line == ""
    except StopIteration:
        return None

    return pd.DataFrame(
        [
            [int(num) for num in line.split()]
            for line in
            # Boards are always 5 lines deep
            itertools.islice(data, 5)
        ],
        dtype="Int64",
    )


def _check_bingo(df: pd.DataFrame) -> bool:
    cols = True
    for i in range(5):
        row = df.iloc[i]

        if not row.hasnans:
            cols = False
            continue

        if all(row.isna()):
            return True

    if not cols:
        return False

    for col_id in df.columns:
        col = df[col_id]

        if all(col.isna()):
            return True

    return False


def main(data: Iterator[str]) -> None:
    draws = [int(num) for num in next(data).split(",")]
    print(draws)

    boards = []
    while True:
        board = _read_board(data)
        if board is None:
            break
        boards.append(board)

    for board in boards:
        print()
        print(board)

    winning_board: pd.DataFrame = None
    for num in draws:
        print("Drawing #", num)
        for board in boards:
            board.replace(num, pd.NA, inplace=True)
            if _check_bingo(board):
                print("BINGO!")
                winning_board = board

        if winning_board is not None:
            break

        # i = input()
        # if i == "q":
        #     return
        # elif i == "d":
        #     import pdb
        #
        #     pdb.set_trace()

    print(winning_board)
    score = winning_board.sum().sum()
    print(score)
    print(score * num)
