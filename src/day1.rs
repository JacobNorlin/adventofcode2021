use crate::utils::read_file_vec;

pub fn main() {
    let contents = read_file_vec("./inputs/day1.txt");
    let nums = contents.iter().map(|v| v.parse::<i32>().unwrap()).collect();
    part1(&nums);
    part2(&nums);
}

fn part1(vec: &Vec<i32>) {
    let mut count = 0;
    let mut i = 1;
    while i < vec.len() {
        let prev_num = vec[i - 1];
        let num = vec[i];
        if num > prev_num {
            count = count + 1;
        }
        i = i + 1;
    }

    println!("day1 part 1: {}", count);
}

fn part2(vec: &Vec<i32>) {
    let mut count = 0;
    let mut i = 2;
    let mut prev_sum = 2147483647;
    while i < vec.len() {
        let sum = vec[i - 2] + vec[i - 1] + vec[i];
        if sum > prev_sum {
            count = count + 1
        }
        prev_sum = sum;
        i = i + 1;
    }

    println!("day1 part 2: {}", count);
}
