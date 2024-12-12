# Copyright (c) 2024 Lexi Robinson
#
# Licensed under the EUPL, Version 1.2
#
# You may not use this work except in compliance with the Licence.
# You should have received a copy of the Licence along with this work. If not, see:
# <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
# See the Licence for the specific language governing permissions and limitations under the Licence.

from typing import Iterator

import pandas as pd

SIMULATION_DAYS = 256


def main(data: Iterator[str]) -> None:
    fische = pd.DataFrame([[i, 0] for i in range(9)], columns=("gen", "count"))

    for gen in next(data).split(","):
        fische.loc[fische["gen"] == int(gen), "count"] += 1

    # print(fische.T.to_string(header=False))

    # Love exponential growth
    for i in range(SIMULATION_DAYS):
        zeros = fische.loc[fische["gen"] == 0]
        if not zeros.empty:
            num_zeros = zeros["count"].iloc[0]
            fische.loc[fische["gen"] == 0, "gen"] = 9
            fische.loc[fische["gen"] == 7, "count"] += num_zeros

        fische["gen"] -= 1
        print(i, end=" ", flush=True)
        # print(i)
        # print(fische.T.to_string(header=False))

    print()
    print(fische["count"].sum())
