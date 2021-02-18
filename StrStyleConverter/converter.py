# ***************
#  String Styles
# ***************
class StrStyle:
    @staticmethod
    def split(str_: str) -> list:
        pass

    @staticmethod
    def concat(str_list: list) -> str:
        pass

    @staticmethod
    def is_this(str_: str) -> bool:
        pass


class NoneStyle(StrStyle):
    @staticmethod
    def split(str_: str) -> list:
        return []

    @staticmethod
    def concat(str_list: list) -> str:
        return ""

    @staticmethod
    def is_this(str_: str) -> bool:
        return str_ == ""
    

class UpperCamel(StrStyle):
    """
    like SuperMarioBros
    """
    @staticmethod
    def split(str_: str) -> list:
        return "".join([
            char if char.islower() else f" {char}"
            for char in str_
        ]).split()

    @staticmethod
    def concat(str_list: list) -> str:
        return "".join([
            str_.capitalize()
            for str_ in str_list
        ])

    @staticmethod
    def is_this(str_: str) -> bool:
        return "_" not in str_ and str_[0].isupper()


class LowerCamel(StrStyle):
    """
    like superMarioBros
    """
    @staticmethod
    def split(str_: str) -> list:
        return "".join([
            char if char.islower() else f" {char}"
            for char in str_
        ]).split()

    @staticmethod
    def concat(str_list: list) -> str:
        return str_list[0].lower() \
            + "".join([
                str_.capitalize()
                for str_ in str_list[1:]
            ])

    @staticmethod
    def is_this(str_: str) -> bool:
        return "_" not in str_ and str_[0].islower()


class UpperSnake(StrStyle):
    """
    like SUPER_MARIO_BROS
    """
    @staticmethod
    def split(str_: str) -> list:
        return str_.split("_")

    @staticmethod
    def concat(str_list: list) -> str:
        return "_".join([
            str_.upper() for str_ in str_list
        ])

    @staticmethod
    def is_this(str_: str) -> bool:
        return str_.isupper()


class LowerSnake(StrStyle):
    """
    like super_mario_bros
    """
    @staticmethod
    def split(str_: str) -> list:
        return str_.split("_")

    @staticmethod
    def concat(str_list: list) -> str:
        return "_".join([
            str_.lower()
            for str_ in str_list
        ])

    @staticmethod
    def is_this(str_: str) -> bool:
        return str_.islower()


def style_of(str_: str) -> StrStyle:
    STYLES = (
        NoneStyle,
        UpperCamel,
        LowerCamel,
        UpperSnake,
        LowerSnake,
    )
    for style in STYLES:
        if style.is_this(str_):
            return style
    raise Exception(
        f"no StrStyle matching {str_} found"
    )
    

def convert_to(str_: str, style: StrStyle) -> str:
    from_style = style_of(str_)
    str_list = from_style.split(str_)
    return style.concat(str_list)


if __name__ == "__main__":
    from itertools import permutations as perm
    
    test_str = [
        "SuperMarioBros",
        "superMarioBros",
        "SUPER_MARIO_BROS",
        "super_mario_bros",
    ]

    for str0, str1 in perm(test_str, r=2):
        to_style = style_of(str1)
        str_conv = convert_to(str0, to_style)
        is_good = str_conv == str1
        print(
            "OK" if is_good else "NG",
            ":",
            str0,
            "-->",
            str1,
            f"(ans: {str_conv})",
        )
