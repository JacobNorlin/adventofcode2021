use crate::utils::read_file_vec;

pub fn main() {
    let lines = read_file_vec("./inputs/day4.txt", "\n\n");

    let bingo_numbers = &lines[0]
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    let mut boards = vec![vec![vec![0; 5]; 5]; lines.len() - 1];
    for i in 1..lines.len() {
        let board = to_board(&lines[i]);
        boards[i - 1] = board;
    }

    part1(bingo_numbers, &boards);
    part2(bingo_numbers, &boards);
}

fn part1(bingo_numbers: &Vec<i32>, boards: &Vec<Vec<Vec<i32>>>) {
    let mut marked_numbers = vec![-1; bingo_numbers.len()];
    for (i, bingo_number) in bingo_numbers.iter().enumerate() {
        marked_numbers[i] = *bingo_number;
        for board in boards {
            let bingo = check_bingo(&board, &marked_numbers);
            if bingo == true {
                //First 5 rows is the matrix of the board
                let sum = sum_unmarked(&board[0..5], &marked_numbers);
                println!("day4 part 1: {}", sum * bingo_number);
                return;
            }
        }
    }
}

fn part2(bingo_numbers: &Vec<i32>, boards: &Vec<Vec<Vec<i32>>>) {
    let mut marked_numbers = vec![-1; bingo_numbers.len()];
    let mut board_has_won = vec![false; boards.len()];
    let mut sum = -1;
    for (i, bingo_number) in bingo_numbers.iter().enumerate() {
        marked_numbers[i] = *bingo_number;
        for (board_num, board) in boards.iter().enumerate() {
            if board_has_won[board_num] == true {
                continue;
            }
            let bingo = check_bingo(&board, &marked_numbers);
            if bingo == true {
                //First 5 rows is the matrix of the board
                sum = sum_unmarked(&board[0..5], &marked_numbers) * bingo_number;
                board_has_won[board_num] = true;
            }
        }
    }
    println!("day4 part 2: {}", sum);
}

fn sum_unmarked(board: &[Vec<i32>], marked_numbers: &Vec<i32>) -> i32 {
    return get_unmarked_numbers(board, marked_numbers)
        .iter()
        .fold(0, |acc, v| acc + v);
}

fn get_unmarked_numbers(board: &[Vec<i32>], marked_numbers: &Vec<i32>) -> Vec<i32> {
    let all_nums = board
        .iter()
        .flatten()
        .cloned()
        .filter(|v| !marked_numbers.iter().any(|k| k == v))
        .collect::<Vec<i32>>();
    return all_nums;
}

fn to_board(board: &String) -> Vec<Vec<i32>> {
    //Split into matrix of bingo numbers
    let m: Vec<Vec<i32>> = board
        .split("\n")
        .map(|v| {
            v.split_whitespace()
                .map(|k| k.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    //Enumerate all bingo vectors
    let hori = m.clone();

    let mut vert = vec![vec![0; 5]; 5];
    for i in 0..vert.len() {
        for j in 0..vert.len() {
            vert[i][j] = m[j][i] as i32;
        }
    }

    //Consume into list of all possible bingo thingies
    let rows: Vec<Vec<i32>> = hori
        .into_iter()
        .chain(vert.into_iter())
        // .chain(diag.into_iter())
        .collect();
    return rows;
}

fn check_bingo(board: &Vec<Vec<i32>>, marked_numbers: &Vec<i32>) -> bool {
    let val = board
        .iter()
        .any(|row| check_row_matches(row, marked_numbers));

    return val;
}

fn check_row_matches(row: &Vec<i32>, marked_numbers: &Vec<i32>) -> bool {
    //there shouldn't be duplicate numbers so this is probably same
    return row.iter().all(|v| marked_numbers.iter().any(|k| *k == *v));
}
