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

#[derive(PartialEq, Debug)]
enum Compass {
    North,
    East,
    South,
    West,
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

    fn check_win(&self, row: usize, file: usize, candidate: Colour, mut win_sum: Vec<Compass>, visited: &mut Vec<(usize, usize)>) -> Vec<Compass> {
        if visited.contains(&(row, file)){
            return win_sum;
        } else {
            visited.push((row, file))
        }


        if row == 0 {
            win_sum.push(Compass::North);
        }
        if file == self.size - 1 {
            win_sum.push(Compass::East);
        }
        if row == self.size - 1 {
            win_sum.push(Compass::South);
        }
        if file == 0 {
            win_sum.push(Compass::West);
        }

        if row > 0 && self.board[row-1][file] != None && self.board[row-1][file].unwrap() == candidate {
            win_sum = self.check_win(row-1, file, candidate, win_sum, visited);
        }
        if file+1 < self.size && self.board[row][file+1] != None && self.board[row][file+1].unwrap() == candidate {
            win_sum = self.check_win(row, file+1, candidate, win_sum, visited);
        }
        if row+1 < self.size && self.board[row+1][file] != None && self.board[row+1][file].unwrap() == candidate {
            win_sum = self.check_win(row+1, file, candidate, win_sum, visited);
        }
        if file > 0 && self.board[row][file-1] != None && self.board[row][file-1].unwrap() == candidate {
            win_sum = self.check_win(row, file-1, candidate, win_sum, visited);
        }
        win_sum

    }
}

fn main() {
    let board: Board = Board::new(5);
    game_loop(board)
}

fn game_loop(mut board: Board) {
    let mut counter: Colour = Colour::Black;
    loop{
        board.print();
        println!("{:?}'s turn", counter);
        let mut player_move = String::new();
        io::stdin().read_line(&mut player_move).expect("TODO: panic message");
        let player_move = player_move.trim();
        let board_file: usize;
        let board_row: usize;
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
        if !(player_move.len() == 2 && player_move.as_bytes()[0].is_ascii_alphabetic()) {
            println!("{}", ERR_BAD_INPUT);
            continue
        }
        match player_move.as_bytes()[0] {
            b if b >= b'a' && ((b - b'a') as usize) < board.size => {
                board_file = (b - b'a') as usize;
            }
            _ => {
                println!("{}", ERR_BAD_INPUT);
                continue
            }
        }
        match &player_move[1..2] {
            s if s.parse::<usize>().is_ok_and(|n| n <= board.size && n > 0) => {
                board_row = s.parse::<usize>().unwrap() - 1;
            }
            _ => {
                println!("{}", ERR_BAD_INPUT);
                continue
            }
        }
        if board.board[board_row][board_file] == None { board.board[board_row][board_file] = Some(counter)}
        else {
            println!("{}{:?}", ERR_OCCUPIED, board.board[board_row][board_file]);
            continue
        }
        let winner = board.check_win(board_row, board_file, counter, Vec::new(), &mut Vec::new());
        if winner.contains(&Compass::North) && winner.contains(&Compass::South) {
            println!("{:?} won by connecting North and South!", counter);
            io::stdin().read_line(&mut String::new()).expect("TODO: panic message");
            return;
        }
        else if winner.contains(&Compass::East) && winner.contains(&Compass::West) {
            println!("{:?} won by connecting East and West!", counter);
            io::stdin().read_line(&mut String::new()).expect("TODO: panic message");
            return;
        }
        counter = Colour::advance_turn(counter);

    }
}