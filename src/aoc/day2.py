import re

from .utils import read_file

DAY = "2"


def validate(input: str) -> bool:
    match = re.match(r"^(\d+)-(\d+) (.): (.+)$", input)
    assert match
    min = int(match[1])
    max = int(match[2])

    count = re.findall(match[3], match[4])

    if not count and min > 0:
        return False

    count = len(count)

    return count <= max and count >= min


def validate2(input: str) -> bool:
    match = re.match(r"^(\d+)-(\d+) (.): (.+)$", input)
    assert match
    one = int(match[1]) - 1
    two = int(match[2]) - 1
    char = match[3]
    password = match[4]

    one_v = password[one] == char
    two_v = password[two] == char
    return (one_v or two_v) and not (one_v and two_v)


def main():
    num_valid = 0
    for line in read_file(DAY):
        valid = validate2(line)
        print(f"Input: {line} Valid: {valid}")
        if valid:
            num_valid += 1
    print(num_valid)


if __name__ == "__main__":
    main()
