from random import randint, shuffle
import time


class Card:
    def __init__(self, num):
        if not isinstance(num, int) or not (1 <= num <= 13):
            raise ValueError()
        self.num = num

    def __repr__(self):
        num_str = "0 A 1 2 3 4 5 6 7 8 9 10 J Q K"
        return num_str.split()[self.num]

    def __eq__(self, other):
        return self.num == other.num

    def __ne__(self, other):
        return not self.__eq__(other)

    def __gt__(self, other):
        if (self.num, other.num) == (1, 13):
            return True
        if (self.num, other.num) == (13, 1):
            return False
        return self.num > other.num

    def __lt__(self, other):
        return self.__ne__(other) and not self.__gt__(other)


class Deck(list):
    def __init__(self):
        super().__init__([Card(i) for i in range(1, 13 + 1)])
        shuffle(self)

    def draw(self):
        return self.pop(0)


class GameMaster:
    def __init__(self, turn=3):
        self.turn = turn
        self.deck = Deck()
        self.cpu_hand = None
        self.player_hand = None
        self.trash = []

    def start(self):
        self.__init__()
        self.cpu_hand = self.deck.draw()
        self.player_hand = self.deck.draw()
        print("=== ゲームを開始します ===")
        for _ in range(self.turn):
            self.cpu_play()
            self.player_play()
        self.judge()

    def change_cpu_hand(self):
        self.trash.append(self.cpu_hand)
        self.cpu_hand = self.deck.draw()

    def change_player_hand(self):
        self.trash.append(self.player_hand)
        self.player_hand = self.deck.draw()

    def cpu_play(self):
        print("... CPU 考え中 ...")
        time.sleep(randint(2, 5))
        if randint(0, 1):
            self.change_cpu_hand()
            print("CPU は手札を交換しました。")
        else:
            print("CPU はステイしました。")

    def player_play(self):
        print(f"山札は残り{len(self.deck)}枚です。")
        print("現在のトラッシュ:", self.trash)
        print("あなたの手札:", self.player_hand)
        while True:
            to_change = input("手札を交換しますか？(Y/n)")
            if to_change.lower() in ("y", "n"):
                break
            print("Y または n と入力してください。")
        if to_change.lower() == "y":
            self.change_player_hand()

    def judge(self):
        print("CPU:", self.cpu_hand)
        print("あなた:", self.player_hand)
        if self.player_hand > self.cpu_hand:
            print("あなたの勝ちです。")
        elif self.player_hand < self.cpu_hand:
            print("あなたの負けです。")
        else:
            raise Exception("unexpected error occured")


if __name__ == "__main__":
    master = GameMaster()
    master.start()
