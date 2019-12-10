from math import gcd
from collections import defaultdict
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
    def __repr__(self):
        return "({},{})".format(self.x,self.y)
    
field = set()
with open("../input/10-1.txt") as f:
    for y,line in enumerate(f):
        for x,c in enumerate(line):
            if c == '#':
                field.add(Point(x,y))

def simplify(x,y):
    divisor = gcd(x,y)
    return x//divisor,y//divisor
def get_visible(asteroid, field):
    adjacency = defaultdict(list)
    for other in field:
        if other == asteroid:
            continue
        angle = simplify(other.x - asteroid.x, other.y - asteroid.y)
        adjacency[angle].append(other)
    return adjacency

visibilities = {}
counts = {}
for asteroid in field:
    vis = get_visible(asteroid, field)
    counts[asteroid] = len(vis)
    visibilities[asteroid] = vis

station_pos, part1 = max(counts.items(), key=lambda x: x[1])
print(part1)
from math import atan2
from itertools import cycle, dropwhile
sorted_angles = sorted(visibilities[station_pos].keys(), key=lambda x: atan2(x[1],x[0]))
angles = dropwhile(lambda x: x != (0,-1), cycle(sorted_angles))
vis = visibilities[station_pos]
for i in range(0,199):
    destroyed = False
    while not destroyed:
        curr = next(angles)
        if vis[curr]:
            print(i, curr, vis[curr])
            vis[curr].pop(0)
            destroyed = True

result = next(angles)
x,y = vis[result][0].x,vis[result][0].y
print(result, vis[result], 100 * x + y)
