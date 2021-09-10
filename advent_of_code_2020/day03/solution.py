from argparse import ArgumentParser
from math import prod

init_map = lambda path: open(path, "r").read().strip().split("\n")
map_get = lambda map, x, y: map[y][x % len(map[0])]
get_tree_cnt = lambda map, x_vec, y_vec: sum(map_get(map, x, y) == "#" for x, y in zip(range(0, 99999, x_vec), range(0, len(map), y_vec)))


def main(args):
    map = init_map(args.input)

    tree_sum = get_tree_cnt(map, 3, 1)
    print(f"Part1: {tree_sum}")

    tree_mult = prod(get_tree_cnt(map, *vec) for vec in ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2)))
    print(f"Part2: {tree_mult}")

if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("input", default="input.txt")

    main(parser.parse_args())
