import itertools
import re
from typing import Iterable, List, Set, Tuple


class Step:
    name: str
    dependencies: Set["Step"]
    dependants: Set["Step"]
    done = False

    def __init__(self, name: str):
        self.name = name
        self.dependencies = set()
        self.dependants = set()

    def __repr__(self) -> str:
        deps = ",".join(step.name for step in self.dependencies)
        return f"Step {self.name} [{deps}]"


STEP_RE = re.compile(r"Step (.) must be finished before step (.) can begin.")


def extract_steps(data: Iterable[str]) -> Iterable[Tuple[str, str]]:
    for step in data:
        match = STEP_RE.match(step)
        assert match is not None
        yield match[1], match[2]


def main(data: Iterable[str]):
    raw_steps = sorted(extract_steps(data), key=lambda s: s[0] + s[1])

    # all steps is sorted because py3.8
    all_steps = {
        step: Step(step)
        for step in sorted(set(itertools.chain.from_iterable(raw_steps)))
    }

    print(raw_steps)

    for dependency, step_name in raw_steps:
        dep_step = all_steps[dependency]
        step = all_steps[step_name]
        step.dependencies.add(dep_step)
        dep_step.dependants.add(step)

    print(all_steps)

    order = ""

    while len(all_steps) > 0:
        print()
        print(all_steps)
        for name, step in all_steps.items():
            if len(step.dependencies) > 0:
                continue

            print("Ye", name)
            for dep in step.dependants:
                dep.dependencies.remove(step)
            order += name
            del all_steps[name]
            break

    print(order)
