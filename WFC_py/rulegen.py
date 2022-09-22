class World:
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.contents = [[None for y in range(height)] for x in range(width)]

    def add_pattern(self, pattern: list):
        for i in range(len(self.contents)):
            for j in range(len(self.contents[i])):
                self.contents[i][j] = pattern[i*len(self.contents[i])+j]

    def render(self):
        for i in range(self.height):
            for j in range(self.width):
                print(self.contents[j][i], end=' ')
            print()


# generate adjacency rules
def generate_rules(world, N):
    rule = {}
    for i in range(len(world.contents)):
        for j in range(len(world.contents[i])):
            this_content = world.contents[i][j]
            if world.contents[i][j] not in rule:
                rule[this_content] = {}
                for direction in range(4):
                    rule[this_content][direction] = []

    for i in range(len(world.contents)):
        for j in range(len(world.contents[i])):
            this_content = world.contents[i][j]
            if i > 0: # left
                rule[this_content][0].append(world.contents[i-1][j])
                rule[this_content][0] = list(set(rule[this_content][0]))
            if i < world.width-1: # right
                rule[this_content][1].append(world.contents[i+1][j])
                rule[this_content][1] = list(set(rule[this_content][1]))
            if j > 0: # up
                rule[this_content][2].append(world.contents[i][j-1])
                rule[this_content][2] = list(set(rule[this_content][2]))
            if j < world.height-1: # down
                rule[this_content][3].append(world.contents[i][j+1])
                rule[this_content][3] = list(set(rule[this_content][3]))
    return rule




def main():
    N = 2
    world = World(4, 4)
    for i in range(len(world.contents)):
        for j in range(len(world.contents[i])):
            world.contents[i][j] = 0
    world.render()
    pattern = [1,1,2,2,
               1,1,2,2,
               3,3,4,4,
               3,3,4,4]
    world.add_pattern(pattern)
    generate_rules(world, N)
    world.add_pattern(pattern)
    rule = generate_rules(world, N)
    print(rule)
    world.render()


if __name__ == '__main__':
    main()