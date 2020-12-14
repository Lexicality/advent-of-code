from typing import Dict, Iterable


def main(data: Iterable[str]):
    timestamp, schedule = data
    timestamp = int(timestamp)
    busses = [int(bus) for bus in schedule.split(",") if bus != "x"]

    bus_leavers: Dict[int, int] = {}

    for bus in busses:
        latest = bus
        while latest < timestamp:
            latest += bus
        bus_leavers[bus] = latest

    soonest = sorted(bus_leavers.items(), key=lambda a: a[1])[0]
    print(soonest)
    print(soonest[0] * (soonest[1] - timestamp))
