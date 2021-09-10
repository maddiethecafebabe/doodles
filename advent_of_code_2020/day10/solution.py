from typing import List


def load_inputs(t: str, only_process=False):
    if not only_process:
        t = open(t, "r").read()
    return [int(x) for x in t.strip().split("\n")]

def do_part1(jolts: List[int]) -> int:
    jolts = jolts[:]
    jolts.sort()
    jolts.append(jolts[-1] + 3)
    jolts.insert(0, 0)

    diffs = {0: 0, 1: 0, 2: 0, 3: 0}
    for i in range(len(jolts) - 1, 0, -1):
        diffs[jolts[i] - jolts[i - 1]] += 1
    return diffs[1] * diffs[3]

def flatten_dict(d, depth: int) -> dict:
    _d = d.copy()
    substituded = True
    print(f"{' ' * depth}{d=}")
    
    cnt = 0
    while substituded:
        print(_d)
        print(cnt)
        cnt += 1
        substituded = False

        for orig_key, orig_value in d.items():
            for n_key, new_value in _d.items():
                if orig_key in new_value and len(orig_value) > 1:
                    substituded = True
                    print(f"Substituting {orig_key} with {orig_value} in _d[{n_key}]->{new_value}")
                    n_val = new_value.copy()
                    for x in orig_value:
                        n_val.add(x)
                    n_val.remove(orig_key)
                    _d[n_key] = n_val

    return _d

def do_part2(jolts: List[int]) -> int:
    jolts = jolts[:]
    jolts.sort()
    jolts.insert(0, 0)
    jolts.append(jolts[-1] + 3)

    lookup = {}
    for i, n in enumerate(jolts):
        si = i
        avail = []
        while (sn := jolts[si]) - n <= 3:
            avail.append(sn)
            si += 1
            if sn == jolts[-1]:
                break
        lookup[n] = set(avail)

    lookup = flatten_dict(lookup, 1)

    for k, v in lookup.items():
        print(f"\t{k}: {v}")

def test():
    inps1 = load_inputs("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4", True)
    inps2 = load_inputs("28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3", True)

    assert do_part1(inps1) == 35
    assert do_part1(inps2) == 220
    
    assert do_part2(inps1) == 8
    x= do_part2(inps2)
    print(f"{x} should be {19208}")

def main(path):
    inps = load_inputs(path)

    print("Day10:\n  Part1: {}\n  Part2: {}".format(
        do_part1(inps), do_part2(inps)
    ))

if __name__ == "__main__":
    from sys import argv

    if len(argv) < 2:
        test()
    else:
        main(argv[1])
