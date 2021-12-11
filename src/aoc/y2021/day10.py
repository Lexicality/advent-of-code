from typing import Iterable, List

MATCHERS = {
    "[": "]",
    "(": ")",
    "{": "}",
    "<": ">",
}
OPENERS = set(MATCHERS.keys())
CLOSERS = set(MATCHERS.values())

SYNTAX_SCORES = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137,
}


class ParserError(ValueError):
    token: str

    def __init__(self, *args: object, token: str) -> None:
        super().__init__(*args)
        self.token = token


class Parser:
    closing_stack: List[str]

    def __init__(self) -> None:
        self.closing_stack = []

    def _step(self, token: str) -> None:
        if token in OPENERS:
            self.closing_stack.append(MATCHERS[token])
        elif token in CLOSERS:
            try:
                expected = self.closing_stack.pop()
            except IndexError:
                raise ParserError(f"Unexpected closing {token}", token=token)
            if token != expected:
                raise ParserError(
                    f"Expected {expected}, but found {token} instead", token=token
                )
        else:
            raise ParserError(f"Unexpected {token}", token=token)

    def parse(self, line: str) -> None:
        self.closing_stack.clear()
        for token in line:
            self._step(token)


def main(data: Iterable[str]) -> None:
    parser = Parser()
    score = 0

    for line in data:
        try:
            parser.parse(line)
        except ParserError as e:
            print(line, e)
            score += SYNTAX_SCORES[e.token]

    print(score)
