import math

def transformation(x):
    return math.floor(x / 3) - 2

with open("../../input/01-1.txt") as f:
    total = sum(map(transformation,(int(line) for line in f)))
    print(total)
