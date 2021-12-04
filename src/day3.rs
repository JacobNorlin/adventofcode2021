use crate::utils::read_file_vec;

pub fn main() {
    let lines = read_file_vec("./inputs/day3.txt", "\n");
    let m: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();

    part1(&m);
    part2(&m);
}

fn part1(vec: &Vec<Vec<char>>) {
    let mut gamma = 0;
    let mut epsilon = 0;

    let inner_len = vec[0].len();

    let mut i = 0;
    let base: i32 = 2;
    while i < inner_len {
        let common_bit = get_most_common_bit(&vec, i);
        let p = inner_len as u32 - i as u32 - 1;
        if common_bit == '0' {
            epsilon |= base.pow(p);
        } else {
            gamma = gamma + base.pow(p);
        }
        i = i + 1;
    }

    println!("day3 part 1: {}", gamma * epsilon);
}

fn part2(vec: &Vec<Vec<char>>) {
    let inner_len = vec[0].len();

    let mut oxygen_rating = vec.clone();
    let mut co2_rating = vec.clone();
    let mut i = 0;
    let mut ox_res = 0;
    let mut co2_res = 0;
    while i < inner_len {
        let common_bit = get_most_common_bit(&oxygen_rating, i);
        let least_bit = get_most_common_bit(&co2_rating, i);
        oxygen_rating = oxygen_rating
            .into_iter()
            .filter(|x| x[i] == common_bit)
            .collect();
        if oxygen_rating.len() == 1 {
            ox_res = char_bit_vec_to_i32(&oxygen_rating[0]);
        }
        co2_rating = co2_rating
            .into_iter()
            .filter(|x| x[i] != least_bit)
            .collect();
        if co2_rating.len() == 1 {
            co2_res = char_bit_vec_to_i32(&co2_rating[0]);
        }
        i = i + 1;
    }
    println!("day3 part 2: {}", ox_res * co2_res);
}

fn char_bit_vec_to_i32(bits: &Vec<char>) -> i32 {
    let l = bits.len();
    return bits.iter().enumerate().fold(0, |acc, (k, bit)| {
        if *bit == '1' {
            acc + (2 as i32).pow(l as u32 - k as u32 - 1)
        } else {
            acc
        }
    });
}

fn get_most_common_bit(vec: &Vec<Vec<char>>, index: usize) -> char {
    let mut zero_count = 0;
    let mut one_count = 0;
    for row in vec {
        let b = row[index];
        if b == '1' {
            one_count = one_count + 1
        } else {
            zero_count = zero_count + 1;
        }
    }
    if zero_count > one_count {
        return '0';
    } else {
        return '1';
    }
}
