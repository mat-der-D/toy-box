#![allow(unused)]
use std::collections::HashMap;
use std::convert::From;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait ToDescriptiveString {
    fn to_descriptive_string(&self, colored: bool) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum Player {
    First,
    Second,
}

impl Player {
    fn the_other(&self) -> Self {
        use Player::*;
        match self {
            First => Second,
            Second => First,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    ForwardLeft,
    ForwardRight,
    BackwardLeft,
    BackwardRight,
}

impl Direction {
    fn to_dxdy(&self, player: Player) -> [i32; 2] {
        use Player::*;
        let sign = match player {
            First => 1,
            Second => -1,
        };
        use Direction::*;
        match self {
            Forward => [sign * (-1), 0],
            Backward => [sign * 1, 0],
            Left => [0, sign * (-1)],
            Right => [0, sign * 1],
            ForwardLeft => [sign * (-1), sign * (-1)],
            ForwardRight => [sign * (-1), sign * 1],
            BackwardLeft => [sign * 1, sign * (-1)],
            BackwardRight => [sign * 1, sign * 1],
        }
    }

    fn from_dxdy(dxdy: [i32; 2], player: Player) -> Option<Self> {
        use Player::*;
        let dxdy = match player {
            Player::First => dxdy,
            Player::Second => [-dxdy[0], dxdy[1]],
        };
        use Direction::*;
        match dxdy {
            [dx, dy] if (dx.abs() > 1) | (dy.abs() > 1) => None,
            [0, 0] => None,
            [-1, 0] => Some(Forward),
            [1, 0] => Some(Backward),
            [0, -1] => Some(Left),
            [0, 1] => Some(Right),
            [-1, -1] => Some(ForwardLeft),
            [-1, 1] => Some(ForwardRight),
            [1, -1] => Some(BackwardLeft),
            [1, 1] => Some(BackwardRight),
            _ => unreachable!(),
        }
    }
}

// ToDo: write test for from_dxdy, to_dxdy

// --------------------------------------------------
//  Animal
// --------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Animal {
    Chick,
    Hen,
    Elephant,
    Giraffe,
    Lion,
}

impl Animal {
    fn can_move_to(&self, target: Direction) -> bool {
        use Animal::*;
        use Direction::*;
        match self {
            Chick => matches!(target, Forward),
            Hen => !matches!(target, BackwardLeft | BackwardRight),
            Elephant => !matches!(target, Forward | Backward | Left | Right),
            Giraffe => matches!(target, Forward | Backward | Left | Right),
            Lion => true,
        }
    }
}

// --------------------------------------------------
//  Piece
// --------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PieceKind {
    Chicken,
    Elephant,
    Giraffe,
    Lion,
}

impl PieceKind {
    fn get_face(&self, is_promoted: bool) -> Animal {
        match (self, is_promoted) {
            (PieceKind::Chicken, false) => Animal::Chick,
            (PieceKind::Chicken, true) => Animal::Hen,
            (PieceKind::Elephant, _) => Animal::Elephant,
            (PieceKind::Giraffe, _) => Animal::Giraffe,
            (PieceKind::Lion, _) => Animal::Lion,
        }
    }

    fn can_move_to(&self, target: Direction, is_promoted: bool) -> bool {
        self.get_face(is_promoted).can_move_to(target)
    }
}

#[derive(Debug)]
struct Piece {
    kind: PieceKind,
    is_promoted: bool,
    owner: Player,
}

impl Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Animal::*;
        let face_chars = match self.get_face() {
            Chick => ['C', 'c'],
            Hen => ['H', 'h'],
            Elephant => ['E', 'e'],
            Giraffe => ['G', 'g'],
            Lion => ['L', 'l'],
        };
        use Player::*;
        let idx: usize = match self.owner {
            First => 0,
            Second => 1,
        };
        write!(f, "{}", face_chars[idx])
    }
}

