from abc import ABC
from typing import TYPE_CHECKING, Iterable, Literal, Optional, Union

from more_itertools import peekable

OPERATORS = {"(", "*", "+", ")"}
Operator = Union[Literal["*"], Literal["+"]]
Expr = Union[str, int]
if TYPE_CHECKING:
    Exprs = peekable[Expr]
else:
    Exprs = peekable


class Step(ABC):
    value: int


class Value(Step):
    def __init__(self, value: int) -> None:
        self.value = value

    def __repr__(self) -> str:
        return f"{self.value}"


class CompilationUnit(Step):
    left: Step
    right: Step
    opr: Operator

    def __init__(self, left: Step, right: Step, operator: Operator) -> None:
        self.left = left
        self.right = right
        self.opr = operator

    @property
    def value(self) -> int:
        lv = self.left.value
        rv = self.right.value
        if self.opr == "+":
            return lv + rv
        elif self.opr == "*":
            return lv * rv
        assert False, "??"

    def __repr__(self) -> str:
        return f"({self.left} {self.opr} {self.right})"


def parse_single(exprs: Exprs) -> Step:
    expr: Expr = next(exprs)
    if isinstance(expr, int):
        return Value(expr)
    assert expr == "("
    step = None
    while exprs.peek() != ")":
        step = compilate(step, exprs)
    # Consume the ")"
    next(exprs)
    return step


def compilate(left: Optional[Step], exprs: Exprs) -> Step:
    if left is None:
        left = parse_single(exprs)

    opr: Operator = next(exprs)
    assert opr == "+" or opr == "*", f"Expected operator, got {opr}"

    right = parse_single(exprs)

    return CompilationUnit(left, right, opr)


def expressionify(line: str) -> Exprs:
    for char in list(line):
        if char == " ":
            continue
        elif char in OPERATORS:
            yield char
        else:
            # All the numbers in the input are single digit so we can cheat like crazy
            yield int(char)


def do_expr(line: str) -> int:
    line = "(" + line + ")"
    exprs = peekable(expressionify(line))
    unit = parse_single(exprs)
    print("Input:", line)
    print("Parsed:", unit)
    print("Output:", unit.value)
    return unit.value


def main(input: Iterable[str]):
    totes = 0
    for line in input:
        totes += do_expr(line)
    # ret = do_expr("1 + (2 * 3) + (4 * (5 + 6))")
    # if ret != 51:
    #     print("HECK")
    print(totes)
