use crate::utils::read_file_vec;

pub fn main() {
    let input = read_file_vec("./inputs/day10.txt", "\n");
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let mut score = 0;
    for row in input {
        let mut expected = Vec::new();
        for c in row.chars() {
            match c {
                '{' => expected.push('}'),
                '<' => expected.push('>'),
                '[' => expected.push(']'),
                '(' => expected.push(')'),
                _ => {
                    let peek = *expected.last().unwrap();
                    if c != peek {
                        score += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("bad"),
                        };
                        break;
                    } else {
                        expected.pop();
                    }
                }
            }
        }
    }
    println!("day10 part 1: {}", score);
}

fn part2(input: &Vec<String>) {
    let mut scores: Vec<i64> = Vec::new();
    for row in input {
        let mut expected = Vec::new();
        let mut valid = true;
        for c in row.chars() {
            match c {
                '{' => expected.push('}'),
                '<' => expected.push('>'),
                '[' => expected.push(']'),
                '(' => expected.push(')'),
                _ => {
                    let peek = *expected.last().unwrap();
                    if c != peek {
                        valid = false;
                    } else {
                        expected.pop();
                    }
                }
            }
        }
        if valid {
            let row_score = expected.iter().rev().fold(0, |acc, c| {
                let s = match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("bad!"),
                };
                return (acc * 5) + s;
            });
            scores.push(row_score);
        }
    }
    scores.sort();
    println!("day10 part 2: {}", scores[(scores.len() - 1) / 2]);
}
