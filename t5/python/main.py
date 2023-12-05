import sys

class NewMap:
    def __init__(self) -> None:
        self.ranges = []

    def add_range(self, start, offset, value):
        self.ranges.append((start, offset, value))

    def get(self, v):
        for start, offset, value in self.ranges:
            # print(start, end, value, v)
            if v >= start and v < start+offset:
                return value + (v-start)
        else:
            return v
        
    def get_rev(self, v):
        for value, offset, start in self.ranges:
            # print(start, offset, value, v)
            if v >= start and v < start+offset:
                return value + (v-start)
        else:
            return v
        
    def get_max_value(self):
        return max((tup[2] for tup in self.ranges))


def get_map(lines):
    m = NewMap()

    for line in lines:
        d, s, r = [int(i) for i in line.split()]

        m.add_range(s, r, d)
    
    return m

maps = {}

current_lines = []
reading_map = False
map_read = None

for line in sys.stdin:
    
    if line.startswith("seeds: "):
        seeds = [int(n) for n in line.removeprefix("seeds: ").strip().split()]
        continue

    if "map:" in line:
        map_read, _ = line.split('-', 1)
        reading_map = True
        continue


    if reading_map is True:
        if len(line.strip()) > 0:
            current_lines.append(line)
        else:
            maps[map_read] = get_map(current_lines)
            current_lines = []
            reading_map = False
            map_read = None 

maps[map_read] = get_map(current_lines)


def read_location(seed):
    soil = maps["seed"].get(seed)
    fertilizer  = maps["soil"].get(soil)
    water = maps["fertilizer"].get(fertilizer)
    light = maps["water"].get(water)
    temperature = maps["light"].get(light)
    humidity = maps["temperature"].get(temperature)
    return maps["humidity"].get(humidity)

def read_seed(location):
    humidity = maps["humidity"].get_rev(location)
    temperature = maps["temperature"].get_rev(humidity)
    light = maps["light"].get_rev(temperature)
    water = maps["water"].get_rev(light)
    fertilizer = maps["fertilizer"].get_rev(water)
    soil = maps["soil"].get_rev(fertilizer)
    return maps["seed"].get_rev(soil)

print("part 1:")
print(min([read_location(s) for s in seeds]))
print(all([s == read_seed(read_location(s)) for s in seeds]))

print("part 2:", )
seed_ranges = [(i, n) for i, n in zip(seeds[::2], seeds[1::2])]

for i in range(0,  maps["humidity"].get_max_value()):
    s = read_seed(i)
    for j, n in seed_ranges:
        if s >= j and s < j+n:
            print("part 2:", i)
            exit()

