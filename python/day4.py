from functools import reduce

digit_offset = [1, 10, 100, 1000, 10000, 100000]
def get_digit(num, digit):
    return (num // digit_offset[digit]) % 10

# takes a list with the number of digits in each sequence of repeated digits
# returns (bool, bool) with the validity for part 1 and part 2 respectively
def valid_counts(counts):
    # folding through all counts once. accumulator is a tuple holding the
    # current validity status for each part
    valid = lambda acc, x: ((acc[0] or x >= 2),(acc[1] or x == 2))
    return reduce(valid, counts, (False, False))
    
def valid_password(num):
    digits = (get_digit(num, i) for i in reversed(range(6)))
    prev = next(digits)
    adjacent_counts = [1]
    for curr in digits:
        if prev > curr:
            return (False, False)
        if prev == curr:
            adjacent_counts[-1] += 1
        if prev < curr:
            adjacent_counts.append(1)
        prev = curr
    return valid_counts(adjacent_counts)

with open("../input/04-1.txt") as f:
    low, high = map(int,f.read().strip().split('-'))
    
p1_count = 0
p2_count = 0
for value in range(low,high+1):
    part1, part2 = valid_password(value)
    if part1:
        p1_count += 1
    if part2:
        p2_count += 1
print(p1_count, p2_count)

