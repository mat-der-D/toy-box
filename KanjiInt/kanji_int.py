from enum import Enum


# ***************************************************
#  Helper Class
# ***************************************************
class LargeDigit(Enum):
    MAN = ("万", 4)
    OKU = ("億", 8)
    CHO = ("兆", 12)
    KEI = ("京", 16)
    GAI = ("垓", 20)
    JO_ = ("𥝱", 24)
    JOH = ("穣", 28)
    KOH = ("溝", 32)
    KAN = ("澗", 36)
    SEI = ("正", 40)
    SAI = ("載", 44)
    GOK = ("極", 48)
    GGS = ("恒河沙", 52)
    ASG = ("阿僧祇", 56)
    NYT = ("那由他", 60)
    FKS = ("不可思議", 64)
    MRT = ("無量大数", 68)

    def __init__(self, kanji, pow_):
        self.kanji = kanji
        self.num = 10 ** pow_

    @classmethod
    def max_num(cls):
        max_pow = max(digit.value[1] for digit in cls)
        return 10 ** (max_pow + 4) - 1


# ***************************************************
#  Helper Functions
# ***************************************************
def _int_to_kanji_upto9999(n: int) -> str:
    if not isinstance(n, int):
        raise TypeError(f"int_to_kanji() argument must be an integer, not '{type(n)}'")
    if not (0 < n < 10000):
        raise ValueError(f"out of range 0 < n < 10000: {n}")

    def _additional_digit(num_int: int, digit_str: str):
        _one_digit = dict(zip(range(1, 10), [""] + list("二三四五六七八九")))
        return "" if not num_int else _one_digit[num_int] + digit_str

    to_return = ""
    for num, digit in zip(f"{n:0>4}", ("千", "百", "十", "")):
        to_return += _additional_digit(int(num), digit)
    if n % 10 == 1:
        to_return += "一"
    return to_return


def _kanji_to_int_upto9999(kanji: str) -> int:
    _one_digit_1to9 = dict(zip([""] + list("二三四五六七八九"), range(1, 10)))
    _one_digit_0to9 = dict(zip([""] + list("一二三四五六七八九"), range(0, 10)))

    to_return: int = 0
    for digit_str, digit_num in zip("千百十", (1000, 100, 10)):
        i = kanji.find(digit_str)
        if i != -1:
            to_return += _one_digit_1to9[kanji[:i]] * digit_num
            kanji = kanji[i + 1:]
    to_return += _one_digit_0to9[kanji]
    return to_return


# ***************************************************
#  Core Functions
# ***************************************************
def int_to_kanji(n: int) -> str:
    if not isinstance(n, int):
        raise TypeError(f"int_to_kanji() argument must be an integer, not '{type(n)}'")
    if abs(n) > LargeDigit.max_num():
        raise ValueError(f"out of the scope for int_to_kanji(): {n}")

    if not n:
        return "零"
    if n < 0:
        return "負" + int_to_kanji(-n)

    to_return = ""
    for d in reversed(LargeDigit):
        if d.num <= n:
            to_return += _int_to_kanji_upto9999(n // d.num) + d.kanji
            n = n % d.num
    if n:
        to_return += _int_to_kanji_upto9999(n)
    return to_return


def kanji_to_int(kanji: str) -> int:
    if kanji == "零":
        return 0
    if kanji.startswith("負"):
        return - kanji_to_int(kanji[1:])

    to_return: int = 0
    for d in reversed(LargeDigit):
        i = kanji.find(d.kanji)
        if i != -1:
            to_return += _kanji_to_int_upto9999(kanji[:i]) * d.num
            kanji = kanji[i + len(d.kanji):]
    to_return += _kanji_to_int_upto9999(kanji)
    return to_return


def test_consistency(num: int):
    num_kanji = int_to_kanji(num)
    num_returns = kanji_to_int(num_kanji)
    print(num, "\n->", num_kanji, "\n->", num_returns)
    if num == num_returns:
        print("OK!")
    else:
        raise Exception(f"consistency test failed for {num}")


if __name__ == '__main__':
    test_consistency(513098751019438762423409876709237203962309567860234756802398750298762351)
