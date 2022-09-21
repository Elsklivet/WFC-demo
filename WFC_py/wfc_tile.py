import random
from Tile import Tile
from rules import pm_one, bne

CHOICES = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

def observe(tile, tilemap):
    if tile.collapsed:
        print(f"Tile {tile.x}, {tile.y} is already collapsed, returning")
        return
    tile.collapsed = True
    tile.value = random.choice(tile.choices)
    tile.entropy = 0
    print(f"observed: {tile.x}, {tile.y}, collapsed to {tile.value}")

    # TODO:
    # 1. Propagate the value to the neighbors
    # 2. find the neighbor with the lowest entropy
    # 3. collapse that neighbor
    prop_up(tile, tilemap)
    prop_down(tile, tilemap)
    prop_left(tile, tilemap)
    prop_right(tile, tilemap)

    choose_next(tile, tilemap)
    # print(f"next: {next_tile.x}, {next_tile.y}, with options {next_tile.choices}")
    # observe(next_tile, tilemap)


def choose_next(tile, tilemap):
    # find the tile with the lowest entropy
    # find its neighbors
    up = tile.find_up(tilemap)
    down = tile.find_down(tilemap)
    left = tile.find_left(tilemap)
    right = tile.find_right(tilemap)
    # get their entropies
    entropy = {
        "up": float('inf') if (up is None or up.collapsed) else up.entropy,
        "down": float('inf') if (down is None or down.collapsed) else down.entropy,
        "left": float('inf') if (left is None or left.collapsed) else left.entropy,
        "right": float('inf') if (right is None or right.collapsed) else right.entropy
    }
    sorted_by_entropy = dict(sorted(entropy.items(), key=lambda item: item[1]))
    # choose the neighbor with the lowest entropy
    # TODO:
    # 1. if there are multiple neighbors with the same entropy, choose one at random
    # 2. if there are no neighbors with entropy, choose the tile with the lowest entropy using repick()

    print(f"sorted_by_entropy: {sorted_by_entropy}")
    lowest = list(sorted_by_entropy.values())[0]
    range_to_pick = [i for i, x in enumerate(sorted_by_entropy.values()) if x == lowest]
    print(f"range_to_pick: {range_to_pick}")
    chosen_index = random.choice(range_to_pick)
    chosen_tile = list(sorted_by_entropy.keys())[chosen_index]
    print(f"chosen: {chosen_index}")
    if lowest == float('inf'):
        print("no neighbors with entropy, repicking")
        return 
        # repick()
    if chosen_tile == "up":
        # print("chose up")
        observe(up, tilemap)
    elif chosen_tile == "down":
        # print("chose down")
        observe(down, tilemap)
    elif chosen_tile == "left":
        # print("chose left")
        observe(left, tilemap)
    elif chosen_tile == "right":
        # print("chose right")
        observe(right, tilemap)
    else:
        print("Could not make a choice")


def repick():
    pass


def prop_up(tile, tilemap):
    # print(f"prop_up received: {tile.value}")
    up = tile.find_up(tilemap)
    if up is None:
        return
    prev_value = tile.value
    up.apply_rule(pm_one, prev_value)
    # print(f"the upper tile is now: {up.value}")
    # next_tile = choose_next(up, tilemap)
    # observe(next_tile, tilemap)

def prop_down(tile, tilemap):
    # print(f"prop_down received: {tile.value}")
    down = tile.find_down(tilemap)
    if down is None:
        return
    prev_value = tile.value
    down.apply_rule(pm_one, prev_value)
    # print(f"the lower tile is now: {down.value}")
    # next_tile = choose_next(down, tilemap)
    # observe(next_tile, tilemap)
    # observe(down, tilemap)

def prop_left(tile, tilemap):
    # print(f"prop_left received: {tile.value}")
    left = tile.find_left(tilemap)
    if left is None:
        return
    prev_value = tile.value
    left.apply_rule(pm_one, prev_value)
    # print(f"the left tile is now: {left.value}")
    # next_tile = choose_next(left, tilemap)
    # observe(next_tile, tilemap)
    # observe(left, tilemap)

def prop_right(tile, tilemap): 
    # print(f"prop_right received: {tile.value}")
    right = tile.find_right(tilemap) 
    if right is None: 
        return
    prev_value = tile.value
    right.apply_rule(pm_one, prev_value)
    # print(f"the right tile is now: {right.value}")
    # next_tile = choose_next(right, tilemap)
    # observe(next_tile, tilemap)
    # observe(right, tilemap)


def main():

    size = 3

    choices = CHOICES.copy()
    print(choices)
    TILEMAP = [[Tile(x, y, choices) for y in range(size)] for x in range(size)]
    tilemap = TILEMAP.copy()

    print(f"Beginning with size = {size}")

    start = random.choice(tilemap[random.randint(0, len(tilemap)-1)])
    observe(start, tilemap)

    for x in range(len(tilemap)):
        for y in range(len(tilemap[0])):
            print(tilemap[x][y].value, end=' ')
        print()


if __name__ == '__main__':
    main()