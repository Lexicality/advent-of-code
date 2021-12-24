from __future__ import annotations

from typing import Iterator, List, Type

from bitstring import BitStream, ConstBitStream


class Packet:
    version: int
    type: int

    def __init__(self, version: int, type: int) -> None:
        self.version = version
        self.type = type

    def consume(self, data: ConstBitStream) -> None:
        raise NotImplementedError("Base Class!")

    def __str__(self) -> str:
        return f"Packet {self.type} version {self.version}"

    def part1(self) -> int:
        return self.version


class PacketLiteral(Packet):
    value: int = -1

    def consume(self, data: ConstBitStream) -> None:
        value = BitStream()
        keep_reading = True
        while keep_reading:
            keep_reading = data.read("bool")
            value += data.read(4)
        self.value = value.uint

    def __str__(self) -> str:
        return f"Literal packet version {self.version} with value: {self.value}"


LINE_TYPE_LENGTH = False
LINE_TYPE_COUNT = True


class PacketOperator(Packet):
    contents: List[Packet]

    def __init__(self, version: int, type: int) -> None:
        super().__init__(version, type)
        self.contents = []

    def _consume_count(self, data: ConstBitStream) -> None:
        num_packets = data.read("uint:11")
        for _ in range(num_packets):
            self.contents.append(_parse_packet(data))

    def _consume_length(self, data: ConstBitStream) -> None:
        packet_length = data.read("uint:15")
        end_pos = data.pos + packet_length
        while data.pos < end_pos:
            self.contents.append(_parse_packet(data))
        if data.pos > end_pos:
            raise RuntimeError("Consumed too much data!")

    def consume(self, data: ConstBitStream) -> None:
        line_type = data.read("bool")
        if line_type == LINE_TYPE_COUNT:
            self._consume_count(data)
        else:
            self._consume_length(data)

    def __str__(self) -> str:
        contents = ", ".join(str(packet) for packet in self.contents)
        return (
            f"Operator {self.type} packet version {self.version}"
            f" with children: {contents}"
        )

    def part1(self) -> int:
        return super().part1() + sum(packet.part1() for packet in self.contents)


def _get_packet_class(type: int) -> Type[Packet]:
    if type == 4:
        return PacketLiteral
    return PacketOperator


def _parse_packet(data: ConstBitStream) -> Packet:
    version, type = data.readlist("3, 3")

    PacketClass = _get_packet_class(type)
    packet = PacketClass(version, type)
    packet.consume(data)
    return packet


def main(data: Iterator[str]):
    for encoded in data:
        print(encoded)
        bits = ConstBitStream(hex=encoded)
        packet = _parse_packet(bits)
        print(packet)
        print(packet.part1())