impl Piece {
    fn from_char(c: char) -> Option<Piece> {
        use Player::*;
        let owner = match c {
            x if x.is_uppercase() => First,
            x if x.is_lowercase() => Second,
            _ => return None,
        };
        use PieceKind::*;
        let (kind, is_promoted) = match c {
            'C' | 'c' => (Chicken, false),
            'H' | 'h' => (Chicken, true),
            'E' | 'e' => (Elephant, false),
            'G' | 'g' => (Giraffe, false),
            'L' | 'l' => (Lion, false),
            _ => return None,
        };
        Some(Piece {
            kind,
            is_promoted,
            owner,
        })
    }

    fn promote(&mut self) {
        self.is_promoted = true;
    }

    fn demote(&mut self) {
        self.is_promoted = false;
    }

    fn get_face(&self) -> Animal {
        self.kind.get_face(self.is_promoted)
    }

    fn can_move_to(&self, target: Direction) -> bool {
        self.kind.can_move_to(target, self.is_promoted)
    }
}

// --------------------------------------------------
//  Board
// --------------------------------------------------
#[derive(Debug, Clone)]
struct ParseBoardError;

impl fmt::Display for ParseBoardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid string for board")
    }
}

impl error::Error for ParseBoardError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
struct Board {
    board: [[Option<Piece>; 3]; 4],
}

impl Default for Board {
    fn default() -> Self {
        Board::from_str("gle;-c-;-C-;ELG;").unwrap()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_str = String::new();
        for row in self.board.iter() {
            for position in row.iter() {
                let new_str = match position {
                    Some(p) => p.to_string(),
                    None => String::from("-"),
                };
                board_str.push_str(new_str.as_str());
            }
            board_str.push(';');
        }
        write!(f, "{}", board_str)
    }
}

impl FromStr for Board {
    type Err = ParseBoardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sep = ';';
        let s = &*s.replace("\n", "").replace("\r", "").replace(" ", "");
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut board: [[Option<Piece>; 3]; 4] = Default::default();
        for c in s.chars() {
            if c == sep {
                if j == 3 {
                    i += 1;
                    j = 0;
                } else {
                    return Err(ParseBoardError);
                }
            } else {
                if (0..4).contains(&i) & (0..3).contains(&j) {
                    board[i][j] = Piece::from_char(c);
                    j += 1;
                } else {
                    return Err(ParseBoardError);
                }
            }
        }
        if (i, j) != (4, 0) {
            Err(ParseBoardError)
        } else {
            Ok(Board { board })
        }
    }
}

impl ToDescriptiveString for Board {
    fn to_descriptive_string(&self, colored: bool) -> String {
        let sep = String::from(" +-+-+-+");
        let mut str_vec = vec![String::from("  0 1 2"), sep.clone()];
        for i in 0..4 {
            let mut row_str = String::new();
            row_str.push_str(i.to_string().as_str());
            row_str.push('|');
            for position in &self.board[i] {
                match position {
                    Some(p) => {
                        if colored {
                            use Player::*;
                            let color = match p.owner {
                                First => "\x1b[97m\x1b[40m",
                                Second => "\x1b[30m\x1b[47m",
                            };
                            row_str.push_str(color);
                        }
                        row_str.push_str(p.to_string().as_str());
                        if colored {
                            row_str.push_str("\x1b[0m");
                        }
                    }
                    None => row_str.push(' '),
                }
                row_str.push('|');
            }
            str_vec.push(row_str);
            str_vec.push(sep.clone());
        }
        str_vec.join("\n")
    }
}

impl Board {
    fn can_move_from_to_by(&self, from: [usize; 2], to: [usize; 2], by: Player) -> bool {
        let piece_from = match self.board[from[0]][from[1]].as_ref() {
            Some(x) => x,
            None => return false,
        };
        if by != piece_from.owner {
            return false;
        }

        match self.board[to[0]][to[1]].as_ref() {
            Some(Piece { owner, .. }) if *owner == by => return false,
            _ => {}
        };
        let dxdy = [to[0] as i32 - from[0] as i32, to[1] as i32 - from[1] as i32];
        let direction = match Direction::from_dxdy(dxdy, by) {
            Some(d) => d,
            None => return false,
        };
        piece_from.can_move_to(direction)
    }

