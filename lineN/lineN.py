from itertools import product
import numpy as np


class Board(object):
    def __init__(self, dim, disp):
        self.board = [
            [[None]*dim for _ in range(dim)]
            for _ in range(dim)
        ]
        self.d = dim
        self.disp = disp
        
    def put_stone(self, position, color):
        try:
            x, y = position
            pole = self.board[x][y]
        except ValueError:
            if self.disp:
                print("ERROR! " \
                      "Position should be (x, y) or [x, y].")
        except IndexError:
            if self.disp:
                print("ERROR! " \
                      "Position (x, y) should be " \
                      "included in 0-{}.".format(self.d-1))
        except:
            if self.disp:
                print("ERROR! Something went wrong.")
        else:
            return self.put_stone_pole(self.board[x][y], color)

    def put_stone_pole(self, pole, color):
        if color is None:
            if self.disp:
                print("ERROR! 'color' should not be None.")
        else:
            for n in range(self.d):
                if pole[n] is None:
                    pole[n] = color
                    return True
            if self.disp:
                print("This pole is full!")
            return False


class Judge(object):
    def __init__(self, dim):
        self.d = dim
    
    def extract_lines(self, board):
        lines_list = []
        
        # length 1 line
        lines_list.extend([
            [board[x][y][z] for x in range(self.d)]
            for y, z in product(range(self.d), repeat=2)])
        lines_list.extend([
            [board[x][y][z] for y in range(self.d)]
            for x, z in product(range(self.d), repeat=2)])
        lines_list.extend([
            [board[x][y][z] for z in range(self.d)]
            for x, y in product(range(self.d), repeat=2)])
        # length sqrt(2) line
        lines_list.extend([
            [board[x][x][y] for x in range(self.d)]
            for y in range(self.d)])
        lines_list.extend([
            [board[x][self.d-1-x][y] for x in range(self.d)]
            for y in range(self.d)])
        lines_list.extend([
            [board[x][y][x] for x in range(self.d)]
            for y in range(self.d)])
        lines_list.extend([
            [board[x][y][self.d-1-x] for x in range(self.d)]
            for y in range(self.d)])
        lines_list.extend([
            [board[y][x][x] for x in range(self.d)]
            for y in range(self.d)])
        lines_list.extend([
            [board[y][x][self.d-1-x] for x in range(self.d)]
            for y in range(self.d)])
        # length sqrt(3) line
        lines_list.extend([
            [board[x][x][x] for x in range(self.d)],
            [board[x][x][self.d-1-x] for x in range(self.d)],
            [board[x][self.d-1-x][x] for x in range(self.d)],
            [board[self.d-1-x][x][x] for x in range(self.d)]])

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
    def __init__(self, dim=4, player1="BB", player2="WW", disp=True):
        self.d = dim
        self.b = Board(dim, disp)
        self.j = Judge(dim)
        self.finished = False
        self.turn = player1
        self.dict_players = {
            player1: player2,
            player2: player1
            }
        self.winner = None
        self.disp = disp
        self.players = [player1, player2]
        
        if self.disp:
            print(self)
            self.show_turn()

    def restart_game(self):
        self.__init__(self.d, self.players[0], self.players[1],
                      self.disp)
            
    def change_turn(self):
        self.turn = self.dict_players[self.turn]

    def show_turn(self):
        if self.finished:
            print("The game has already finished. " \
                  "The winner is {}.".format(self.winner))
        else:
            print("It is {}'s turn.".format(self.turn))

    def show_board(self):
        print(self)

    def put_stone(self, x, y):
        if self.finished and self.disp:
            print("The game has already finished. " \
                  "The winner is {}.".format(self.winner))
        else:
            put = self.b.put_stone((x, y), self.turn)
            if put:
                if self.disp:
                    self.show_board()
                self.winner = self.j.winner(self.b.board)
                if self.winner is not None:
                    if self.disp:
                        print("{} win!".format(self.winner))
                    self.finished = True
                if not self.finished:
                    self.change_turn()
                    if self.disp:
                        self.show_turn()

    def help(self):
        explanation = \
        "g = Game()        : start game \n"            \
        "g.put_stone(x, y) : put stone at (x, y) \n"   \
        "g.restart_game()  : restart game \n"          \
        "g.show_board()    : show board \n"            \
        "g.show_turn()     : show who's turn it is \n" \
        "g.show()          : show this"
        print(explanation)

    def __repr__(self):
        v_board = [[[
            self.b.board[x][y][z]
            for x in range(self.d)]
            for y in range(self.d-1, -1, -1)]
            for z in range(self.d-1, -1, -1)]
        return str(np.array(v_board))

if __name__ == "__main__":
    print("""
    Let's begin the game!
    Type 'g = Game()' and press Enter to start.
    To put the stone, type 'g.put_stone(x, y)' and press Enter.
    To restart the game, type 'g.restart_game()' and press Enter.
    For more information, press g.help() (not implemented yet).
    """)
