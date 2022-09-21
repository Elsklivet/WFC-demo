def pm_one(tile, prev_value):
    if prev_value is None:
        return
    tile.choices = list(filter(lambda x: x <= prev_value + 1 and x >= prev_value - 1, tile.choices))
    # print(f"pm_one: {tile.choices}")
    tile.entropy = len(tile.choices)
    if tile.entropy == 1:
        tile.value = tile.choices[0]
        tile.collapsed = True

def bne(tile, prev_value):
    if prev_value is None:
        return
    tile.choices = list(filter(lambda x: x != prev_value, tile.choices))
    # print(f"bne: {tile.choices}")
    tile.entropy = len(tile.choices)
    if tile.entropy == 1:
        tile.value = tile.choices[0]
        tile.collapsed = True
