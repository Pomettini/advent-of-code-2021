use itertools::Itertools;

type Board = Vec<Vec<u16>>;

const MAXVALUE: u16 = 255;
const MATRIX_SIZE: usize = 5;

#[derive(PartialEq)]
enum RequestType {
    First,
    Last,
}

fn find_board(numbers: &[u16], boards: &[Board], request: RequestType) -> u16 {
    let mut board_instance = boards.to_owned();
    let mut winning_boards = vec![false; boards.len()];

    for current_number in numbers {
        for index in 0..board_instance.len() {
            mark_number_on_board(&mut board_instance[index], *current_number);

            if winning_boards[index] {
                continue;
            }

            if !check_board_win(&board_instance[index]) {
                continue;
            }

            match request {
                RequestType::First => {
                    return calculate_score(&board_instance[index], *current_number);
                }
                RequestType::Last => {
                    winning_boards[index] = true;

                    if !winning_boards.contains(&false) {
                        return calculate_score(&board_instance[index], *current_number);
                    }
                }
            }
        }
    }
    0
}

fn mark_number_on_board(board: &mut Board, number: u16) {
    for row in board.iter_mut().take(MATRIX_SIZE) {
        for col in row.iter_mut().take(MATRIX_SIZE) {
            if *col == number {
                *col = MAXVALUE;
            }
        }
    }
}

fn check_board_win(board: &Board) -> bool {
    for i in 0..MATRIX_SIZE {
        let mut horizontal_score = 0;
        for j in 0..MATRIX_SIZE {
            horizontal_score += board[i][j];
        }
        if horizontal_score == MAXVALUE * MATRIX_SIZE as u16 {
            return true;
        }
    }

    for j in 0..MATRIX_SIZE {
        let mut vertical_score = 0;
        for i in 0..MATRIX_SIZE {
            vertical_score += board[i][j];
        }
        if vertical_score == MAXVALUE * MATRIX_SIZE as u16 {
            return true;
        }
    }

    false
}

fn calculate_score(current_board: &Board, current_number: u16) -> u16 {
    let sum =
        current_board.iter().flatten().fold(
            0,
            |acc, val| if *val != MAXVALUE { acc + *val } else { acc },
        );

    sum * current_number
}

fn main() {
    let input = include_str!("../../inputs/day4.txt");

    let numbers: Vec<u16> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u16>().unwrap())
        .collect();

    let boards: Vec<Board> = input
        .lines()
        .skip(1)
        .chunks(MATRIX_SIZE + 1)
        .into_iter()
        .map(|board| {
            board
                .skip(1)
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|numbers| {
                    numbers
                        .split_whitespace()
                        .map(|number| number.parse::<u16>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let first = find_board(&numbers, &boards, RequestType::First);

    assert_eq!(first, 50008);

    let second = find_board(&numbers, &boards, RequestType::Last);

    assert_eq!(second, 17408);
}
