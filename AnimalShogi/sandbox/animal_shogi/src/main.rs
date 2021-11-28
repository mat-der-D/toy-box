#![allow(unused)]
use std::collections::{HashMap, HashSet};
use std::error;
use std::fmt;
use std::io;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum ParseBoardError {
    InvalidColumnNum(usize),
    InvalidRowNum(usize),
}

impl fmt::Display for ParseBoardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            ParseBoardError::InvalidColumnNum(n) => format!("contains a row with {} columns", n),
            ParseBoardError::InvalidRowNum(n) => format!("contains {} rows", n),
        };
        f.write_str(&description)
    }
}

impl error::Error for ParseBoardError {}

#[derive(PartialEq, Debug)]
enum IndexError {
    OutOfBoardIndex(usize, usize),
    OutOfPoolIndex(Player, usize),
}

impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            IndexError::OutOfBoardIndex(i, j) => format!("out of board index [{}, {}]", i, j),
            IndexError::OutOfPoolIndex(player, index) => {
                format!(
                    "Player {} does not have the piece with index '{}'",
                    player, index
                )
            }
        };
        f.write_str(&description)
    }
}

impl error::Error for IndexError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Player {
    First,
    Second,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = &format!("{:?}", self);
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    fn next_index(&self, owner: &Player, index: [usize; 2]) -> Result<[usize; 2], &str> {
        let mut index: [i32; 2] = [index[0] as i32, index[1] as i32];
        // CAUTION: the above line could be critical because of unsafe type cast;
        //          (only values from 0 to 3 are expected in usual situations)
        let sign: i32 = match owner {
            Player::First => 1,
            Player::Second => -1,
        };
        let step: [i32; 2] = match self {
            Direction::Forward => [-1 * sign, 0 * sign],
            Direction::Backward => [1 * sign, 0 * sign],
            Direction::Left => [0 * sign, -1 * sign],
            Direction::Right => [0 * sign, 1 * sign],
            Direction::ForwardLeft => [-1 * sign, -1 * sign],
            Direction::ForwardRight => [-1 * sign, 1 * sign],
            Direction::BackwardLeft => [1 * sign, -1 * sign],
            Direction::BackwardRight => [1 * sign, 1 * sign],
        };
        index[0] += step[0];
        index[1] += step[1];
        if (index[0] < 0) | (index[0] > 3) | (index[1] < 0) | (index[1] > 2) {
            Err("boundary error")
        } else {
            let index: [usize; 2] = [index[0] as usize, index[1] as usize];
            // CAUTION: the above line could be critical because of unsafe type cast;
            //          (only values from 0 to 3 are expected in usual situations)
            Ok(index)
        }
    }
}

#[derive(Debug, Clone)]
enum AnimalType {
    Chick,
    Hen,
    Elephant,
    Giraffe,
    Lion,
}

impl AnimalType {
    fn make_animal(&self) -> Animal {
        Animal::new(self)
    }
}

#[derive(Debug, Clone)]
struct Animal {
    animal_type: AnimalType,
    possible_directions: HashSet<Direction>,
}

impl Animal {
    fn new(animal_type: &AnimalType) -> Self {
        let possible_directions: HashSet<_> = match animal_type {
            AnimalType::Chick => [Direction::Forward].iter().cloned().collect(),
            AnimalType::Hen => [
                Direction::Forward,
                Direction::Backward,
                Direction::Left,
                Direction::Right,
                Direction::ForwardLeft,
                Direction::ForwardRight,
            ]
            .iter()
            .cloned()
            .collect(),
            AnimalType::Elephant => [
                Direction::ForwardLeft,
                Direction::ForwardRight,
                Direction::BackwardLeft,
                Direction::BackwardRight,
            ]
            .iter()
            .cloned()
            .collect(),
            AnimalType::Giraffe => [
                Direction::Forward,
                Direction::Backward,
                Direction::Left,
                Direction::Right,
            ]
            .iter()
            .cloned()
            .collect(),
            AnimalType::Lion => [
                Direction::Forward,
                Direction::Backward,
                Direction::Left,
                Direction::Right,
                Direction::ForwardLeft,
                Direction::ForwardRight,
                Direction::BackwardLeft,
                Direction::BackwardRight,
            ]
            .iter()
            .cloned()
            .collect(),
        };
        Animal {
            animal_type: animal_type.clone(),
            possible_directions,
        }
    }
}

