# Because why not


## Day 1
```py
[print([b2, b3][i](re(s.argv[1]), s.argv[2] if len(s.argv) > 2 else 2020)[0]) for i in range(2) for b3 in [lambda x, w: [(a,b, c, a*b*c) for c in x for b in x for a in x if a+b+c == w]] for b2 in [lambda x, w: [(a,b, a*b) for b in x for a in x if a+b == w]] for re in (lambda f: [int(x.strip()) for x in open(f, "r").readlines()], ) for s in (__builtins__.__import__("sys"), )]
```

## Day 2
```py
[[print(f"Amount of valid passwords:\n  old policy: {sum(1 for entry in entries if old_is_valid_password(*entry))}\n  new policy: {sum(1 for entry in entries if new_is_valid_password(*entry))}") for entries in (findall(r"(\d+)-(\d+)\s(\w):\s(\w+)", open(sys.argv[1], "r").read()), )] for sys, findall, new_is_valid_password, old_is_valid_password in [(__builtins__.__import__("sys"), __builtins__.__import__("re").findall, lambda lower, upper, char, pw: sum(1 for idx in (int(lower), int(upper)) if pw[idx - 1] == char) == 1, lambda lower, upper, char, pw: int(lower) <= sum(1 for c in pw if c == char) <= int(upper))]]
```

## Day 3
```py
[[[print(f"Part1: {get_tree_cnt(map, 3, 1, map_get)}\nPart2: {prod(get_tree_cnt(map, *vec, map_get) for vec in ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2)))}")] for map in (init_map(argv[1] if len(argv) > 1 else "input.txt"), )] for init_map, map_get, get_tree_cnt, argv, prod in [(lambda path: open(path, "r").read().strip().split("\n"), lambda map, x, y: map[y][x % len(map[0])], lambda map, x_vec, y_vec, map_get: sum(map_get(map, x, y) == "#" for x, y in zip(range(0, 99999, x_vec), range(0, len(map), y_vec))), __builtins__.__import__("sys").argv, __builtins__.__import__("math").prod)]]
```

## Day 4
```py
[[[[[[[print(f"Day4\n  Part1: {sum(has_all_needed([field for field, _ in matches_], needed) for matches_ in matches)}\n  Part2: {sum(all((checks[field](value)) for field, value in matches_) and has_all_needed([field for field, _ in matches_], needed) for matches_ in matches)}")] for matches in [[findall(r"(\w+)(?::)(\S+)", o) for o in read_file(argv[1])]]] for checks in [{"byr": lambda v: valid_int_range(v, 1920, 2002, 4), "iyr": lambda v: valid_int_range(v, 2010, 2020, 4), "eyr": lambda v: valid_int_range(v, 2020, 2030, 4), "hgt": lambda v: v[-2:] in ["cm", "in"] and valid_int_range(v[:-2], *((150, 193) if v[-2:] == "cm" else (59, 76))), "hcl": lambda v: v.startswith("#") and valid_int(v[1:], 6, hex=True), "ecl": lambda v: v in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"], "pid": lambda v: valid_int(v, 9), "cid": lambda v: True}]] for valid_int_range in [lambda v, low, up, length=None, hex=False: valid_int(v, length, hex) and (low <= int(v) <= up)]] for needed, read_file, valid_int, has_all_needed in [(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",], lambda path: [" ".join(o.strip().split("\n")) for o in open(path, "r").read().split("\n\n")], lambda v, length=None, hex=False: all(c in [digits, hexdigits.lower()][hex] for c in v) and (True if length is None else len(v) == length), lambda l, needed: all(x in l for x in needed))]] for argv, findall, digits, hexdigits in ((__builtins__.__import__("sys").argv, __builtins__.__import__("re").findall, __builtins__.__import__("string").digits, __builtins__.__import__("string").hexdigits), )]]
```

## Day 5
```py
[[(print(f"Day5:\n  Part1: {[ps.sort(), ps][-1][-1]}"), print(f"  Part2: {[*filter(lambda pid: pid+1 not in ps and pid not in [ps[0], ps[-1]],ps)][0] + 1}"))] for ps in [[(int(p[:-3].replace("B", "1").replace("F", "0"), 2) * 8 + int(p[-3:].replace("R", "1").replace("L", "0"), 2)) for p in open(__builtins__.__import__("sys").argv[1], "r").read().strip().split("\n")]]]
```

## Day 6
```py
print("Day6\n  Part1: {}\n  Part2: {}".format(sum(len({c for c in group.replace("\n", "")}) for group in open(__builtins__.__import__("sys").argv[1], "r").read().strip().split("\n\n")), sum(sum(all(c in p for p in group.split("\n")) for c in "abcdefghijklmnopqrstuvwxyz") for group in open(__builtins__.__import__("sys").argv[1], "r").read().strip().split("\n\n"))))
```

