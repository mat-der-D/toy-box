import tkinter as tk
from tkinter import messagebox
import numpy as np
import pandas as pd
from pandastable import Table


class TableView(tk.Frame):
    def __init__(self, master, dataframe):
        super().__init__(master)
        master.geometry("350x230")
        self.pack(fill=tk.BOTH, expand=True)
        self._df = dataframe
        self.df = self._df.copy()

        self.create_widget()

    def create_widget(self):
        self.bt = tk.Button(
            self, text="Show Table",
            command=self.show_table
        )
        self.bt.pack(fill=tk.BOTH, expand=True)

    def show_table(self):
        self.tbl_window = tk.Toplevel(self.master)
        frame = tk.Frame(self.tbl_window)
        frame.pack(fill=tk.BOTH, expand=True)

        self.tbl = Table(frame, dataframe=self.df)
        self.tbl.showIndex()
        self.tbl.show()

        frame_reset = tk.Frame(self.tbl_window)
        frame_reset.pack(fill=tk.BOTH, expand=True)
        btn = tk.Button(
            frame_reset, text="Reset DataFrame", command=self.reset_df
        )
        btn.pack()


    def reset_df(self):
        if messagebox.askyesno("Confirmation", "Do you really reset?"):
            self.df = self._df.copy()
            self.tbl_window.destroy()
            self.show_table()
        

if __name__ == "__main__":
    df = pd.DataFrame(
        np.arange(100*50).reshape(100, 50),
        columns=["col" + str(i) for i in range(50)],
        index=["idx" + str(i) for i in range(100)],
    )
    tv = TableView(tk.Tk(), dataframe=df)
    tv.mainloop()
