// #![allow(unused)]
use std::collections::HashMap;
use std::convert::From;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait ToDescriptiveString {
    fn to_descriptive_string(&self, colored: bool) -> String;
}

// ---------------------------------------------------
//  Player
// ---------------------------------------------------
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
enum Player {
    First,
    Second,
}

// impl Player {
//     fn the_other(&self) -> Self {
//         use Player::*;
//         match self {
//             First => Second,
//             Second => First,
//         }
//     }
// }

// ---------------------------------------------------
//  Direction
// ---------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    fn reversed(&self) -> Self {
        use Direction::*;
        match self {
            Forward => Backward,
            Backward => Forward,
            Left => Right,
            Right => Left,
            ForwardLeft => BackwardRight,
            ForwardRight => BackwardLeft,
            BackwardLeft => ForwardRight,
            BackwardRight => ForwardLeft,
        }
    }
}

impl TryFrom<(i32, i32)> for Direction {
    type Error = &'static str;
    fn try_from(dxdy: (i32, i32)) -> Result<Self, Self::Error> {
        use Direction::*;
        match dxdy {
            (-1, 0) => Ok(Forward),
            (1, 0) => Ok(Backward),
            (0, -1) => Ok(Left),
            (0, 1) => Ok(Right),
            (-1, -1) => Ok(ForwardLeft),
            (-1, 1) => Ok(ForwardRight),
            (1, -1) => Ok(BackwardLeft),
            (1, 1) => Ok(BackwardRight),
            _ => Err("no corresponding direction"),
        }
    }
}

impl From<Direction> for (i32, i32) {
    fn from(direction: Direction) -> Self {
        use Direction::*;
        match direction {
            Forward => (-1, 0),
            Backward => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
            ForwardLeft => (-1, -1),
            ForwardRight => (-1, 1),
            BackwardLeft => (1, -1),
            BackwardRight => (1, 1),
        }
    }
}

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
            Hen => !matches!(target, ForwardLeft | ForwardRight),
            Elephant => !matches!(target, Forward | Backward | Left | Right),
            Giraffe => matches!(target, Forward | Backward | Left | Right),
            Lion => true,
        }
    }
}

// ---------------------------------------------------
//  Piece
// ---------------------------------------------------
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

    fn get_face(&self) -> Animal {
        self.kind.get_face(self.is_promoted)
    }

    fn is_kind_of(&self, kind: PieceKind) -> bool {
        self.kind == kind
    }

    fn is_owned_by(&self, player: Player) -> bool {
        self.owner == player
    }

    fn can_move_to(&self, target: Direction) -> bool {
        self.get_face().can_move_to(target)
    }

    fn can_move_from_to(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        use Player::*;
        let dxdy = match self.owner {
            First => (to.0 as i32 - from.0 as i32, to.1 as i32 - from.1 as i32),
            Second => (from.0 as i32 - to.0 as i32, from.1 as i32 - to.1 as i32),
        };
        let target = match (Direction::try_from(dxdy), self.owner) {
            (Ok(d), First) => d,
            (Ok(d), Second) => d.reversed(),
            (Err(_), _) => return false,
        };
        self.can_move_to(target)
    }
}

// ---------------------------------------------------
//  Board
// ---------------------------------------------------
#[derive(Debug)]
struct Board {
    data: [[Option<Piece>; 3]; 4],
}

