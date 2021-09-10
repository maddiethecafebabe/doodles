def load_inputs(t: str, only_process=False):
    if not only_process:
        t = open(t, "r").read()
    return [int(x) for x in t.strip().split("\n")]

def break_part1(codes, preamble_len = 25):
    # load preamble
    preamble = codes[:preamble_len]
    codes = codes[preamble_len:]
    found = None

    for n in codes:
        for a in preamble:
            for b in preamble:
                if a == b: continue
                if a + b == n:
                    found = (a, b)
                    break
        if found is None:
            return n

        found = None
        preamble.pop(0)
        preamble.append(n)

def break_part2(inps, preamble_len=25):
    invalid_num = break_part1(inps, preamble_len)

    sms = []
    for i, _ in enumerate(inps):
        si = i
        while sum(sms) < invalid_num:
            if si >= len(inps):
                break # just in case
            sms.append(inps[si])
            si += 1

        if sum(sms) == invalid_num:
            return min(sms) + max(sms)
        else:
            sms = []

def test():
    inp = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576"
    inps = load_inputs(inp, True)

    assert break_part1(inps, 5) == 127
    assert break_part2(inps, 5) == 62

def main(path):
    inps = load_inputs(path)
    print(f"Day9:\n  Part1: {break_part1(inps)}\n  Part2: {break_part2(inps)}")

if __name__ == "__main__":
    from sys import argv

    if len(argv) < 2:
        test()
    else:
        main(argv[1])
