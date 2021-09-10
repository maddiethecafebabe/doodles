from math import prod
from collections import namedtuple

Input = namedtuple("Input", ["rules", "your_ticket", "nearby_tickets"])

def load_input(t: str, only_process=False) -> Input:
    if not only_process:
        t = open(t, "r").read().strip()

    t = t.split("\n\n")
    rules = []
    for l in t[0].split("\n"):
        name, rest = l.split(":")
        ranges = [[int(x.strip()) for x in r.split("-")] for r in rest.split("or")]
        rules.append((name, ranges))
    your_ticket = [int(x) for x in t[1].split("\n")[1].split(",")]
    nearby_tickets = [[int(x) for x in l.split(",")] for l in t[2].split("\n")[1:]]

    return Input(rules, your_ticket, nearby_tickets)

def do_part1(inp: Input):
    return sum(sum(0 if any(any(lower <= value <= upper for lower, upper in ranges) for _, ranges in inp.rules)else value for value in tik) for tik in inp.nearby_tickets)

def do_part2(inp: Input):
    valid_tiks = [x for x in filter(lambda tik: all(any(any(l <= v <= u for l, u in ranges) for _, ranges in inp.rules) for v in tik), inp.nearby_tickets)]

    # get what fieds each rule could map to
    possibilities = {}
    for name, ranges in inp.rules:
        for fidx in range(len(valid_tiks[0])):
            if all(any(l <= tik[fidx] <= u for l, u in ranges) for tik in valid_tiks):
                possibilities[name] = [fidx] + possibilities.get(name, [])

    # figure out which rule belongs to which field
    while any(len(x) > 1 for x in possibilities.values()):  # while there are still ambigious fields
        for k, v in possibilities.items():
            if len(v) > 1:
                continue
            elif len(v) == 1:                               # for every not ambigous field
                for v2 in possibilities.values():           
                    if v[0] in v2 and v2 is not v:          # remove it from the possible fields of 
                        v2.remove(v[0])                     # all others
    return prod(inp.your_ticket[possibilities[name][0]] for name in possibilities.keys() if name.startswith("departure"))

def test():
    inps = load_input("class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12", only_process=True)
    r1 = do_part1(inps)
    r2 = do_part2(inps)

    assert r1 == 71

def main(path):
    inps = load_input(path)

    print("Day16:\n  Part1: {}\n  Part2: {}".format(
        do_part1(inps),
        do_part2(inps)
    ))

if __name__ == "__main__":
    from sys import argv

    if len(argv) < 2:
        test()
    else:
        main(argv[1])
