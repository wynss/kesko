from enum import Enum


class Rgba:
    def __init__(self, red: float, green: float, blue: float, alpha: float = 1.0):
        self.red = red
        self.green = green
        self.blue = blue
        self.alpha = alpha

    def to_json(self):
        return {
            "Rgba": {
                "red": self.red,
                "green": self.green,
                "blue": self.blue,
                "alpha": self.alpha,
            }
        }

    def to_list(self):
        return [self.red, self.green, self.blue, self.alpha]


class Color(Enum):
    WHITE = Rgba(1.0, 1.0, 1.0)
    RED = Rgba(1.0, 0.0, 0.0)
    GREEN = Rgba(0.0, 1.0, 0.0, 1.0)
    BLUE = Rgba(0.0, 0.0, 1.0, 1.0)
    DEEP_ORANGE = Rgba(1.0, 0.34, 0.13, 1.0)

    def to_json(self):
        return self.value.to_json()
