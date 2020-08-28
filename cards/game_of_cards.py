import random


class Card:
    def __init__(self, suit=None, rank=None, is_joker=False):

        if not self.valid_property(suit, rank, is_joker):
            errmsg = "Invalid initialization of Card"
            raise ValueError(errmsg)
        
        self.suit = suit
        self.rank = rank
        self.is_joker = is_joker

    def valid_property(self, suit, rank, is_joker):
        if is_joker:
            return suit is rank is None
        else:
            SUITS = ("H", "D", "S", "C")
            RANKS = range(1, 14)
            return suit in SUITS and rank in RANKS
    
    def __str__(self):
        if self.is_joker:
            return "JOKER"
        else:
            S_SUITS = {"H":"♡", "D":"♢", "S":"♠", "C":"♣"}
            S_RANKS = "0 A 2 3 4 5 6 7 8 9 10 J Q K".split()
            return S_SUITS[self.suit] + S_RANKS[self.rank]
    
    def __repr__(self):
        return f"card({self.__str__()})"
    
    def is_face_card(self):
        return not self.is_pip_card()

    def is_pip_card(self):
        return self.rank in range(1, 11)

    def color(self):
        if self.suit in ("H", "D"):
            return "RED"
        elif self.suit in ("S", "C"):
            return "BLACK"
        else:
            return None
        
    
class GameOfCards:
    def __init__(self, num_players):
        self.num_players = num_players
        self.stock = []
        self.hands = [[] for _ in range(num_players)]

    # --- Methods for stock ---
    def shuffle(self):
        random.shuffle(self.stock)

    def draw(self, place=0, silent=True):
        """
        By default, you draw a card at the top of 'self.stock'.
        The drawn card is returned.
        If 'place' is specified, the 'place'-th card
        from the top of the stock is drawn.
        """
        if len(self.stock) == 0:
            if not silent:
                print("The stock is empty.")
            return None
        elif place >= len(self.stock):
            if not silent:
                print(f"No card exists at the place {place}.")
            return None
        else:
            return self.stock.pop(place)

    def simple_deck(self, num_joker):
        """
        Return a deck including all cards of four suits and
        thirteen ranks(A,2,3,4,5,6,7,8,9,10,J,Q,K),
        and 'num_joker'(int) jokers.
        """
        if not isinstance(num_joker, int) or num_joker < 0:
            raise TypeError(
                "'num_joker' must be a non-negative int.")
        
        cardlist = [
            Card(suit=suit, rank=rank)
            for suit in ("H", "D", "S", "C")
            for rank in range(1, 14)
        ]
        jokers = [Card(is_joker=True)] * num_joker
        return cardlist + jokers
    
    # --- Methods for hands ---
    def discard(self, player, card=None, index=None):
        if (card is None)^(index is None):
            if card is not None:
                try:
                    self.hands[player].remove(card)
                except ValueError:
                    print(f"The card {card} does not exist "
                          f"in player {player}'s hand.")
                    return None
                else:
                    return card
                
            if index is not None:
                try:
                    card = self.hands[player][index]
                except IndexError:
                    print("IndexError: list index out of range")
                    return None
                else:
                    return self.discard(player, card=card)
                
        else:
            raise ValueError(
                "Only one of 'card' or 'index' "
                "should have some value.")
        

    def add(self, player, card):
        if not isinstance(card, Card):
            raise TypeError(
                "Elements in hand must have type 'Card'.")
        self.hands[player].append(card)
        
        
# ***********
#   SAMPLE
# ***********
# import random ## used in this sample


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
