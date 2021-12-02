from typing import Iterable


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
    aim = 0

    for cmd, amt in cmds:
        if cmd == "forward":
            x += amt
            z += aim * amt
            if z < 0:
                raise ValueError("Ship tried to take off??")
        elif cmd == "up":
            aim -= amt
        elif cmd == "down":
            aim += amt
        else:
            raise NotImplementedError(f"Unknown command '{cmd}'!")
        print(cmd, amt, f"x: {x:,}, y: {y:,}, z: {z:,} aim: {aim:,}")

    print(x * z)
