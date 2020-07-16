from itertools import product
import numpy as np


class Board(object):
    def __init__(self):
        self.board = [
            [[None]*4 for _ in range(4)]
            for _ in range(4)
        ]

    def put_stone(self, position, color):
        try:
            x, y = position
            pole = self.board[x][y]
        except ValueError:
            print("ERROR! " \
                  "Position should be (x, y) or [x, y].")
        except IndexError:
            print("ERROR! " \
                  "Position (x, y) should be included in 0-3.")
        except:
            print("ERROR! Something went wrong.")
        else:
            return self.put_stone_pole(self.board[x][y], color)

    def put_stone_pole(self, pole, color):
        if color is None:
            print("ERROR! 'color' should not be None.")
        else:
            for n in range(4):
                if pole[n] is None:
                    pole[n] = color
                    return True
            print("This pole is full!")
            return False


class Judge(object):
    def extract_lines(self, board):
        lines_list = []

        # length 1 line
        lines_list.extend([
            [board[x][y][z] for x in range(4)]
            for y, z in product(range(4), repeat=2)])
        lines_list.extend([
            [board[x][y][z] for y in range(4)]
            for x, z in product(range(4), repeat=2)])
        lines_list.extend([
            [board[x][y][z] for z in range(4)]
            for x, y in product(range(4), repeat=2)])
        # length sqrt(2) line
        lines_list.extend([
            [board[x][x][y] for x in range(4)]
            for y in range(4)])
        lines_list.extend([
            [board[x][3-x][y] for x in range(4)]
            for y in range(4)])
        lines_list.extend([
            [board[x][y][x] for x in range(4)]
            for y in range(4)])
        lines_list.extend([
            [board[x][y][3-x] for x in range(4)]
            for y in range(4)])
        lines_list.extend([
            [board[y][x][x] for x in range(4)]
            for y in range(4)])
        lines_list.extend([
            [board[y][x][3-x] for x in range(4)]
            for y in range(4)])
        # length sqrt(3) line
        lines_list.extend([
            [board[x][x][x] for x in range(4)],
            [board[x][x][3-x] for x in range(4)],
            [board[x][3-x][x] for x in range(4)],
            [board[3-x][x][x] for x in range(4)]])

        return lines_list

    def ocupy_component(self, x_list):
        N = len(x_list)
        if x_list == [x_list[0]]*N:
            return x_list[0]
        else:
            return None

    def winner(self, board):
        l_list = self.extract_lines(board)
        w_set = set(map(self.ocupy_component, l_list))
        w_set.remove(None)
        N = len(w_set)
        if N == 0:
            return None
        elif N == 1:
            return w_set.pop()
        else:
            raise Exception(
                "The board status is invalid; " \
                "more than 2 players make line(s).")

        
class Game(object):
    def __init__(self):
        self.b = Board()
        self.j = Judge()
        self.finished = False
        self.turn = "Black"
        self.dict_players = {
            "Black": "White",
            "White": "Black"
            }
        self.winner = None
        print(self)
        self.show_turn()

    def change_turn(self):
        self.turn = self.dict_players[self.turn]

    def show_turn(self):
        if self.finished:
            print("The game has already finished. " \
                  "The winner is {}.".format(self.winner))
        else:
            print("It is {}'s turn.".format(self.turn))

    def put_stone(self, x, y):
        if self.finished:
            print("The game has already finished. " \
                  "The winner is {}.".format(self.winner))
        else:
            put = self.b.put_stone((x, y), self.turn)
            if put:
                print(self)
                self.winner = self.j.winner(self.b.board)
                if self.winner is not None:
                    print("{} win!".format(self.winner))
                    self.finished = True
                if not self.finished:
                    self.change_turn()
                    self.show_turn()

    def restart_game(self):
        self.__init__()

    def help(self):
        print("""
        Sorry, it has not been implemented yet...
        """)

    def __repr__(self):
        v_board = [[[
            self.b.board[x][y][z]
            for x in range(4)]
            for y in range(4)]
            for z in range(4)]
        return str(np.array(v_board))

if __name__ == "__main__":
    print("""
    Let's begin the game!
    Type 'g = Game()' and press Enter to start.
    To put the stone, type 'g.put_stone(x, y)' and press Enter.
    To restart the game, type 'g.restart_game()' and press Enter.
    For more information, press g.help() (not implemented yet).
    """)
