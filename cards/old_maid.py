import random
from game_of_cards import GameOfCards


class OldMaid(GameOfCards):
    def __init__(self, num_players):
        super().__init__(num_players)
        self.stock = self.simple_deck(num_joker=1)
        self.turn = 0
        self.order = list(range(self.num_players))
        
        self.deal()
    
    def deal(self):
        self.shuffle()
        for n, card in enumerate(self.stock):
            player = n % self.num_players
            if not self.discard_pair(player, card):
                self.add(player, card)
        self.stock.clear()

    def discard_pair(self, player, card):
        for c in self.hands[player]:
            if c.rank == card.rank:
                self.discard(player=player, card=c)
                return True
        return False    
    
    def take_card(self, from_player, to_player, index):
        card = self.discard(from_player, index=index)
        if not self.discard_pair(to_player, card):
            self.add(to_player, card)
            return card, False
        return card, True

    def is_active(self, player):
        return len(self.hands[player]) > 0
    
    def num_active(self):
        active_members = [
            player
            for player in self.order
            if self.is_active(player)
        ]
        return len(active_members)

    def next_player(self):
        followers = self.order[self.turn:] + self.order[:self.turn]
        for n in range(1, self.num_players):
            if self.is_active(followers[n]):
                return followers[n]
        return None

    # ----- main part -----
    def play(self):
        while self.num_active() >= 2:
            self.play_one_turn()

        print("----- GAME SET! -----")
        for player in self.order:
            print(f"Player {player}: {self.hands[player]}")

    def play_one_turn(self):
        player = self.order[self.turn]
        next_player = self.next_player()
        if self.is_active(player) and next_player is not None:
            print(f"Player {player}'s turn!")

            # Select which card to take
            len_hand = len(self.hands[next_player])
            index = random.randint(0, len_hand-1)

            # Take a card
            card, discarded = \
                self.take_card(next_player, player, index)
            if discarded:
                print(f"Player {player} discarded "
                      f"a pair including {card}!")

            # victory judgement
            if not self.is_active(next_player):
                print(f"Player {next_player} won!")
            if not self.is_active(player):
                print(f"Player {player} won!")

        # turn increment
        self.turn = (self.turn + 1) % self.num_players
        

if __name__ == "__main__":
    old_maid = OldMaid(4)
    old_maid.play()