#[derive(Debug, Clone)]
enum PieceType {
    Chicken,
    Elephant,
    Giraffe,
    Lion,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PieceType {
    fn make_piece(&self) -> Piece {
        Piece::new(self)
    }
}

#[derive(Debug, Clone)]
struct Piece {
    piece_type: PieceType,
    top: Animal,
    bottom: Animal,
    is_reversed: bool,
}

impl Piece {
    fn new(piece_type: &PieceType) -> Self {
        let top = match piece_type {
            PieceType::Chicken => Animal::new(&AnimalType::Chick),
            PieceType::Elephant => Animal::new(&AnimalType::Elephant),
            PieceType::Giraffe => Animal::new(&AnimalType::Giraffe),
            PieceType::Lion => Animal::new(&AnimalType::Lion),
        };
        let bottom = match piece_type {
            PieceType::Chicken => Animal::new(&AnimalType::Hen),
            _ => top.clone(),
        };
        Piece {
            piece_type: piece_type.clone(),
            top,
            bottom,
            is_reversed: false,
        }
    }

    fn evolve(&mut self) {
        self.is_reversed = true;
    }

    fn devolve(&mut self) {
        self.is_reversed = false;
    }

    fn possible_directions(&self) -> &HashSet<Direction> {
        if self.is_reversed {
            &self.bottom.possible_directions
        } else {
            &self.top.possible_directions
        }
    }
}

#[derive(Debug, Clone)]
struct OwnedPiece {
    owner: Player,
    piece: Piece,
}

impl fmt::Display for OwnedPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Piece {
            piece_type,
            is_reversed,
            ..
        } = &self.piece;
        let s = match (&self.owner, piece_type, is_reversed) {
            (Player::First, PieceType::Chicken, false) => 'C'.to_string(),
            (Player::First, PieceType::Chicken, true) => 'H'.to_string(),
            (Player::First, PieceType::Elephant, _) => 'E'.to_string(),
            (Player::First, PieceType::Giraffe, _) => 'G'.to_string(),
            (Player::First, PieceType::Lion, _) => 'L'.to_string(),
            (Player::Second, PieceType::Chicken, false) => 'c'.to_string(),
            (Player::Second, PieceType::Chicken, true) => 'h'.to_string(),
            (Player::Second, PieceType::Elephant, _) => 'e'.to_string(),
            (Player::Second, PieceType::Giraffe, _) => 'g'.to_string(),
            (Player::Second, PieceType::Lion, _) => 'l'.to_string(),
        };
        write!(f, "{}", s)
    }
}

impl FromStr for OwnedPiece {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(OwnedPiece {
                owner: Player::First,
                piece: Piece::new(&PieceType::Chicken),
            }),
            "H" => {
                let mut piece = Piece::new(&PieceType::Chicken);
                piece.evolve();
                Ok(OwnedPiece {
                    owner: Player::First,
                    piece,
                })
            }
            "E" => Ok(OwnedPiece {
                owner: Player::First,
                piece: Piece::new(&PieceType::Elephant),
            }),
            "G" => Ok(OwnedPiece {
                owner: Player::First,
                piece: Piece::new(&PieceType::Giraffe),
            }),
            "L" => Ok(OwnedPiece {
                owner: Player::First,
                piece: Piece::new(&PieceType::Lion),
            }),
            "c" => Ok(OwnedPiece {
                owner: Player::Second,
                piece: Piece::new(&PieceType::Chicken),
            }),
            "h" => {
                let mut piece = Piece::new(&PieceType::Chicken);
                piece.evolve();
                Ok(OwnedPiece {
                    owner: Player::Second,
                    piece,
                })
            }
            "e" => Ok(OwnedPiece {
                owner: Player::Second,
                piece: Piece::new(&PieceType::Elephant),
            }),
            "g" => Ok(OwnedPiece {
                owner: Player::Second,
                piece: Piece::new(&PieceType::Giraffe),
            }),
            "l" => Ok(OwnedPiece {
                owner: Player::Second,
                piece: Piece::new(&PieceType::Lion),
            }),
            _ => Err("no piece"),
        }
    }
}

impl OwnedPiece {
    fn evolve(&mut self) {
        self.piece.evolve();
    }

    fn devolve(&mut self) {
        self.piece.devolve();
    }
}

#[derive(Debug, Clone)]
struct PiecePool {
    pools: HashMap<Player, Vec<Piece>>,
}

