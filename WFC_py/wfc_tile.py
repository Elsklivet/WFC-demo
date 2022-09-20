import random

CHOICES = [0,1,2,3,4]

class Tile:
    def __init__(self, x, y):
        self.choices = CHOICES.copy()
        self.value = None
        self.entropy = len(self.choices)
        self.collapsed = False
        self.x = x
        self.y = y

    def apply_rule(self, prev_value):
        if prev_value is None:
            return
        if prev_value == 0:
            if 2 in self.choices:
                self.choices.remove(2)
            if 3 in self.choices:
                self.choices.remove(3)
        elif prev_value == 1:
            if 3 in self.choices:
                self.choices.remove(3)
            if 4 in self.choices:
                self.choices.remove(4)
        elif prev_value == 2:
            if 2 in self.choices:
                self.choices.remove(2)
            if 3 in self.choices:
                self.choices.remove(3)
        elif prev_value == 3:
            if 0 in self.choices:
                self.choices.remove(0)
            if 1 in self.choices:
                self.choices.remove(1)
        elif prev_value == 4:
            if 1 in self.choices:
                self.choices.remove(1)
            if 2 in self.choices:
                self.choices.remove(2)
        self.entropy = len(self.choices)
        if self.entropy == 1:
            self.value = self.choices[0]
            self.collapsed = True

    def find_up(self, tilemap):
        if self.y == 0:
            return None
        return tilemap[self.x][self.y-1]

    def find_down(self, tilemap):
        if self.y == len(tilemap[0])-1:
            return None
        return tilemap[self.x][self.y+1]

    def find_left(self, tilemap):
        if self.x == 0:
            return None
        return tilemap[self.x-1][self.y]

    def find_right(self, tilemap):
        if self.x == len(tilemap)-1:
            return None
        return tilemap[self.x+1][self.y]

TILEMAP = [[Tile(x, y) for y in range(10)] for x in range(10)]

def observe(tile):
    if tile.collapsed:
        return
    tile.collapsed = True
    tile.value = random.choice(tile.choices)
    tile.entropy = 0
    prop_up(tile)
    prop_down(tile)
    prop_left(tile)
    prop_right(tile)


def prop_up(tile):
    up = tile.find_up(TILEMAP)
    if up is None:
        return
    prev_value = tile.value
    up.apply_rule(prev_value)
    observe(up)

def prop_down(tile):
    down = tile.find_down(TILEMAP)
    if down is None:
        return
    prev_value = tile.value
    down.apply_rule(prev_value)
    observe(down)

def prop_left(tile):
    left = tile.find_left(TILEMAP)
    if left is None:
        return
    prev_value = tile.value
    left.apply_rule(prev_value)
    observe(left)

def prop_right(tile): 
    right = tile.find_right(TILEMAP) 
    if right is None: 
        return
    prev_value = tile.value
    right.apply_rule(prev_value)
    observe(right)





def main():

    tilemap = TILEMAP.copy()

    for x in range(len(tilemap)):
        for y in range(len(tilemap[0])):
            print(tilemap[x][y].value, end=' ')
        print()

    start = random.choice(tilemap[random.randint(0, len(tilemap)-1)])
    observe(start)

    for x in range(len(tilemap)):
        for y in range(len(tilemap[0])):
            print(tilemap[x][y].value, end=' ')
        print()




if __name__ == '__main__':
    main()