impl Default for Board {
    fn default() -> Self {
        Board::from_str("gle;-c-;-C-;ELG;").unwrap()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_str = String::new();
        for row in self.data.iter() {
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
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sep = ';';
        let s = &*s.replace("\n", "").replace("\r", "").replace(" ", "");
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut board_data: [[Option<Piece>; 3]; 4] = Default::default();
        for c in s.chars() {
            if c == sep {
                if j == 3 {
                    i += 1;
                    j = 0;
                } else {
                    return Err("ParseBoardError");
                }
            } else {
                if (0..4).contains(&i) & (0..3).contains(&j) {
                    board_data[i][j] = Piece::from_char(c);
                    j += 1;
                } else {
                    return Err("ParseBoardError");
                }
            }
        }
        if (i, j) != (4, 0) {
            Err("ParseBoardError")
        } else {
            Ok(Board { data: board_data })
        }
    }
}

impl ToDescriptiveString for Board {
    fn to_descriptive_string(&self, colored: bool) -> String {
        let sep = String::from(" +-+-+-+");
        let mut str_vec = vec![String::from("  0 1 2"), sep.clone()];
        for i in 0_usize..4_usize {
            let mut row_str = String::new();
            row_str.push_str(i.to_string().as_str());
            row_str.push('|');
            for position in &self.data[i] {
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
    fn get_cell_at(&self, (x, y): (usize, usize)) -> &Option<Piece> {
        &self.data[x][y]
    }

    fn get_mut_cell_at(&mut self, (x, y): (usize, usize)) -> &mut Option<Piece> {
        self.data.get_mut(x).unwrap().get_mut(y).unwrap()
    }

    fn get_cells_around(&self, (x, y): (usize, usize)) -> HashMap<(usize, usize), &Option<Piece>> {
        let mut d = HashMap::new();
        let x_min = if x == 0 { 0 } else { x - 1 };
        let x_max = if x == 3 { 3 } else { x + 1 };
        let y_min = if y == 0 { 0 } else { y - 1 };
        let y_max = if y == 2 { 2 } else { y + 1 };
        for ix in x_min..=x_max {
            for iy in y_min..=y_max {
                if (ix, iy) != (x, y) {
                    d.insert((ix, iy), self.get_cell_at((ix, iy)));
                }
            }
        }
        d
    }

    fn set_piece_at(&mut self, piece: Piece, (x, y): (usize, usize)) {
        self.data[x][y] = Some(piece);
    }

    fn move_piece_from_to(&mut self, from: (usize, usize), to: (usize, usize)) -> Option<Piece> {
        let mut p: Option<Piece> = None;
        std::mem::swap(&mut self.data[from.0][from.1], &mut p);
        std::mem::swap(&mut self.data[to.0][to.1], &mut p);
        p
    }
}

// ---------------------------------------------------
//  Pool
// ---------------------------------------------------
#[derive(Debug)]
struct Pools {
    data: HashMap<Player, Vec<PieceKind>>,
}

impl Default for Pools {
    fn default() -> Self {
        let mut data = HashMap::new();
        for player in Player::iter() {
            data.insert(player, Vec::new());
        }
        Pools { data }
    }
}

impl ToDescriptiveString for Pools {
    fn to_descriptive_string(&self, colored: bool) -> String {
        use Player::*;

        fn each_string(self_: &Pools, player: Player, colored: bool) -> String {
            let piece_names: Vec<String> = self_
                .get_pool_of(player)
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

impl Pools {
    fn get_pool_of(&self, owner: Player) -> &Vec<PieceKind> {
        self.data.get(&owner).unwrap()
    }

    fn get_mut_pool_of(&mut self, owner: Player) -> &mut Vec<PieceKind> {
        self.data.get_mut(&owner).unwrap()
    }

    fn add(&mut self, new_owner: Player, piece: Piece) {
        self.get_mut_pool_of(new_owner).push(piece.kind);
    }

    fn remove(&mut self, owner: Player, index: usize) -> Option<Piece> {
        let pool = self.get_mut_pool_of(owner);
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
    fn to_descriptive_string(&self, _colored: bool) -> String {
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
    pools: Pools,
}

impl AnimalShogiMaster {
    fn move_piece_from_to(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
    ) -> Result<String, String> {
        // Validation
        if (from.0 >= 4) | (from.1 >= 3) | (to.0 >= 4) | (to.1 >= 3) {
            return Err("index out of bounds".to_string());
        };

        let by = self.counter.player;
        let cell_from = self.board.get_cell_at(from);
        match cell_from {
            None => return Err("no piece exists".to_string()),
            Some(piece) if !piece.is_owned_by(by) => {
                return Err("piece is not controllable".to_string())
            }
            _ => {}
        };
        let cell_to = self.board.get_cell_at(to);
        if matches!(cell_to, Some(piece) if piece.is_owned_by(by)) {
            return Err("target place is occupied".to_string());
        };
        if !matches!(cell_from, Some(piece) if piece.can_move_from_to(from, to)) {
            return Err("the piece cannot move in that direction".to_string());
        };

        // Execution
        let mut message = String::from("piece was successfully moved");
        let taken_piece = self.board.move_piece_from_to(from, to);
        if taken_piece.is_some() {
            self.pools.add(by, taken_piece.unwrap());
            message += "; a piece was taken";
        }
        use Player::*;
        if matches!((to.0, self.counter.player), (0, First) | (3, Second)) {
            let moved_piece = self.board.get_mut_cell_at(to).as_mut().unwrap();
            if moved_piece.get_face() == Animal::Chick {
                moved_piece.promote();
                message += "; Chick was promoted to Hen";
            };
        };
        self.counter.advance();
        Ok(message)
    }

    fn drop_piece_to(&mut self, index: usize, to: (usize, usize)) -> Result<String, String> {
        // Validation
        if (to.0 >= 4) | (to.1 >= 3) {
            return Err("index out of bounds".to_string());
        };
        if self.board.get_cell_at(to).is_some() {
            return Err("target place is occupied".to_string());
        };

        // Execution
        let by = self.counter.player;
        let opt_piece = self.pools.remove(by, index);
        match opt_piece {
            None => Err("no piece found in the pool".to_string()),
            Some(piece) => {
                self.board.set_piece_at(piece, to);
                self.counter.advance();
                Ok("piece was dropped successfully".to_string())
            }
        }
    }

    fn get_winner(&self) -> Option<Player> {
        use PieceKind::*;
        use Player::*;
        for player in Player::iter() {
            if self.pools.get_pool_of(player).contains(&Lion) {
                return Some(player);
            }

            let x_front = match player {
                First => 0,
                Second => 3,
            };
            for y in 0_usize..3_usize {
                if !matches!(
                    self.board.get_cell_at((x_front, y)),
                    Some(piece) if piece.is_kind_of(Lion) & piece.is_owned_by(player)
                ) {
                    continue;
                };

                // when Lion was found in the front line
                let mut determined = true;
                for (xy, cell) in self.board.get_cells_around((x_front, y)).iter() {
                    if matches!(
                        cell,
                        Some(piece) if piece.can_move_from_to(*xy, (x_front, y))
                    ) {
                        determined = false;
                        break;
                    }
                }
                match determined {
                    true => return Some(player),
                    false => break,
                };
            }
        }
        None
    }

    fn execute_command(&mut self, command: &str) -> Result<String, String> {
        let mut nums = Vec::new();
        for c in command.chars() {
            match c.to_digit(10) {
                Some(n) => nums.push(n as usize),
                None => return Err("failed to parse command".to_string()),
            }
        }
        match nums.len() {
            3 => self.drop_piece_to(nums[0], (nums[1], nums[2])),
            4 => self.move_piece_from_to((nums[0], nums[1]), (nums[2], nums[3])),
            _ => Err("invalid length of command".to_string()),
        }
    }

    fn display_status(&self, colored: bool) {
        println!("{}", "-".repeat(32));
        println!("{}", self.counter.to_descriptive_string(colored));
        println!("{}", self.board.to_descriptive_string(colored));
        println!("{}", self.pools.to_descriptive_string(colored));
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
            match self.master.get_winner() {
                Some(winner) => {
                    self.master.display_status(colored);
                    println!("{:?} won!", winner);
                    break;
                }
                _ => {}
            }
            input.clear();
        }
    }
}

// ---------------------------------------------------
//  Main
// ---------------------------------------------------
fn main() {
    let mut ui = AnimalShogiUI::default();
    ui.play(true);
}