impl Default for PiecePool {
    fn default() -> Self {
        let mut pools: HashMap<Player, Vec<Piece>> = HashMap::new();
        pools.insert(Player::First, Vec::new());
        pools.insert(Player::Second, Vec::new());
        PiecePool { pools }
    }
}

impl PiecePool {
    fn validate_index(&self, owner: &Player, index: usize) -> Result<&str, IndexError> {
        let pool = self.pool_of(&owner);
        if index < pool.len() {
            Ok("")
        } else {
            Err(IndexError::OutOfPoolIndex(owner.clone(), index))
        }
    }

    // fn is_valid_index(&self, owner: &Player, index: usize) -> bool {
    //     let pool = self.pool_of(&owner);
    //     index < pool.len()
    // }

    fn pool_of(&self, owner: &Player) -> &Vec<Piece> {
        &self.pools[owner]
    }

    fn add_piece(&mut self, new_owner: &Player, new_piece: OwnedPiece) {
        let mut piece = new_piece.piece;
        piece.devolve();
        let mut pool = self.pools.get_mut(new_owner).unwrap();
        pool.push(piece);
    }

    fn remove_piece(&mut self, owner: &Player, index: usize) -> OwnedPiece {
        let mut pool = self.pools.get_mut(owner).unwrap();
        let piece = pool.remove(index);
        OwnedPiece {
            owner: owner.clone(),
            piece,
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    board: [[Option<OwnedPiece>; 3]; 4],
}

impl FromStr for Board {
    type Err = ParseBoardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board: [[Option<OwnedPiece>; 3]; 4] = Default::default();
        let lines: Vec<&str> = s.split(";").collect();

        let num_rows: usize = lines.len();
        if num_rows != 4 {
            return Err(ParseBoardError::InvalidRowNum(num_rows));
        }

        for (i, line) in lines.iter().enumerate() {
            let num_columns: usize = line.len();
            if num_columns != 3 {
                return Err(ParseBoardError::InvalidRowNum(num_rows));
            }
            for (j, c) in line.chars().enumerate() {
                let mut buffer = [0u8; 4];
                let s: &mut str = c.encode_utf8(&mut buffer);
                board[i][j] = match OwnedPiece::from_str(s) {
                    Ok(p) => Some(p),
                    Err(_) => None,
                }
            }
        }
        Ok(Board { board })
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::from_str("gle;-c-;-C-;ELG").unwrap()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.to_string_for_display(false);
        write!(f, "{}", s)
    }
}

impl Board {
    fn validate_index(index: [usize; 2]) -> Result<&'static str, IndexError> {
        if (index[0] <= 3) & (index[1] <= 2) {
            Ok("")
        } else {
            Err(IndexError::OutOfBoardIndex(index[0], index[1]))
        }
    }

    // fn is_valid_index(index: [usize; 2]) -> bool {
    //     (index[0] <= 3) & (index[1] <= 2)
    // }

    fn owner_of(&self, index: [usize; 2]) -> Option<&Player> {
        match &self.board[index[0]][index[1]] {
            Some(OwnedPiece { owner, .. }) => Some(owner),
            None => None,
        }
    }

    fn possible_next_of(&self, index: [usize; 2]) -> HashSet<[usize; 2]> {
        let mut s = HashSet::new();
        if let Some(OwnedPiece { owner, piece }) = &self.board[index[0]][index[1]] {
            for direction in piece.possible_directions().iter() {
                let next_index = match direction.next_index(&owner, index) {
                    Ok(x) => x,
                    Err(_) => continue,
                };
                match self.owner_of(next_index) {
                    Some(x) if x == owner => continue,
                    _ => s.insert(next_index),
                };
            }
        }
        s
    }

    fn place_piece(&mut self, piece: OwnedPiece, index: [usize; 2]) {
        self.board[index[0]][index[1]] = Some(piece);
    }

    fn move_piece(&mut self, index_from: [usize; 2], index_to: [usize; 2]) -> Option<OwnedPiece> {
        let mut p: Option<OwnedPiece> = None;
        std::mem::swap(&mut self.board[index_from[0]][index_from[1]], &mut p);
        std::mem::swap(&mut self.board[index_to[0]][index_to[1]], &mut p);
        p
    }

