from collections import defaultdict
from typing import DefaultDict, Iterable


def main(data: Iterable[str]) -> None:
    cmds = [
        (cmd, int(amt))
        for cmd, amt in (
            #
            line.split(" ", maxsplit=1)
            for line in data
        )
    ]

    x = 0
    y = 0
    z = 0

    for cmd, amt in cmds:
        if cmd == "forward":
            x += amt
        elif cmd == "up":
            z -= amt
            if z < 0:
                raise ValueError("Ship tried to take off??")
        elif cmd == "down":
            z += amt
        else:
            raise NotImplementedError(f"Unknown command '{cmd}'!")
        print(cmd, amt, f"x: {x:,}, y: {y:,}, z: {z:,}")

    print(x * z)
