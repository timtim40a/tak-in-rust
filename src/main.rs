use std::io;

const ERR_OCCUPIED: &str = "The requested square is already occupied by ";
const ERR_BAD_INPUT: &str = "Bad input";

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq, Eq)]
enum Colour {
    Black,
    White,
}

impl Colour {
    fn advance_turn(colour: Colour) -> Colour {
        if colour == Colour::Black {Colour::White}
        else {Colour::Black}
    }
}

struct Board {
    board: Vec<Vec<Option<Colour>>>,
    size: usize,
}

impl Board {
    fn new(size: usize) -> Board {
        Board {
            board: vec![vec![None; size]; size],
            size,
        }
    }

    fn print(&self) {
        for i in self.board.as_slice() {
            println!("{}", "_".repeat(self.size * 8 + 1));
            for j in i {
                match j {
                    Some(Colour::Black) => print!("| Black "),
                    Some(Colour::White) => print!("| White "),
                    None => print!("|       ")
                }
            }
            println!("|")
        }
        println!("{}", "_".repeat(self.size * 8 + 1));
    }
    
    fn check_win(&self) {
        println!("Checking win conditions");
    }
}

fn main() {
    let mut board: Board = Board::new(3);
    gameloop(board)
}

fn gameloop(mut board: Board) {
    let mut counter: Colour = Colour::Black;
    loop{
        board.print();
        println!("{:?}'s turn", counter);
        let mut player_move = String::new();
        io::stdin().read_line(&mut player_move).expect("TODO: panic message");
        let player_move = player_move.trim();
        let board_file: usize;
        match &player_move {
            &"" => {
                println!("{}", ERR_BAD_INPUT);
                continue
            },
            &"quit" => break,
            &"clear" => {
                board = Board::new(board.size);
                continue
            },
            _ => {},
        }
        match &player_move[0..1] {
            "a" => {
                board_file = 0
            }
            "b" => {
                board_file = 1
            }
            "c" => {
                board_file = 2
            }
            _ => {
                println!("{}", ERR_BAD_INPUT);
                continue
            }
        }
        match &player_move[1..2] {
            "1" => {
                if board.board[0][board_file] == None { board.board[0][board_file] = Some(counter)}
                else {
                    println!("{}{:?}", ERR_OCCUPIED, board.board[0][board_file]);
                    continue
                }
            }
            "2" => {
                if board.board[1][board_file] == None { board.board[1][board_file] = Some(counter)}
                else {
                    println!("{}{:?}", ERR_OCCUPIED, board.board[1][board_file]);
                    continue
                }
            }
            "3" => {
                if board.board[2][board_file] == None { board.board[2][board_file] = Some(counter)}
                else {println!("{}{:?}", ERR_OCCUPIED, board.board[2][board_file]); continue}
            }
            _ => {
                println!("{}", ERR_BAD_INPUT);
                continue
            }
        }
        board.check_win();
        counter = Colour::advance_turn(counter);

    }
}