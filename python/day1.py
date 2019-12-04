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

if __name__ == "__main__":
    with open("../input/01-1.txt") as f:
        answer1 = sum(map(transformation,(int(line) for line in f)))
        answer2 = sum(map(continuous_transform,(int(line) for line in f)))
        print("part 1 {} part 2 {}", answer1, answer2)
