def load_input(f: str, just_process=False):
    if not just_process:
        f = open(f, "r").read()
    return [(x[:1], int(x[1:])) for x in f.strip().split("\n")]

mov_lookup = {"E": (1, 0), "W": (-1, 0), "S": (0, -1), "N": (0, 1)}
direction_lookup = ["E", "S", "W", "N"]

def get_manhattan_distance_part1(inputs: str, origin=(0, 0, 0)) -> int:
    coords = [*origin]

    for cmd, arg in inputs:
        # rotate ship
        if cmd in ("L", "R"):
            if cmd == "L":
                arg = 360 - arg
            coords[2] = (coords[2] + arg // 90) % 4
        else:
            # replace F with the current direction
            if cmd == "F":
                cmd = direction_lookup[coords[2]]
            # move ship
            for i in range(2):
                coords[i] = coords[i] + arg * mov_lookup[cmd][i]
    return abs(coords[0] - origin[0]) + abs(coords[1] - origin[1])

def get_manhattan_distance_part2(inputs: str, origin=(0,0), waypoint=[10,1]) -> int:
    coords = [*origin]
    rot = {"L": 1, "R": -1}

    for cmd, arg in inputs:
        # do a 90° point rotate (P(x, y) => P(-y, x)) every 90°
        if cmd in ("L", "R"):
            for _ in range(arg // 90):
                waypoint = [-waypoint[1] * rot[cmd], waypoint[0] * rot[cmd]]
        # move ship n times multiplied with the waypoint vector
        elif cmd == "F": 
            coords = [coords[0] + waypoint[0] * arg, coords[1] + waypoint[1] * arg]
        # move waypoint relative vector n times in cmd direction
        else:
            for i in range(2):
                waypoint[i] += arg * mov_lookup[cmd][i]

    return abs(coords[0] - origin[0]) + abs(coords[1] - origin[1])

def test():
    example_input = load_input("F10\nN3\nF7\nR90\nF11", True)

    assert get_manhattan_distance_part1(example_input) == 25
    assert get_manhattan_distance_part2(example_input) == 286

def main(argv):
    inp = load_input(argv[1])
    print("Day12:\n  Part1: {}\n  Part2: {}".format(
        get_manhattan_distance_part1(inp),
        get_manhattan_distance_part2(inp)
    ))

if __name__ == "__main__":
    from sys import argv

    if len(argv) < 2:
        test()
    else:
        main(argv)