    fn can_take(&self, pos: [usize; 2]) -> bool {
        match &self.board[pos[0]][pos[1]] {
            Some(piece) => {
                let the_other_player = piece.owner.the_other();
                for x in 0..4 {
                        if self.can_move_from_to_by([x, y], pos, the_other_player) {
                            return true;
                        }
                    }
                }
                false
            }
            None => false,
        }
    }

    fn move_from_to(&mut self, from: [usize; 2], to: [usize; 2]) -> Option<Piece> {
        let mut p: Option<Piece> = None;
        std::mem::swap(&mut self.board[from[0]][from[1]], &mut p);
        std::mem::swap(&mut self.board[to[0]][to[1]], &mut p);
        p
    }

    fn can_drop_to(&self, to: [usize; 2]) -> bool {
        self.board[to[0]][to[1]].is_none()
    }

    fn drop_piece_to(&mut self, piece: Piece, to: [usize; 2]) {
        self.board[to[0]][to[1]] = Some(piece);
    }

    fn frontmost_line(&self, player: Player) -> &[Option<Piece>; 3] {
        use Player::*;
        match player {
            First => &self.board[0],
            Second => &self.board[3],
        }
    }
}

// --------------------------------------------------
//  PiecePools
// --------------------------------------------------
#[derive(Debug)]
struct PiecePools {
    pools: HashMap<Player, Vec<PieceKind>>,
}

impl Default for PiecePools {
    fn default() -> Self {
        use Player::*;
        let mut pools = HashMap::new();
        for player in Player::iter() {
            pools.insert(player, Vec::new());
        }
        PiecePools { pools }
    }
}

