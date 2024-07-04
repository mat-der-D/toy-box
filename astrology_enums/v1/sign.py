from enum import Enum


class Sign(Enum):
    Aries = (0, "牡羊座")
    Taurus = (1, "牡牛座")
    Gemini = (2, "双子座")
    Cancer = (3, "蟹座")
    Leo = (4, "獅子座")
    Virgo = (5, "乙女座")
    Libra = (6, "天秤座")
    Scorpio = (7, "蠍座")
    Sagittarius = (8, "射手座")
    Capricorn = (9, "山羊座")
    Aquarius = (10, "水瓶座")
    Pisces = (11, "魚座")

    def __init__(self, idx: int, jap: str):
        self.index = idx
        self.japanese = jap

    @classmethod
    def from_index(cls, idx: int) -> 'Sign':
        for sign in cls:
            if sign.index == idx:
                return sign
        raise ValueError(f"invalid index: {idx}")

    @classmethod
    def from_degree(cls, degree: float) -> 'Sign':
        idx = int(degree // 30) % 12
        return cls.from_index(idx)

    @property
    def modality(self) -> 'Modality':
        return Modality.from_index(self.index % len(Modality))

    @property
    def element(self) -> 'Element':
        return Element.from_index(self.index % len(Element))

    @classmethod
    def from_modality_element(cls, modality: 'Modality', element: 'Element') -> 'Sign':
        for sign in cls:
            if sign.modality == modality and sign.element == element:
                return sign
        raise ValueError(f"Sign not found: {repr(modality)}, {repr(element)}")


class Modality(Enum):
    Cardinal = (0, "活動宮")
    Fixed = (1, "不動宮")
    Mutable = (2, "柔軟宮")

    def __init__(self, idx: int, jap: str):
        self.index = idx
        self.japanese = jap

    @classmethod
    def from_index(cls, idx: int) -> 'Modality':
        for modality in cls:
            if modality.index == idx:
                return modality
        raise ValueError(f"invalid index: {idx}")

    @classmethod
    def from_degree(cls, degree: float) -> 'Modality':
        idx = int(degree // 30) % 3
        return cls.from_index(idx)


class Element(Enum):
    Fire = (0, "火")
    Earth = (1, "土")
    Air = (2, "風")
    Water = (3, "水")

    def __init__(self, idx: int, jap: str):
        self.index = idx
        self.japanese = jap

    @classmethod
    def from_index(cls, idx: int) -> 'Element':
        for element in cls:
            if element.index == idx:
                return element
        raise ValueError(f"invalid index: {idx}")

    @classmethod
    def from_degree(cls, degree: float) -> 'Element':
        idx = int(degree // 30) % 4
        return cls.from_index(idx)
