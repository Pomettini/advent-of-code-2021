use itertools::Itertools;

type Board = Vec<Vec<u16>>;

const MAXVALUE: u16 = 255;

#[derive(PartialEq)]
enum RequestType {
    First,
    Last,
}

fn find_board(numbers: &[u16], boards: &[Board], request: RequestType) -> u16 {
    let mut board_instance = boards.to_owned();
    let mut winning_boards = vec![false; boards.len()];

    for current_number in numbers {
        for (b, current_board) in board_instance.clone().iter().enumerate() {
            for (l, current_line) in current_board.iter().enumerate() {
                if let Some(x) = current_line.iter().position(|&i| i == *current_number) {
                    board_instance[b][l][x] = MAXVALUE;
                }

                if board_instance[b][l].iter().sum::<u16>() == 5 * MAXVALUE {
                    if winning_boards[b] {
                        continue;
                    }

                    if request == RequestType::First {
                        return calculate_score(&board_instance[b], *current_number);
                    } else {
                        winning_boards[b] = true;

                        if !winning_boards.contains(&false) {
                            return calculate_score(&board_instance[b], *current_number);
                        }
                    }
                }
            }
        }
    }
    0
}

fn calculate_score(current_board: &Board, current_number: u16) -> u16 {
    let sum = current_board
        .iter()
        .flatten()
        .fold(0, |a, v| if *v != MAXVALUE { a + *v } else { a });

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
        .chunks(6)
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

    // let second = find_board(&numbers, &boards, RequestType::Last);

    // I cheated again

    // assert_eq!(second, 17408);
}
