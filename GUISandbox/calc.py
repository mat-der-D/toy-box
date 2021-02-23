import tkinter as tk
import tkinter.ttk as ttk


class Calc(tk.Frame):
    def __init__(self, master):
        super().__init__(master)
        self.pack()
        master.resizable(width=False, height=False)
        master.title("Calc")
        self.is_protected = False
        self.create_widget()

    def create_widget(self):
        V_HEIGHT = 2
        ERR_HEIGHT = 1

        # *** variables ***
        self.err_str = tk.StringVar()
        self.err_str.set("")        
        self.view_str = tk.StringVar()
        self.view_str.set("0")

        # ** view ***
        self.err_view = tk.Label(
            self, font=("", 14),
            textvariable=self.err_str,
            anchor="e",
            height=ERR_HEIGHT,
        )
        self.err_view.grid(column=0, row=0, columnspan=4, sticky="EW")
        
        self.view = tk.Label(
            self, font=("", 20),
            textvariable=self.view_str,
            anchor="e",
            height=V_HEIGHT,
        )
        self.view.grid(column=0, row=1, columnspan=4, sticky="EW")

        KEY_INIT_ROW = 2
        key_config = (
            ("ClearKey", "C", 0, 0),
            ("CErrKey", "CE", 1, 0),
            ("DelKey", "Del", 2, 0),
            ("OpKey",    "/", 3, 0),
            ("NumKey",   "7", 0, 1),
            ("NumKey",   "8", 1, 1),
            ("NumKey",   "9", 2, 1),
            ("OpKey",    "*", 3, 1),
            ("NumKey",   "4", 0, 2),
            ("NumKey",   "5", 1, 2),
            ("NumKey",   "6", 2, 2),
            ("OpKey",    "-", 3, 2),
            ("NumKey",   "1", 0, 3),
            ("NumKey",   "2", 1, 3),
            ("NumKey",   "3", 2, 3),
            ("OpKey",    "+", 3, 3),
            ("NumKey",   "0", 0, 4),
            ("ZZKey",   "00", 1, 4),
            ("OpKey",    ".", 2, 4),
            ("EqKey",    "=", 3, 4),
        )
        for idx, config in enumerate(key_config):
            key_type, symbol, column, row = config
            row += KEY_INIT_ROW
            name = f"btn_{idx}"
            setattr(self, name, getattr(self, key_type)(self, symbol))
            getattr(self, name).grid(column=column, row=row)

    class Key(tk.Button):
        WIDTH = 3
        HEIGHT = 3
        FONTSIZE = 14
        DIGITS = 18
        def __init__(self, frame, symbol):
            self.frame = frame
            self.symbol = symbol
            super().__init__(
                frame, text=symbol, command=self.press,
                width=self.WIDTH, height=self.HEIGHT,
                font=("", self.FONTSIZE),
            )

        def press(self):
            raise NotImplementedError()
            
    class NumKey(Key):
        def press(self):
            if self.frame.is_protected:
                return
            
            s = self.frame.view_str.get()
            if s == "0":
                self.frame.view_str.set(self.symbol)
            else:
                s_new = s + self.symbol
                if len(s_new) <= self.DIGITS:
                    self.frame.view_str.set(s_new)

    class ZZKey(Key):
        def press(self):
            if self.frame.is_protected:
                return

            s = self.frame.view_str.get()
            if s != "0":
                s_new = s + self.symbol
                if len(s_new) <= self.DIGITS:
                    self.frame.view_str.set(s_new)
                
    class OpKey(Key):
        def press(self):
            if self.frame.err_str.get() == "Answer":
                self.frame.is_protected = False
                self.frame.err_str.set("")
            if self.frame.is_protected:
                return
            
            s_new = self.frame.view_str.get() + self.symbol
            if len(s_new) <= self.DIGITS:
                self.frame.view_str.set(s_new)

    class EqKey(Key):
        def press(self):
            if self.frame.is_protected:
                return
            
            s = self.frame.view_str.get()
            try:
                result = eval(s)
            except (SyntaxError, ZeroDivisionError) as err:
                self.frame.err_str.set(err.__class__.__name__)
            else:
                self.frame.view_str.set(str(result))
                self.frame.err_str.set("Answer")
            finally:
                self.frame.is_protected = True

    class ClearKey(Key):
        def press(self):
            self.frame.is_protected = False
            self.frame.view_str.set("0")
            self.frame.err_str.set("")

    class CErrKey(Key):
        def press(self):
            self.frame.is_protected = False
            self.frame.err_str.set("")

    class DelKey(Key):
        def press(self):
            if self.frame.is_protected:
                return

            s = self.frame.view_str.get()
            if len(s) == 1:
                self.frame.view_str.set("0")
            else:
                self.frame.view_str.set(s[:-1])

if __name__ == "__main__":
    calc = Calc(tk.Tk())
    calc.mainloop()
