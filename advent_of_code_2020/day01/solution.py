from argparse import ArgumentParser
from sys import argv


def find_wanted_sum(entries: list, wanted_sum: int, amount_summands: int) -> tuple:
    if amount_summands == 2:
        for i in entries:
            for n in entries:
                if i + n == wanted_sum:
                    print(f"Found wanted sum {wanted_sum}: {i=}, {n=} => {i * n}")
                    return i, n
    else:
        for i in entries:
            for n in entries:
                for a in entries:
                    if i + n + a == wanted_sum:
                        print(f"Found wanted sum {wanted_sum}: {i=}, {n=}, {a=} => {a * i * n}")
                        return a, i, n

def main(args):
    with open(args.input_file, "r") as fp:
        entries = [int(x.strip()) for x in fp.readlines()]
    find_wanted_sum(entries, args.wanted_sum, args.amount_summands)

if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("input_file")
    parser.add_argument("--wanted-sum", type=int, default=2020)
    parser.add_argument("--amount-summands", type=int, default=2)

    main(parser.parse_args())
