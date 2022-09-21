import random
from Tile import Tile
from rules import pm_one, bne

CHOICES = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

def observe(tile, tilemap):
    if tile.collapsed:
        return
    tile.collapsed = True
    tile.value = random.choice(tile.choices)
    tile.entropy = 0
    # print(f"observed: {tile.x}, {tile.y}, collapsed to {tile.value}")

    # TODO:
    # 1. Propagate the value to the neighbors
    # 2. find the neighbor with the lowest entropy
    # 3. collapse that neighbor
    prop_up(tile, tilemap)
    prop_down(tile, tilemap)
    prop_left(tile, tilemap)
    prop_right(tile, tilemap)


def prop_up(tile, tilemap):
    # print(f"prop_up received: {tile.value}")
    up = tile.find_up(tilemap)
    if up is None:
        return
    prev_value = tile.value
    up.apply_rule(pm_one, prev_value)
    # print(f"the upper tile is now: {up.value}")
    observe(up, tilemap)

def prop_down(tile, tilemap):
    # print(f"prop_down received: {tile.value}")
    down = tile.find_down(tilemap)
    if down is None:
        return
    prev_value = tile.value
    down.apply_rule(pm_one, prev_value)
    # print(f"the lower tile is now: {down.value}")
    observe(down, tilemap)

def prop_left(tile, tilemap):
    # print(f"prop_left received: {tile.value}")
    left = tile.find_left(tilemap)
    if left is None:
        return
    prev_value = tile.value
    left.apply_rule(pm_one, prev_value)
    # print(f"the left tile is now: {left.value}")
    observe(left, tilemap)

def prop_right(tile, tilemap): 
    # print(f"prop_right received: {tile.value}")
    right = tile.find_right(tilemap) 
    if right is None: 
        return
    prev_value = tile.value
    right.apply_rule(pm_one, prev_value)
    # print(f"the right tile is now: {right.value}")
    observe(right, tilemap)


def main():

    size = 10

    choices = CHOICES.copy()
    print(choices)
    TILEMAP = [[Tile(x, y, choices) for y in range(size)] for x in range(size)]
    tilemap = TILEMAP.copy()

    for x in range(len(tilemap)):
        for y in range(len(tilemap[0])):
            print(tilemap[x][y].value, end=' ')
        print()

    start = random.choice(tilemap[random.randint(0, len(tilemap)-1)])
    observe(start, tilemap)

    for x in range(len(tilemap)):
        for y in range(len(tilemap[0])):
            print(tilemap[x][y].value, end=' ')
        print()


if __name__ == '__main__':
    main()