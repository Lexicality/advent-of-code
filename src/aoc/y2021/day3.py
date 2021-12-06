from typing import Iterable

import pandas as pd


def _extracto(df: pd.DataFrame, mode: bool) -> int:
    df = df.copy()

    for col in df.columns:
        counts = df[col].value_counts()

        n_1 = counts["1"]
        n_0 = counts["0"]

        print()
        print(df)
        print(f"Col #{col} 1s: {n_1} 0s: {n_0}")

        if mode:
            target = "1"
            if n_1 > n_0:
                target = "1"
            elif n_1 < n_0:
                target = "0"
        else:
            target = "0"
            if n_1 > n_0:
                target = "0"
            elif n_1 < n_0:
                target = "1"

        df = df.loc[df[col] == target]

        print(df)

        if len(df.index) == 1:
            break

    print("success!")
    print(df)

    splat = df.agg("".join, axis=1).iloc[0]
    print(splat)

    return int(splat, base=2)


def main(data: Iterable[str]) -> None:
    df = pd.DataFrame([list(line) for line in data])

    print("o2")
    o2 = _extracto(df, True)
    print("co2")
    co2 = _extracto(df, False)

    print(f"o2 {o2} {o2:b}")
    print(f"co2 {co2} {co2:b}")

    print(o2 * co2)
