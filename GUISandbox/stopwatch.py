import datetime as dt
import tkinter as tk
import tkinter.ttk as ttk
from tkinter import messagebox


class StopWatch:
    def __init__(self):
        self.is_active = False
        self.memory = dt.timedelta()
        self.start_time = None

    @property
    def now(self):
        if self.is_active:
            return self.memory + (dt.datetime.now() - self.start_time)
        else:
            return self.memory

    def start_stop(self):
        if self.is_active:
            self.memory += dt.datetime.now() - self.start_time
            self.start_time = None
            self.is_active = False
        else:
            self.start_time = dt.datetime.now()
            self.is_active = True

    def clear(self):
        self.__init__()
            

class StopWatchApp(ttk.Frame):
    def __init__(self, master):
        super().__init__(master)
        master.title("Stop Watch")
        master.geometry("230x130")
        master.minsize(width=230, height=130)
        master.resizable(width=True, height=False)
        master.protocol("WM_DELETE_WINDOW", self.quit_menu)
        self.pack(anchor="center", fill=tk.BOTH, expand=True)
        self.watch = StopWatch()
        self.create_widget()
        self.renew_display()

    def create_widget(self):
        PADX = 10
        PADY = 5
        BUTTON_H = 2
        
        self.time_str = tk.StringVar()
        self.time_str.set(str(self.watch.now))

        self.en = ttk.Entry(
            self, textvar=self.time_str, state="readonly",
            font=("", 20)
        )
        self.en.pack(fill="x", padx=PADX, pady=PADY)

        self.bt_switch = tk.Button(
            self, text="Start/Stop", height=BUTTON_H,
            command=self.watch.start_stop
        )
        self.bt_switch.pack(fill="x", padx=PADX, pady=PADY)

        self.bt_clear = tk.Button(
            self, text="Clear", height=BUTTON_H,
            command=self.watch.clear,
        )
        self.bt_clear.pack(fill="x", padx=PADX, pady=PADY)

    def renew_display(self):
        self.time_str.set(str(self.watch.now))
        self.master.after(10, self.renew_display)

    def quit_menu(self):
        ans = messagebox.askyesno("Quit", "Do you quit StopWatch?")
        if ans:
            self.master.destroy()


if __name__ == "__main__":
    app = StopWatchApp(tk.Tk())
    app.mainloop()
