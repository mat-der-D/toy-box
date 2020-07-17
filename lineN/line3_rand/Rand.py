import os
from random import *
import sys

sys.path.append(os.path.abspath(".."))

from lineN import Game



if __name__ == '__main__':
    g = Game(dim=3, disp=False, player1=0, player2=1)
    win_num = [0, 0]
    
    for n in range(1000000):
        if n % 1000 == 0:
            print("n={}".format(n))
        if n % 100000 == 0:
            print("---------- n={} ----------".format(n))
        g.restart_game()
        
        while g.winner is None:
            x, y = randint(0, 3), randint(0, 3)
            g.put_stone(x, y)

        win_num[g.winner] += 1

    print(win_num)
    print("{}%".format(100*win_num[0]/sum(win_num)))
