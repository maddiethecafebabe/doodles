from sys import argv
from collections import namedtuple
from typing import List, Tuple
from re import findall, compile

sub_regex = compile(r"(?:(\d)\s((?:\w+\s\w+)))+")

class BagRule:
    colour: str = None
    sub_bags: List[Tuple[int, str]] = []

    def __init__(self, raw: str):
        colour, subs = raw.split(" bags contain")
        self.colour = colour.strip()
        self.sub_bags = findall(sub_regex, subs)
            
    def __repr__(self):
        l = ", ".join("  {0} * {1}".format(*bag) for bag in self.sub_bags) or "  None"
        return (f"<BagRule for {self.colour}:" + l + ">")

def load_rules(txt: str):
    rules = {}
    for raw_rule in txt.split("\n"):
        rule = BagRule(raw_rule)
        rules[rule.colour] = rule
    return rules

def can_bag_hold(rules: dict, bag: BagRule, name: str, orig: str=None) -> bool:
    log(f"can_bag_hold::({bag.colour=}, {name=}, {orig=})")
    
    if orig is None:
        orig = bag.colour
    for _, s in bag.sub_bags:
        if s == name:
            log(f"  [+] [{orig}] {bag.colour} can hold {name}")
            return True
        if (sub := rules[s]).sub_bags:
            if can_bag_hold(rules, sub, name, orig):
                return True
    log(f"  [-] [{orig}] {bag.colour} can not hold {name}")
    return False

def how_many_does_bag_hold(rules: dict, bag: BagRule, orig: str=None, already_done: dict = {}) -> int:
    log(f"how_many_does_bag_hold::({bag.colour=}, {orig=}, {already_done=})")
    if orig is None:
        orig = bag.colour

    r_cnt = 0
    for cnt, s in bag.sub_bags:
        if s in already_done.keys():
            log(f"  [+] already_done: {s} -> {already_done[s]}")
            r_cnt += int(cnt) * already_done[s] + int(cnt)
        else:
            log(f"  [?] {s=} not in {already_done=}")
            if (sub := rules[s]).sub_bags:
                log(f"  [?] {s=} contains sub bags")
                r_cnt += int(cnt) *  how_many_does_bag_hold(rules, sub, orig, already_done) + int(cnt)
            else:
                log(f"  [?] {sub=} singleton")
                log(f"    {r_cnt=} + {int(cnt)}")
                r_cnt += 1 * int(cnt)
            already_done[bag.colour] = r_cnt
    return r_cnt


log = lambda *args: 0
rules = load_rules(open(argv[1], "r").read().strip())
print(
    "Day7:\n  Part1: {}\n  Part2: {}".format(
        sum(can_bag_hold(rules, r, "shiny gold") for r in rules.values()),
        how_many_does_bag_hold(rules, rules["shiny gold"])
    )
)
