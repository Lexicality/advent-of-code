from typing import Iterable, Literal, TypeAlias

BusContent: TypeAlias = int | Literal["x"]


def bus_party(timestamp: int, busses: list[BusContent]) -> bool:
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
    busses: list[BusContent] = [
        int(bus) if bus != "x" else "x" for bus in schedule.split(",")
    ]
    print(len(busses))
    print(busses[busses[0]])  # type: ignore

    first_prime = busses.pop(0)
    assert isinstance(first_prime, int)
    second_prime = busses[first_prime - 1]
    assert isinstance(second_prime, int)
    itr_value = first_prime * second_prime

    timestamp = -first_prime
    while True:
        timestamp += itr_value
        if bus_party(timestamp, busses):
            print(timestamp)
            return
