import math

def transform(x):
    return (x // 3) - 2

def continuous_transform(x):
    fuel_sum = 0
    curr = transform(x)
    while curr > 0:
        fuel_sum += curr
        curr = transform(x)
    return fuel_sum

with open("../../input/01-1.txt") as f:
    total = sum(map(continuous_transform,(int(line) for line in f)))
    print(total)