impl ToDescriptiveString for PiecePools {
    fn to_descriptive_string(&self, colored: bool) -> String {
        use Player::*;

        fn each_string(self_: &PiecePools, player: Player, colored: bool) -> String {
            let piece_names: Vec<String> = self_.pools[&player]
                .iter()
                .enumerate()
                .map(|(i, p)| format!("{:?}({})", p, i))
                .collect();
            let mut s = String::new();
            if colored {
                match player {
                    First => s.push_str("\x1b[97m\x1b[40m"),
                    Second => s.push_str("\x1b[30m\x1b[47m"),
                };
            }
            s.push_str(format!("{:?}'s Pool: {}", player, piece_names.join(", ")).as_str());
            if colored {
                s.push_str("\x1b[0m");
            }
            s
        }

        Player::iter()
            .map(|p| each_string(self, p, colored))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl PiecePools {
    fn put(&mut self, new_owner: Player, piece: Piece) {
        let kind = piece.kind;
        let mut pool = self.pools.get_mut(&new_owner).unwrap();
        pool.push(kind);
    }

    fn pick_up(&mut self, owner: Player, index: usize) -> Option<Piece> {
        let mut pool = self.pools.get_mut(&owner).unwrap();
        if pool.get(index).is_none() {
            None
        } else {
            let kind = pool.remove(index);
            Some(Piece {
                kind,
                is_promoted: false,
                owner,
            })
        }
    }

    fn get_lion_holder(&self) -> Option<Player> {
        for (player, pool) in self.pools.iter() {
            if pool.contains(&PieceKind::Lion) {
                return Some(player.the_other());
            }
        }
        None
    }
}

// --------------------------------------------------
//  TurnCounter
// --------------------------------------------------
#[derive(Debug)]
struct TurnCounter {
    count: u32,
    player: Player,
}

impl Default for TurnCounter {
    fn default() -> Self {
        TurnCounter {
            count: 1,
            player: Player::First,
        }
    }
}

impl ToDescriptiveString for TurnCounter {
    fn to_descriptive_string(&self, colored: bool) -> String {
        format!("{:?} Player's Turn (count:{})", self.player, self.count)
    }
}

impl TurnCounter {
    fn advance(&mut self) {
        use Player::*;
        match self.player {
            First => self.player = Second,
            Second => {
                self.count += 1;
                self.player = First;
            }
        }
    }
}

// --------------------------------------------------
//  Master
// --------------------------------------------------
#[derive(Debug, Default)]
struct AnimalShogiMaster {
    counter: TurnCounter,
    board: Board,
    pools: PiecePools,
}

impl AnimalShogiMaster {
    fn move_from_to(&mut self, from: [usize; 2], to: [usize; 2]) -> Result<&str, &str> {
        let by = self.counter.player;
        if self.board.can_move_from_to_by(from, to, by) {
            let taken_piece = self.board.move_from_to(from, to);
            if let Some(p) = taken_piece {
                self.pools.put(by, p);
            }
            self.counter.advance();
            Ok("moved")
        } else {
            Err("failed to move")
        }
    }

    fn drop_piece_to(&mut self, index: usize, to: [usize; 2]) -> Result<&str, &str> {
        if !self.board.can_drop_to(to) {
            Err("abc")
        } else {
            let player = self.counter.player;
            let picked_up_piece = self.pools.pick_up(player, index);
            match picked_up_piece {
                Some(p) => {
                    self.board.drop_piece_to(p, to);
                    self.counter.advance();
                    Ok("abc")
                }
                None => Err("abc"),
            }
        }
    }

    fn execute_command(&mut self, command: &str) -> Result<&str, &str> {
        let mut nums = Vec::new();
        for c in command.chars() {
            match c.to_digit(10) {
                Some(n) => nums.push(n as usize),
                None => return Err("failed to parse command"),
            }
        }
        let command_result: Result<&str, &str> = match nums.len() {
            3 => self.drop_piece_to(nums[0], [nums[1], nums[2]]),
            4 => self.move_from_to([nums[0], nums[1]], [nums[2], nums[3]]),
            _ => Err("invalid length of command"),
        };
        command_result
    }

    fn display_status(&self, colored: bool) {
        println!("{}", "-".repeat(32));
        println!("{}", self.counter.to_descriptive_string(colored));
        println!("{}", self.board.to_descriptive_string(colored));
        println!("{}", self.pools.to_descriptive_string(colored));
    }

    fn get_winner(&self) -> Option<Player> {
        let lion_holder = self.pools.get_lion_holder();
        if lion_holder.is_some() {
            return lion_holder;
        }

        for player in Player::iter() {
            let front = self.board.frontmost_line(player);
            for opt_piece in front.iter() {
                if let Some(piece) = opt_piece {
                    if (piece.owner, piece.kind) == (player, PieceKind::Lion) && self.board.can_take()
                }
            }
        }

        // First Win Check

        // Second Win Check

        use Player::*;
        Some(First)
    }
}

// --------------------------------------------------
//  Loop Manager
// --------------------------------------------------
#[derive(Debug, Default)]
struct AnimalShogiUI {
    played: bool,
    master: AnimalShogiMaster,
}

impl AnimalShogiUI {
    fn play(&mut self, ask_colored: bool) {
        if self.played {
            println!("Initialize object before playing.");
            return;
        } else {
            self.played = true;
        }
        let mut input = String::new();
        let mut colored = false; // default value
        if ask_colored {
            loop {
                println!("Do you want to use colored-mode?[Y/n] >");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line.");
                match input
                    .replace("\r\n", "")
                    .replace("\n", "")
                    .to_lowercase()
                    .as_str()
                {
                    "y" => {
                        colored = true;
                        break;
                    }
                    "n" => {
                        colored = false;
                        break;
                    }
                    _ => {
                        println!("please type 'Y' or 'n'");
                        input.clear();
                    }
                }
            }
        }
        input.clear();

        // Main Part
        loop {
            self.master.display_status(colored);
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");
            input = input.replace("\n", "").replace("\r", "");
            let result = self.master.execute_command(input.as_str());
            println!("Command Result: {:?}", result);
            input.clear();
        }
    }
}

fn main() {
    let mut ui = AnimalShogiUI::default();
    ui.play(true);
}
