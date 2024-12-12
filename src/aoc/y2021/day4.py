# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

import itertools
from typing import Iterator, Optional

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


def _boardify(num: int, board: pd.DataFrame) -> bool:
    board.replace(num, pd.NA, inplace=True)
    return _check_bingo(board)


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

    winning_board: pd.DataFrame | None = None
    num = -1
    for num in draws:
        print("Drawing #", num)
        # crimes
        if len(boards) > 1:
            boards = [
                board
                for board in boards
                # crimes crimes crimes
                if not _boardify(num, board)
            ]
        else:
            board = boards[0]
            if _boardify(num, board):
                winning_board = board
                break

        # i = input()
        # if i == "q":
        #     return
        # elif i == "d":
        #     import pdb
        #
        #     pdb.set_trace()

    print(winning_board)
    assert winning_board is not None
    score = winning_board.sum().sum()
    print(score)
    print(score * num)
