groups = open(__builtins__.__import__("sys").argv[1], "r").read().strip().split("\n\n")

part1_cnt = sum(len({c for c in group.replace("\n", "")}) for group in groups)
part2_cnt = sum(sum(all(c in p for p in group.split("\n")) for c in "abcdefghijklmnopqrstuvwxyz") for group in groups)

print(f"Day6\n  Part1: {part1_cnt}\n  Part2: {part2_cnt}")
