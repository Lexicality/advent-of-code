import itertools
import re
from typing import Iterable, Optional, Set, Tuple

NUM_WORKERS = 5
JOB_TIME = 60 - ord("A") + 1


class Worker:
    name: str
    job: Optional["Step"] = None

    def __init__(self, name: str):
        self.name = name

    def __repr__(self) -> str:
        rep = f"Worker #{self.name} "
        if self.job:
            rep += f"Working on {self.job.name}"
        else:
            rep += "(idle!)"
        return rep


class Step:
    name: str
    dependencies: Set["Step"]
    dependants: Set["Step"]
    timeLeft: int
    worker: Optional[Worker] = None

    def __init__(self, name: str):
        self.name = name
        self.dependencies = set()
        self.dependants = set()
        self.timeLeft = JOB_TIME + ord(name)
        print("step", name, "takes", self.timeLeft)

    def __repr__(self) -> str:
        deps = ",".join(step.name for step in self.dependencies)
        rep = f"Step {self.name} [{deps}]"
        if self.worker:
            rep += f" (Being worked on by #{self.worker.name}"
            rep += f", {self.timeLeft:02} seconds left)"
        return rep


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

    for dependency, step_name in raw_steps:
        dep_step = all_steps[dependency]
        step = all_steps[step_name]
        step.dependencies.add(dep_step)
        dep_step.dependants.add(step)

    workers = [Worker(i + 1) for i in range(NUM_WORKERS)]

    order = ""
    time = 0

    done_size = len(all_steps) + 2

    print(
        f"{'time': ^6}",
        *(f"{w.name: ^3}" for w in workers),
        f"{'done': ^{done_size}}",
        sep="|",
    )
    print(
        "-" * 6,
        *("---" for _ in workers),
        "-" * done_size,
        sep="+",
    )

    while len(all_steps) > 0:
        idle_workers = list(worker for worker in workers if worker.job is None)
        to_delete = []

        for name, step in all_steps.items():
            if len(step.dependencies) > 0:
                continue

            if not step.worker and len(idle_workers) > 0:
                step.worker = idle_workers.pop()
                step.worker.job = step
                # print("starting step", step)

            if not step.worker:
                continue

            step.timeLeft -= 1

            if step.timeLeft <= 0:
                to_delete.append(name)
                # print("completed step", step)

        print(
            f"{time: ^6}",
            *(
                f"{worker.job.name: ^3}" if worker.job is not None else " . "
                for worker in workers
            ),
            f"{order: ^{done_size}}",
            sep="|",
        )

        for name in to_delete:
            # print("Ye", name)

            order += name

            step = all_steps[name]

            step.worker.job = None

            for dep in step.dependants:
                dep.dependencies.remove(step)

            del all_steps[name]

        time += 1

    print(time)
