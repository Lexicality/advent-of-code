from typing import Iterator

import pandas as pd


def fuelify(n: int) -> float:
    return (n * (n + 1)) / 2


def main(data: Iterator[str]) -> None:
    crabpos = pd.Series([int(num) for num in next(data).split(",")])

    # brute force because while I got almost the correct mean, I apparently
    # didn't which is extremely upsetting
    min_fuel = 91257681  # my incorrect answer
    for i in range(crabpos.min(), crabpos.max() + 1):
        fuel = int((crabpos - i).abs().apply(fuelify).sum())
        min_fuel = min(fuel, min_fuel)
        if fuel == 91257582:
            print(i)
    print(crabpos.mean())

    print(min_fuel)
