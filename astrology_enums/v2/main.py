from enums import Planet, Sign


def find_planet_sign(plane_idx: int, degree: float):
    planet = Planet.from_index(plane_idx)
    sign = Sign.from_degree(degree)
    return planet, sign


def main():
    planet_idx = 4
    degree = 158.2859
    planet, sign = find_planet_sign(planet_idx, degree)
    print(f"惑星: {planet}")
    print(f"サイン: {sign}")

    sign = Sign.Virgo
    modality = sign.modality
    print(repr(modality))
    element = sign.element
    print(repr(element))
    sign2 = Sign.from_modality_element(modality, element)
    print(repr(sign2))


if __name__ == '__main__':
    main()
