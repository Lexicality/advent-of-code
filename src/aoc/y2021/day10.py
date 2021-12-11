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
COMPLETION_SCORES = {
    ")": 1,
    "]": 2,
    "}": 3,
    ">": 4,
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

    def parse(self, line: str) -> str:
        self.closing_stack.clear()
        for token in line:
            self._step(token)

        return "".join(reversed(self.closing_stack))


def main(data: Iterable[str]) -> None:
    parser = Parser()
    line_scores: List[int] = []

    for line in data:
        try:
            remaining = parser.parse(line)
        except ParserError:
            continue

        line_score = 0
        for char in remaining:
            # print(char, line_score, end=" ")
            line_score *= 5
            # print(line_score, end=" ")
            line_score += COMPLETION_SCORES[char]
            # print(line_score)
        line_scores.append(line_score)

        print(line, line_score)

    sorted_scores = sorted(line_scores)
    center = int(len(line_scores) / 2)

    print(sorted_scores[center])
