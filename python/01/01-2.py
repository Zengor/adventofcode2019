import math

def fuel_needed(x):
    return (x // 3) - 2

def total_fuel(x):
    fuel_sum = 0
    curr = fuel_needed(x)
    while curr > 0:
        fuel_sum += curr
        curr = fuel_needed(x)
    return fuel_sum

with open("../../input/01-1.txt") as f:
    total = sum(map(continuous_transform,(int(line) for line in f)))
    print(total)
