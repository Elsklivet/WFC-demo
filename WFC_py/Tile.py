class Tile:
    def __init__(self, x: int, y: int, choices: list):
        self.choices = choices
        self.value = None
        self.entropy = len(self.choices)
        self.collapsed = False
        self.x = x
        self.y = y

    def apply_rule(self, rule, prev_value):
        rule(self, prev_value)

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
