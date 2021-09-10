

def do_part1(starting, cnt: int=1):
    stack = starting.copy()

    table = {i: 0 for i in starting}
    print(table)


print(do_part1([int(i) for i in "0,3,6".split(",")], 10))