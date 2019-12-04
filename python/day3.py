from itertools import product

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
    def __repr__(self):
        return "({},{})".format(self.x,self.y)

class WireSegment:
    def __init__(self, p, q, start_dist):
        self.p = p
        self.q = q
        self.start_dist = start_dist
        
    def is_horizontal(self):
        return self.horizontal_len() != 0
    def is_vertical(self):
        return self.vertical_len() != 0
    def horizontal_len(self):
        return abs(self.p.x - self.q.x)
    def vertical_len(self):
        return abs(self.p.y - self.q.y)
    def length(self):
        # because one of these will always be 0, this returns the correct total length
        return self.horizontal_len() + self.vertical_len()
    def horizontal_range(self):        
        return range(min(self.p.x, self.q.x), max(self.p.x, self.q.x))
    def vertical_range(self):
        return range(min(self.p.y, self.q.y), max(self.p.y, self.q.y))
    
    # segments are always parallel to one of the axes.
    # this method returns the value for the axis this segment is perpendicular to.
    # ex: for a segment (0,5)->(0,7), this returns 0 as that is it's x value
    def perpendicular_projection(self):
        if self.is_horizontal():
            return self.p.y #as both values are the same it doesn't matter which point picked
        else:
            return self.p.x
    # this method returns a range of values which this segment covers in its parallel axis
    # ex: for a segment (0,5)->(0,7), this returns range(5,7)
    def parallel_range(self):
        if self.is_horizontal():
            return self.horizontal_range()
        else:
            return self.vertical_range()
            
    def intersection(self, other):
        # check if intersection is even possible
        # we know these segments are always parallel to one of the axes
        # if both are parallel, they can't intersect
        # (it's assumed they can't overlap for the scope of this problem)
        if (self.is_horizontal() and other.is_horizontal()) or \
           (self.is_vertical() and other.is_vertical()):
            return None
        # given that they are perpendicular, we check that its
        # projection is within the other's range, and vice-versa
        if self.perpendicular_projection() in other.parallel_range() and \
           other.perpendicular_projection() in self.parallel_range():
           # in this case, the point where the perpendicular projections meets
           # is the intersection. However, which is which depends on which of the two
           # is horizontal, so there needs to be an additional check
            if self.is_horizontal():
                return Point(other.p.x, self.p.y)
            else:
                return Point(self.p.x, other.p.y)
        return None
    
        
    def intersection_step_dist(self, other):        
        def point_dist(segment, point):
            return segment.start_dist + manhattan_dist(segment.p, point)
        point = self.intersection(other)
        if point is None:
            return None
        return point_dist(self, point) + point_dist(other, point)
    
    def __repr__(self):        
        return "{}→{}".format(self.p,self.q)
    
def parse_wire(wire_str):
    dir_mods = {
        'U': (0,-1),
        'D': (0,1),
        'L': (-1,0),
        'R': (1,0),
    }
    segments = []
    curr_pos = Point(0,0)
    length = 0    
    moves = wire_str.split(',')
    for move in moves:
        direction, steps = dir_mods[move[0]], int(move[1:])
        seg_end = Point(curr_pos.x + direction[0] * steps, curr_pos.y + direction[1] * steps)
        segment = WireSegment(curr_pos, seg_end, length)
        length += segment.length()
        curr_pos = seg_end
        segments.append(segment)
    return segments

def manhattan_dist(a,b):
    return abs(a.x - b.x) + abs(a.y - b.y)

def closest_intersection_manhattan(wire_a,wire_b):
    center = Point(0,0)
    dist_to_center = lambda p: manhattan_dist(center,p)
    results = (a.intersection(b) for (a,b) in product(wire_a,wire_b))
    return min(map(dist_to_center,(p for p in results if p is not None)))

def closest_intersection_steps(wire_a,wire_b):
    results = (a.intersection_step_dist(b) for (a,b) in product(wire_a,wire_b))
    return min(filter(lambda d: d is not None, results))

if __name__ == "__main__":
    data = None 
    with open("../input/03-1.txt") as f:
        data = tuple((parse_wire(line) for line in f))
    
    answer1 = closest_intersection_manhattan(data[0],data[1])
    answer2 = closest_intersection_steps(data[0],data[1])
    print("part 1 {} part 2 {}".format(answer1, answer2))
