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

fn main() {
    let mut board: [[Option<Colour>; 3]; 3] = [[None;3];3];
    gameloop(board)
}

fn gameloop(mut board: [[Option<Colour>; 3]; 3]) {
    let mut counter: Colour = Colour::Black;
    loop{
        println!("{:?}\n{:?}\n{:?}\nIt is player {:?}'s turn.",board[0],board[1],board[2],counter);
        let mut player_move = String::new();
        io::stdin().read_line(&mut player_move);
        let player_move = player_move.trim();
        let mut board_file: usize;
        match &player_move {
            &"quit" => break,
            &"clear" => {
                board = [[None; 3]; 3];
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
                if board[0][board_file] == None { board[0][board_file] = Some(counter)}
                else {
                    println!("{}{:?}", ERR_OCCUPIED, board[0][board_file]);
                    continue
                }
            }
            "2" => {
                if board[1][board_file] == None { board[1][board_file] = Some(counter)}
                else {
                    println!("{}{:?}", ERR_OCCUPIED, board[1][board_file]);
                    continue
                }
            }
            "3" => {
                if board[2][board_file] == None { board[2][board_file] = Some(counter)}
                else {println!("{}{:?}", ERR_OCCUPIED, board[2][board_file]); continue}
            }
            _ => {
                println!("{}", ERR_BAD_INPUT);
                continue
            }
        }
        counter = Colour::advance_turn(counter);

    }
}