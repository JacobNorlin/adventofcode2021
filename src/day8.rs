use crate::utils::read_file_vec;
use itertools::Itertools;
use std::collections::HashMap;

pub fn main() {
    let input: Vec<Vec<Vec<String>>> = read_file_vec("./inputs/day8.txt", "\n")
        .iter()
        .map(|v| {
            v.split(" | ")
                .map(|p| p.split(" ").map(|k| k.parse::<String>().unwrap()).collect())
                .collect()
        })
        .collect();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<Vec<Vec<String>>>) {
    let mut sum = 0;
    for row in input {
        let output = &row[1];

        for wires in output {
            let num = match wires.len() {
                2 => 1,
                4 => 1,
                3 => 1,
                7 => 1,
                _ => 0,
            };
            sum += num;
        }
    }
    println!("day8 part 1: {}", sum);
}

fn part2(input: &Vec<Vec<Vec<String>>>) {
    //Generate all possible boards
    let boards = ['a', 'b', 'c', 'd', 'e', 'f', 'g']
        .iter()
        .permutations(7)
        .map(|perm| SignalBoard::new(perm))
        .collect::<Vec<SignalBoard>>();

    let mut sum = 0;
    for row in input {
        let signals = &row[0];
        let output = &row[1];

        let valid_board = boards
            .iter()
            .filter(|board| {
                signals
                    .iter()
                    .all(|signal| board.is_valid_number(signal.chars().collect::<Vec<char>>()))
            })
            .next()
            .unwrap();

        let out_num = output
            .iter()
            .map(|v| valid_board.get_number(v.chars().collect::<Vec<char>>()))
            .enumerate()
            .fold(0, |acc, (i, v)| {
                acc + (10 as i32).pow(3u32 - i as u32) * v as i32
            });
        sum += out_num;
    }
    println!("day8 part 2: {}", sum);
}

#[derive(Debug)]
struct SignalBoard {
    map: HashMap<char, char>,
    valid_numbers: Vec<Vec<char>>,
}

impl SignalBoard {
    fn new(perm: Vec<&char>) -> Self {
        let mut map: HashMap<char, char> = HashMap::new();

        map.insert('a', *perm[0]);
        map.insert('b', *perm[1]);
        map.insert('c', *perm[2]);
        map.insert('d', *perm[3]);
        map.insert('e', *perm[4]);
        map.insert('f', *perm[5]);
        map.insert('g', *perm[6]);

        let valid_numbers = [
            "bacefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdfeg", "acf", "abcdfeg", "abcdfg",
        ]
        .iter()
        .map(|num| {
            num.chars()
                .map(|c| map.get(&c).unwrap())
                .map(|c| *c)
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

        return Self { map, valid_numbers };
    }

    fn is_valid_number(&self, str: Vec<char>) -> bool {
        return self
            .valid_numbers
            .iter()
            .any(|v| str_equal_no_order(&str, v));
    }

    fn get_number(&self, str: Vec<char>) -> i32 {
        for (i, num) in self.valid_numbers.iter().enumerate() {
            if str_equal_no_order(num, &str) {
                return i as i32;
            }
        }
        return -1;
    }
}

fn str_equal_no_order(a: &Vec<char>, b: &Vec<char>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    return a.iter().all(|c| b.contains(c));
}
