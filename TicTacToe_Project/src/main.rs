use std::io;

fn main() {
    tic_tac_toe();
}

fn tic_tac_toe() {
    const X: char = 'X';
    const O: char = 'O';
    let mut board = vec![vec![' '; 3]; 3];

    loop {
        println!("{} | {} | {}", board[0][0], board[0][1], board[0][2]);
        println!("---------");
        println!("{} | {} | {}", board[1][0], board[1][1], board[1][2]);
        println!("---------");
        println!("{} | {} | {}", board[2][0], board[2][1], board[2][2]);

        let mut row: usize;
        let mut col: usize;
        let input = get_input(1);
        row = match input.chars().nth(0).and_then(|c| c.to_digit(10)) {
            Some(r) => r as usize,
            None => {
                println!("Invalid input, please try again");
                continue;
            }
        };
        col = match input.chars().nth(1).and_then(|c| c.to_digit(10)) {
            Some(c) => c as usize,
            None => {
                println!("Invalid input, please try again");
                continue;
            }
        };

        if board[row][col] != ' ' {
            println!("Invalid move, try again");
            continue;
        } else {
            board[row][col] = X;
        }

        let input = get_input(2);
        row = match input.chars().nth(0).and_then(|c| c.to_digit(10)) {
            Some(r) => r as usize,
            None => {
                println!("Invalid input, please try again");
                continue;
            }
        };
        col = match input.chars().nth(1).and_then(|c| c.to_digit(10)) {
            Some(c) => c as usize,
            None => {
                println!("Invalid input, please try again");
                continue;
            }
        };

        if board[row][col] != ' ' {
            println!("Invalid move, try again");
            continue;
        } else {
            board[row][col] = O;
        }


        if check_winner(&board) {
            println!("Winner!");
            break;
        }
        if check_draw(&board) {
            println!("Draw!");
            break;
        }
    }
}

fn get_input(player_idx: i8) -> String {
    println!("Player {}'s turn, input row and col in format 12 (row 1 col 2)", player_idx);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn check_draw(board: &Vec<Vec<char>>) -> bool {
    board.iter().flatten().all(|&x| x != ' ')
}

fn check_winner(board: &Vec<Vec<char>>) -> bool {
    let winning_combinations = vec![
        [(0, 0), (0, 1), (0, 2)],
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],
        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],
        [(0, 0), (1, 1), (2, 2)],
        [(0, 2), (1, 1), (2, 0)],
    ];

    for combination in winning_combinations {
        if board[combination[0].0][combination[0].1] != ' ' &&
            board[combination[0].0][combination[0].1] == board[combination[1].0][combination[1].1] &&
            board[combination[0].0][combination[0].1] == board[combination[2].0][combination[2].1] {
            return true;
        }
    }

    false
}
