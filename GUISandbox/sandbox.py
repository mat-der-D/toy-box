import tkinter as tk
import tkinter.ttk as ttk
from tkinter import messagebox


class AskNameApp:
    def __init__(self, master):
        self.master = master
        master.title("Your Name.")
        master.protocol("WM_DELETE_WINDOW", self.press_x)
        master.resizable(width=False, height=False)
        PADX = 20
        PADY = 5
        
        self.frame_given = self.NameFrame(
            master, "Your Given Name", "Ichiro", "Other:"
        )
        self.frame_given.pack(padx=PADX, pady=PADY, expand=True)

        self.frame_family = self.NameFrame(
            master, "Your Family Name", "Suzuki", "Other:"
        )
        self.frame_family.pack(padx=PADX, pady=PADY, expand=True)
        
        self.frame_button = self.ButtonFrame(
            master, self.frame_family.answer, self.frame_given.answer
        )
        self.frame_button.pack(padx=PADX, pady=PADY, expand=True)
        
    def press_x(self):
        messagebox.showerror("WHAT!?", "Nooooooooooo!!!!!!!!")
        self.master.destroy()
        
    class NameFrame(ttk.Frame):
        def __init__(self, master, question, default, entry_msg):
            super().__init__(master)
            self.default = default
            self.create_widget(question, default, entry_msg)
            for icol in range(3):
                self.grid_columnconfigure(icol, weight=1)
            for irow in range(3):
                self.grid_rowconfigure(icol, weight=1)

        def create_widget(self, question, default, entry_msg):
            # --- Question ---
            self.q_label = ttk.Label(self, text=question)
            self.q_label.grid(column=0, row=0, columnspan=3)

            # --- Radio Button ---
            self.radio_var = tk.StringVar()
            self.radio_var.set("default")            

            self.radio0 = tk.Radiobutton(
                self, value="default", variable=self.radio_var
            )
            self.radio0.configure(
                command=lambda: self.set_entry_state("disabled")
            )
            self.radio0.grid(column=0, row=1)

            self.radio1 = tk.Radiobutton(
                self, value="manual", variable=self.radio_var
            )
            self.radio1.configure(
                command=lambda: self.set_entry_state("normal")
            )
            self.radio1.grid(column=0, row=2)

            # --- Text ---
            self.label0 = ttk.Label(self, text=default)
            self.label0.grid(column=1, row=1, columnspan=2, sticky=tk.W)
            self.label1 = ttk.Label(self, text=entry_msg)
            self.label1.grid(column=1, row=2, sticky=tk.W)

            self.entry_var = tk.StringVar()
            self.entry = ttk.Entry(self, state="disabled", textvar=self.entry_var)
            self.entry.grid(column=2, row=2, sticky=tk.W+tk.E)

        def set_entry_state(self, new_state):
            self.entry["state"] = new_state
            
        def answer(self):
            if self.radio_var.get() == "default":
                return self.default
            else:
                return self.entry_var.get()
 
    class ButtonFrame(ttk.Frame):
        def __init__(self, master, func_family, func_given):            
            super().__init__(master)
            self.func_family = func_family
            self.func_given = func_given
            self.create_widget()

        def create_widget(self):
            self.bt_ok = ttk.Button(self, text="OK", command=self.press_ok)
            self.bt_ok.pack(padx=10, side="left")
            self.bt_cancel = ttk.Button(self, text="Cancel", command=self.press_cancel)
            self.bt_cancel.pack(padx=20, side="right")

        def press_ok(self):
            full_name = self.func_given() + " " + self.func_family()
            messagebox.showinfo("Your name is...", full_name)

        def press_cancel(self):
            self.master.destroy()


if __name__ == "__main__":
    root = tk.Tk()
    app = AskNameApp(root)
    app.master.mainloop()
