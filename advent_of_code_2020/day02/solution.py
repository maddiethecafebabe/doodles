from argparse import ArgumentParser
from re import findall


def old_is_valid_password(lower: int, upper: int, char: str, pw: str) -> bool:
    return int(lower) <= sum(1 for c in pw if c == char) <= int(upper)

def new_is_valid_password(lower: int, upper: int, char: str, pw: str) -> bool:
    return sum(1 for idx in (int(lower), int(upper)) if pw[idx - 1] == char) == 1

def main(args):
    with open(args.input, "r") as fp:
        inputs = findall(r"(\d+)-(\d+)\s(\w):\s(\w+)", fp.read())

    old_valid_cnt, new_valid_cnt = 0, 0
    for bundle in inputs:
        old_valid_cnt += old_is_valid_password(*bundle)
        new_valid_cnt += new_is_valid_password(*bundle)

    print(
        f"Amount of valid passwords:\n"
        f"  old policy: {old_valid_cnt}\n"
        f"  new policy: {new_valid_cnt}"
    )

if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("input", default="input.txt")

    main(parser.parse_args())
