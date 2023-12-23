from enum import Enum, auto
import flet as ft
import random


class StoneColor(Enum):
    BLACK = auto()
    WHITE = auto()
    NONE = auto()


class Stone(ft.Container):
    def __init__(self, color: StoneColor):
        self.color = color
        bgcolor = {
            StoneColor.BLACK: ft.colors.BLACK,
            StoneColor.WHITE: ft.colors.WHITE,
            StoneColor.NONE: ft.colors.TRANSPARENT,
        }[color]
        super().__init__(
            width=80,
            height=20,
            bgcolor=bgcolor,
        )


class StoneTower(ft.UserControl):
    def __init__(self):
        super().__init__()
        self._tower = [Stone(StoneColor.NONE) for _ in range(3)]

    def build(self):
        return ft.Column(
            controls=self._tower,
            horizontal_alignment=ft.CrossAxisAlignment.CENTER,
            spacing=5,
        )

    @property
    def num_stones(self) -> int:
        return sum(s.color is not StoneColor.NONE for s in self._tower)

    def put_stone(self, color: StoneColor):
        if color is StoneColor.NONE:
            return
        for n in range(2, -1, -1):
            if self._tower[n].color is not StoneColor.NONE:
                continue
            self._tower[n] = Stone(color)
            break
        self.update()

    def remove_stone(self) -> StoneColor | None:
        for n in range(2, -1, -1):
            if self._tower[n].color is StoneColor.NONE:
                continue
            color = self._tower[n].color
            self._tower[n] = Stone(StoneColor.NONE)
            return color
        self.update()
        return None

    def clear_stones(self):
        self._tower.clear()
        self._tower.extend(Stone(StoneColor.NONE) for _ in range(3))
        self.update()


class ButtonMode(Enum):
    ADD = auto()
    RESET = auto()


class StoneButton(ft.FilledButton):
    def __init__(self, mode, on_click_add, on_click_reset):
        super().__init__()
        self.width = 120
        self.on_click_add = on_click_add
        self.on_click_reset = on_click_reset
        self.mode = mode
        self.change_mode(mode)

    def change_mode(self, mode: ButtonMode):
        match mode:
            case ButtonMode.ADD:
                self.on_click = self.on_click_add
                self.text = "ADD"
            case ButtonMode.RESET:
                self.on_click = self.on_click_reset
                self.text = "REMOVE"


class StoneTowerButton(ft.UserControl):
    def __init__(self, active: bool):
        super().__init__()
        self._stone_tower = StoneTower()
        self._controls = [self._stone_tower]
        if active:
            btn = StoneButton(
                ButtonMode.ADD,
                on_click_add=self.add_click,
                on_click_reset=self.reset_click,
            )
            self._controls += [btn]

    def build(self):
        return ft.Column(
            controls=self._controls,
            horizontal_alignment=ft.CrossAxisAlignment.CENTER,
            spacing=5,
        )

    def add_click(self, _e):
        stone_color = [StoneColor.BLACK, StoneColor.WHITE][random.randint(0, 1)]
        self._stone_tower.put_stone(stone_color)
        if self._stone_tower.num_stones == 3 and len(self._controls) >= 2:
            btn = self._controls[-1]
            if isinstance(btn, StoneButton):
                btn.change_mode(ButtonMode.RESET)
        self.update()

    def reset_click(self, _e):
        self._stone_tower.clear_stones()
        if len(self._controls) >= 2:
            btn = self._controls[-1]
            if isinstance(btn, StoneButton):
                btn.change_mode(ButtonMode.ADD)
        self.update()


def create_board():
    alignment = ft.MainAxisAlignment.SPACE_EVENLY
    layout = [
        [0, 1, 1, 0],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [0, 1, 1, 0],
    ]
    board = ft.Column([
        ft.Row([
            StoneTowerButton(bool(x)) for x in row
        ], alignment=alignment)
        for row in layout
    ], spacing=20)
    return board


class Application:
    def __call__(self, page: ft.Page):
        page.window_width = 600
        page.window_height = 600
        page.bgcolor = ft.colors.GREY
        page.vertical_alignment = ft.MainAxisAlignment.CENTER
        page.horizontal_alignment = ft.CrossAxisAlignment.CENTER

        board = create_board()
        page.add(board)


def main():
    ft.app(Application())


if __name__ == '__main__':
    main()
