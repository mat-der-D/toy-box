from planet import Planet
from sign import Sign


def find_planet_sign(plane_idx: int, degree: float):
    planet = Planet.from_index(plane_idx)
    sign = Sign.from_degree(degree)
    return planet, sign


def main():
    planet_idx = 4
    degree = 158.2859
    planet, sign = find_planet_sign(planet_idx, degree)
    print(f"惑星: {planet.japanese}")
    print(f"サイン: {sign.japanese}")

    sign = Sign.Virgo
    modality = sign.modality
    print(modality.japanese)
    element = sign.element
    print(element.japanese)
    sign2 = Sign.from_modality_element(modality, element)
    print(sign2.japanese)


if __name__ == '__main__':
    main()
