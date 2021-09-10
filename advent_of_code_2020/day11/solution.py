from typing import List
from copy import deepcopy

char = str

FLOOR = "."
EMPTY = "L"
OCCUPIED = "#"

def load_input(t: str, only_process=False) -> List[List[char]]:
    if not only_process:
        t = open(t, "r").read()
    return [list(l) for l in t.strip().split("\n")]

print_seats = lambda s: print("0123456789\n" + "\n".join("".join(c for c in l) for l in s) + "\n")

def debug_seat(seat, neighbours, coords):
    n = neighbours.copy()
    n[1][1] = seat

    s = ""
    for row in n:
        s += "\t".join(str(x) for x in row) + "\n"
    print(f"  {coords=}\n{s}")
    

def apply_rules_loop_part1(seats: List[List[char]]) -> List[List[char]]:
    new_seats = []
    while new_seats != seats:
        if not new_seats:
            new_seats = deepcopy(seats)
        else:
            seats = deepcopy(new_seats)

        # iterate over every seat
        for y in range(len(seats)):
            for x in range(len(seats[y])):
                seat = seats[y][x]

                if seat == FLOOR:
                    continue

                # get neighbours
                neighbours = [[None, None, None], [None, None, None], [None, None, None]]
                for i in range(-1, 2):
                    for n in range(-1, 2):
                        if n == i == 0:
                            continue
                        coord = (x + i, y + n)

                        # check if coords are in bounds
                        if 0 <= coord[0] < len(seats[y]) and 0 <= coord[1] < len(seats):
                            neighbours[n + 1][i + 1] = seats[coord[1]][coord[0]]

                # apply the rules
                if seat == EMPTY:
                    if not any(any(s == OCCUPIED for s in r) for r in neighbours):
                        new_seats[y][x] = OCCUPIED
                else:
                    if sum(sum(s == OCCUPIED for s in r) for r in neighbours) >= 4:
                        new_seats[y][x] = EMPTY
        
    return sum(sum(s == OCCUPIED for s in r) for r in new_seats), new_seats

def check_direction(seats, pos: tuple) -> tuple:
    def get_next(seats, pos, vec, dir):
        new_pos = (pos[0] + vec[0] * dir, pos[1] + vec[1] * dir)
        if 0 <= new_pos[0] < len(seats[0]) and 0 <= new_pos[1] < len(seats):
            return new_pos, seats[new_pos[1]][new_pos[0]]
        return new_pos, None

    cnt = 0
    for vec in [(1, 0), (0, 1), (1, 1), (1, -1)]:
        for i in (-1, 1):
            _pos, c = get_next(seats, pos, vec, i)
            while c == "." and c is not None:
                _pos, c = get_next(seats, _pos, vec, i)
            cnt += (c == "#")
    return cnt

def apply_rules_loop_part2(seats: List[List[char]]) -> List[List[char]]:
    new_seats = []
    while new_seats != seats:
        if not new_seats:
            new_seats = deepcopy(seats)
        else:
            seats = deepcopy(new_seats)

        # iterate over every seat
        for y in range(len(seats)):
            for x in range(len(seats[y])):
                seat = seats[y][x]

                if seat == FLOOR:
                    continue

                # get neighbours
                neighbours = check_direction(seats, (x, y))
                
                # apply the rules
                if seat == EMPTY:
                    if not neighbours:
                        new_seats[y][x] = OCCUPIED
                else:
                    if neighbours >= 5:
                        new_seats[y][x] = EMPTY
        
    return sum(sum(s == OCCUPIED for s in r) for r in new_seats), new_seats

def test():
    inp = load_input("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL", True)

    r1, seats = apply_rules_loop_part1(deepcopy(inp))
    r2, seats = apply_rules_loop_part2(deepcopy(inp))
    assert r1 == 37
    assert r2 == 26

def main(path):
    inps = load_input(path)
    r1, _ = apply_rules_loop_part1(deepcopy(inps))
    r2, _ = apply_rules_loop_part2(deepcopy(inps))
    print(f"Day11:\n  Part1: {r1}\n  Part2: {r2}")

if __name__ == "__main__":
    from sys import argv
    if len(argv) < 2:
        test()
    else:
        main(argv[1])