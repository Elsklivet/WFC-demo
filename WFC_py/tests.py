from Tile import Tile
import rules
import wfc_tile
import random
import unittest


class TestTile(unittest.TestCase):
    pass

class TestRules(unittest.TestCase):
    pass

class TestWfcTile(unittest.TestCase):
    
    def test_choose_next(self):
        size = 3
        choices = wfc_tile.CHOICES.copy()
        TILEMAP = [[Tile(x, y, choices) for y in range(size)] for x in range(size)]
        tilemap = TILEMAP.copy()
        start = random.choice(tilemap[random.randint(0, len(tilemap)-1)])
        print(f"start_pos: {start.x}, {start.y}")
        wfc_tile.observe(start, tilemap)
        print(f"start_val: {start.value}")
        next_tile = wfc_tile.choose_next(start, tilemap)
        print(f"next_tile: {next_tile.x}, {next_tile.y}")


if __name__ == '__main__':
    unittest.main()
        