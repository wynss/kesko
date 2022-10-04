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
               "alpha": self.alpha 
            }
        }

class Color(Enum):
    WHITE = Rgba(1.0, 1.0, 1.0)
    
    def to_json(self):
        return self.value.to_json()