    fn evolve_at(&mut self, index: [usize; 2]) -> Result<&str, &str> {
        let mut piece = match &mut self.board[index[0]][index[1]] {
            Some(x) => x,
            None => return Err("no piece exists there"),
        };
        piece.evolve();
        Ok("evolved!")
    }

    fn to_string_for_display(&self, colored: bool) -> String {
        let sep = String::from(" +-+-+-+");
        let mut str_vec = vec![String::from("  0 1 2"), sep.clone()];
        for i in 0..4 {
            let mut row_str = String::new();
            row_str.push_str(&i.to_string());
            row_str.push_str("|");
            for j in 0..3 {
                match &self.board[i][j] {
                    Some(owned_piece) => {
                        if colored {
                            let color = match owned_piece.owner {
                                Player::First => "\x1b[97m\x1b[40m",
                                Player::Second => "\x1b[30m\x1b[47m",
                            };
                            row_str.push_str(color);
                        }
                        row_str.push_str(&owned_piece.to_string());
                        if colored {
                            row_str.push_str("\x1b[0m")
                        }
                    }
                    None => row_str.push_str(" "),
                };
                row_str.push_str("|");
            }
            str_vec.push(row_str);
            str_vec.push(sep.clone());
        }
        str_vec.join("\n")
    }
}

#[derive(Debug, Clone)]
struct TurnCounter {
    number: u32,
    player: Player,
}

impl Default for TurnCounter {
    fn default() -> Self {
        TurnCounter {
            number: 1,
            player: Player::First,
        }
    }
}

impl fmt::Display for TurnCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("Player {}'s Turn ({})", self.player, self.number,))
    }
}

