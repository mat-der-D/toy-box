from enums import Planet, Sign, Language


def main():
    planet = Planet(0)
    print(planet)
    print(planet.translate_into(Language.Japanese))
    print(planet.translate_into(Language.English))
    print(planet.translate_into(Language.Spanish))

    sign = Sign.Aries
    print(sign.translate_into(Language.Japanese))
    modality = sign.modality
    print(modality.translate_into(Language.Japanese))
    element = sign.element
    print(element.translate_into(Language.Japanese))
    sign2 = Sign.from_modality_element(modality, element)
    print(sign2.translate_into(Language.Japanese))


if __name__ == '__main__':
    main()
