from typing import Iterable

import pandas as pd


def main(data: Iterable[str]) -> None:
    df = pd.DataFrame([list(line) for line in data])

    df_mode = df.mode(0)

    gamma = int(df_mode.agg("".join, axis=1)[0], base=2)
    print("gamma", gamma, f"{gamma:b}")

    mask = (2 ** len(df.columns)) - 1

    epsilon = gamma ^ mask
    print("epsilon", epsilon, f"{epsilon:b}")

    print(gamma * epsilon)
