from enum import Enum


class Language(Enum):
    Japanese = 0
    English = 1
    Spanish = 2


class TranslatableEnum(Enum):
    def translate_into(self, lang: Language) -> str:
        from i18n import translate  # import here to avoid circular import
        return translate(lang, self)


class Planet(TranslatableEnum):
    Sun = 0
    Moon = 1
    Mercury = 2
    Venus = 3
    Mars = 4
    Jupiter = 5
    Saturn = 6
    Uranus = 7
    Neptune = 8
    Pluto = 9


class Sign(TranslatableEnum):
    Aries = 0
    Taurus = 1
    Gemini = 2
    Cancer = 3
    Leo = 4
    Virgo = 5
    Libra = 6
    Scorpio = 7
    Sagittarius = 8
    Capricorn = 9
    Aquarius = 10
    Pisces = 11

    @classmethod
    def from_degree(cls, degree: float) -> 'Sign':
        return cls(int(degree // 30) % len(cls))

    @property
    def modality(self) -> 'Modality':
        return Modality(self.value % len(Modality))

    @property
    def element(self) -> 'Element':
        return Element(self.value % len(Element))

    @classmethod
    def from_modality_element(cls, modality: 'Modality', element: 'Element') -> 'Sign':
        for sign in cls:
            if sign.modality == modality and sign.element == element:
                return sign
        raise ValueError(f'No sign with modality {modality} and element {element}')


class Modality(TranslatableEnum):
    Cardinal = 0
    Fixed = 1
    Mutable = 2

    @classmethod
    def from_degree(cls, degree: float) -> 'Modality':
        return cls(int(degree // 30) % len(cls))


class Element(TranslatableEnum):
    Fire = 0
    Earth = 1
    Air = 2
    Water = 3

    @classmethod
    def from_degree(cls, degree: float) -> 'Element':
        return cls(int(degree // 30) % len(cls))
