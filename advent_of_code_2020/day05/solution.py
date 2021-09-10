def passport_to_id(p: str) -> int:
    return int(p[:-3].replace("B", "1").replace("F", "0"), 2) * 8 + int(p[-3:].replace("R", "1").replace("L", "0"), 2)
ps = [passport_to_id(p) for p in open("input.txt", "r").read().strip().split("\n")]
print([ps.sort(), ps][-1][-1]) # solve day 1 and prepare list for day2
print([*filter(lambda pid: pid+1 not in ps and pid not in [ps[0], ps[-1]],ps)][0] + 1) # day2
