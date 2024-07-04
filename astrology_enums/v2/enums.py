from enum import Enum


class IdxJapEnum(Enum):
    def __init__(self, idx: int, jap: str):
        self.index = idx
        self.japanese = jap

    def __str__(self):
        return self.japanese

    def __repr__(self):
        return f"{self.__class__.__name__}.{self.name}({self.index}, {self.japanese})"

    @classmethod
    def from_index(cls, idx: int) -> 'IdxJapEnum':
        res = idx % len(cls)
        for variant in cls:
            if variant.index == res:
                return variant
        raise ValueError(f"invalid index: {idx}")


class SignLikeEnum(IdxJapEnum):
    @classmethod
    def from_degree(cls, degree: float) -> 'SignLikeEnum':
        idx = int(degree // 30) % len(cls)
        return cls.from_index(idx)


class Sign(SignLikeEnum):
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

    @property
    def modality(self) -> 'Modality':
        return Modality.from_index(self.index)

    @property
    def element(self) -> 'Element':
        return Element.from_index(self.index)

    @classmethod
    def from_modality_element(cls, modality: 'Modality', element: 'Element') -> 'Sign':
        for sign in cls:
            if sign.modality == modality and sign.element == element:
                return sign
        raise ValueError(f"Sign not found: {repr(modality)}, {repr(element)}")


class Modality(SignLikeEnum):
    Cardinal = (0, "活動宮")
    Fixed = (1, "不動宮")
    Mutable = (2, "柔軟宮")


class Element(SignLikeEnum):
    Fire = (0, "火")
    Earth = (1, "土")
    Air = (2, "風")
    Water = (3, "水")


class Planet(IdxJapEnum):
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
