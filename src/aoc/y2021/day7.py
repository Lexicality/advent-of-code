from typing import Iterable

import pandas as pd


def main(data: Iterable[str]) -> None:
    crabpos = pd.Series([int(num) for num in next(data).split(",")])

    median = crabpos.median()

    fuel = (crabpos - median).abs()

    print(fuel.sum())
