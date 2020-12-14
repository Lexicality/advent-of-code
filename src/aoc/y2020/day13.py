from typing import Iterable, List, Literal, Union


def bus_party(timestamp: int, busses: List[Union[int, Literal["x"]]]) -> bool:
    for i in range(len(busses)):
        timestamp += 1
        bus = busses[i]
        if bus == "x":
            continue
        elif (timestamp % bus) != 0:
            return False

    return True


def main(data: Iterable[str]):
    _, schedule = data
    busses = [int(bus) if bus != "x" else "x" for bus in schedule.split(",")]

    first_bus = busses.pop(0)
    assert isinstance(first_bus, int)

    timestamp = 0
    while True:
        timestamp += first_bus
        if bus_party(timestamp, busses):
            print(timestamp)
            return
