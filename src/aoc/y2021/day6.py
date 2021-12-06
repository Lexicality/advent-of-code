from typing import Iterable

import pandas as pd

SIMULATION_DAYS = 256


def main(data: Iterable[str]) -> None:
    fische = pd.Series([int(num) for num in next(data).split(",")])

    # Love exponential growth
    for i in range(SIMULATION_DAYS):
        zeros = (fische == 0).sum()
        if zeros > 0:
            fische.replace(0, 7, inplace=True)
            fische = fische.append(pd.Series([9 for _ in range(zeros)]))
        fische -= 1
        print(i, end=" ", flush=True)
        # print(fische.to_frame().T.to_string(header=False, index=False))

    print(len(fische))