impl TurnCounter {
    fn advance(&mut self) {
        match self.player {
            Player::First => {
                self.player = Player::Second;
            }
            Player::Second => {
                self.number += 1;
                self.player = Player::First;
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct GameMaster {
    board: Board,
    pools: PiecePool,
    counter: TurnCounter,
}

impl GameMaster {
    fn show_status(&self, colored: bool) {
        println!("---------------------------------");
        self.show_turn();
        self.show_board(colored);
        self.show_pool(&Player::First, colored);
        self.show_pool(&Player::Second, colored);
    }

    fn show_board(&self, colored: bool) {
        println!("{}", self.board.to_string_for_display(colored));
    }

    fn show_pool(&self, player: &Player, colored: bool) {
        let mut piece_names = Vec::new();
        let pool = self.pools.pool_of(player);
        for (i, Piece { piece_type, .. }) in pool.iter().enumerate() {
            piece_names.push(format!("{:?}({})", piece_type, i));
        }
        if colored {
            match player {
                Player::First => println!(
                    "\x1b[97m\x1b[40m{}'s Pool\x1b[0m: {}",
                    player,
                    piece_names.join(", ")
                ),
                Player::Second => println!(
                    "\x1b[30m\x1b[47m{}'s Pool\x1b[0m: {}",
                    player,
                    piece_names.join(", ")
                ),
            };
        } else {
            println!("{}'s Pool: {}", player, piece_names.join(", "));
        }
    }

    fn show_turn(&self) {
        println!("{}", self.counter);
    }

    fn winner(&self) -> Option<&Player> {
        for player in [&Player::First, &Player::Second].iter() {
            let pool = self.pools.pool_of(player);
            for Piece { piece_type, .. } in pool.iter() {
                if matches!(piece_type, PieceType::Lion) {
                    return Some(player);
                }
            }
        }

        fn is_lion_dangerous(board: &Board, index_lion: [usize; 2]) -> bool {
            for i in 0..4 {
                for j in 0..3 {
                    if board.possible_next_of([i, j]).contains(&index_lion) {
                        return true;
                    }
                }
            }
            false
        }

        for (j_lion, piece) in self.board.board[0].iter().enumerate() {
            match piece {
                Some(OwnedPiece {
                    owner: Player::First,
                    piece:
                        Piece {
                            piece_type: PieceType::Lion,
                            ..
                        },
                }) => {
                    if !is_lion_dangerous(&self.board, [0, j_lion]) {
                        return Some(&Player::First);
                    }
                }
                _ => {}
            }
        }

        for (j_lion, piece) in self.board.board[3].iter().enumerate() {
            match piece {
                Some(OwnedPiece {
                    owner: Player::Second,
                    piece:
                        Piece {
                            piece_type: PieceType::Lion,
                            ..
                        },
                }) => {
                    if !is_lion_dangerous(&self.board, [3, j_lion]) {
                        return Some(&Player::Second);
                    }
                }
                _ => {}
            }
        }

        None
    }

    fn move_piece(&mut self, index_from: [usize; 2], index_to: [usize; 2]) -> Result<&str, &str> {
        Self::move_piece_dev(
            &mut self.board,
            &mut self.pools,
            &mut self.counter,
            index_from,
            index_to,
        )
    }

    fn move_piece_dev(
        board: &mut Board,
        pools: &mut PiecePool,
        counter: &mut TurnCounter,
        index_from: [usize; 2],
        index_to: [usize; 2],
    ) -> Result<&'static str, &'static str> {
        let player = &counter.player.clone(); // clone is needed because player is changed when turn is advanced
        if Board::validate_index(index_from).is_err() {
            return Err("origin index out of bounds");
        }
        if Board::validate_index(index_to).is_err() {
            return Err("target index out of bounds");
        }
        match board.owner_of(index_from) {
            Some(x) if x != player => return Err("you cannot control the piece"),
            None => return Err("no piece exists there"),
            _ => {}
        }
        if board.possible_next_of(index_from).contains(&index_to) {
            let taken_piece = board.move_piece(index_from, index_to);
            if [(&Player::First, 0), (&Player::Second, 3)].contains(&(player, index_to[0])) {
                board.evolve_at(index_to).expect("Unexpected Situation");
            }
            counter.advance();
            if let Some(p) = taken_piece {
                pools.add_piece(player, p);
                Ok("moved and took a piece")
            } else {
                Ok("moved")
            }
        } else {
            Err("you cannot move the piece there")
        }
    }

    fn place_piece_from_pool(
        &mut self,
        index_in_pool: usize,
        index_in_board: [usize; 2],
    ) -> Result<&str, &str> {
        Self::place_piece_from_pool_dev(
            &mut self.pools,
            &mut self.board,
            &mut self.counter,
            index_in_pool,
            index_in_board,
        )
    }

    fn place_piece_from_pool_dev(
        pools: &mut PiecePool,
        board: &mut Board,
        counter: &mut TurnCounter,
        index_in_pool: usize,
        index_in_board: [usize; 2],
    ) -> Result<&'static str, &'static str> {
        let player = &counter.player.clone(); // clone is needed because player is changed when turn is advanced
        if pools.validate_index(player, index_in_pool).is_err() {
            return Err("pool index out of bounds");
        }
        if Board::validate_index(index_in_board).is_err() {
            return Err("board index out of bounds");
        }

        if !board.owner_of(index_in_board).is_none() {
            Err("some piece has already been there")
        } else {
            let piece = pools.remove_piece(player, index_in_pool);
            board.place_piece(piece, index_in_board);
            counter.advance();
            Ok("succesfully put the piece")
        }
    }
}

fn main() {
    let mut colored = true;
    let mut input = String::new();
    let mut master = GameMaster::default();

    loop {
        println!("Do you want to use colored-mode?[Y/n] >");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        match &*input.replace("\n", "").to_lowercase() {
            "y" => {
                colored = true;
                input.clear();
                break;
            }
            "n" => {
                colored = false;
                input.clear();
                break;
            }
            _ => {
                println!("please type 'Y' or 'n'");
                input.clear();
            }
        }
    }

    loop {
        master.show_status(colored);
        println!("Input your hand >");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let results: Vec<_> = input
            .replace("\n", "")
            .chars()
            .map(|x| x.to_string().parse::<usize>())
            .collect();
        if results.iter().any(|x| matches!(x, Err(_))) {
            println!("{:?}", Err("Invalid Syntax") as Result<&str, &str>);
            input.clear();
            continue;
        }
        let numbers: Vec<_> = results.iter().cloned().map(|x| x.unwrap()).collect();
        match numbers.len() {
            3 => {
                let result = master.place_piece_from_pool(numbers[0], [numbers[1], numbers[2]]);
                println!("{:?}", result);
            }
            4 => {
                let result = master.move_piece([numbers[0], numbers[1]], [numbers[2], numbers[3]]);
                println!("{:?}", result);
            }
            _ => println!("{:?}", Err("Invalid Syntax") as Result<&str, &str>),
        };
        input.clear();
        if let Some(w) = master.winner() {
            master.show_status(colored);
            println!("{} win!", w);
            break;
        }
    }
}
