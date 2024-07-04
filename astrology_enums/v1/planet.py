from enum import Enum


class Planet(Enum):
    Sun = (0, "太陽")
    Moon = (1, "月")
    Mercury = (2, "水星")
    Venus = (3, "金星")
    Mars = (4, "火星")
    Jupiter = (5, "木星")
    Saturn = (6, "土星")
    Uranus = (7, "天王星")
    Neptune = (8, "海王星")
    Pluto = (9, "冥王星")

    def __init__(self, idx: int, jap: str):
        self.index = idx
        self.japanese = jap

    @classmethod
    def from_index(cls, idx: int) -> 'Planet':
        for planet in cls:
            if planet.index == idx:
                return planet
        raise ValueError(f"invalid index: {idx}")
