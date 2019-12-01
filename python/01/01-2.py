import math

def transformation(x):
    return math.floor(x / 3) - 2

def continuous_transform(x):
    fuel_sum = 0
    curr = x
    while True:
        result = transformation(curr)
        if result > 0:
            fuel_sum += result
            curr = result
        else:
            break
    return fuel_sum

with open("../../input/01-1.txt") as f:
    total = sum(map(continuous_transform,(int(line) for line in f)))
    print(total)
