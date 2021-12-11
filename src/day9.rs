use std::collections::HashMap;

use crate::utils::read_file_vec;

pub fn main() {
    let input: Vec<Vec<u32>> = read_file_vec("./inputs/day9.txt", "\n")
        .iter()
        .map(|row| row.chars().map(|v| v.to_digit(10).unwrap()).collect())
        .collect();
    part1(&input);
    part2(&input);
}

fn part1(m: &Vec<Vec<u32>>) {
    let mut score = 0;
    for y in 0..m.len() {
        for x in 0..m[y].len() {
            if is_lowest(y, x, m) {
                score += 1 + m[y][x];
            }
        }
    }
    println!("day9 part 1: {}", score);
}

fn part2(m: &Vec<Vec<u32>>) {
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for y in 0..m.len() {
        for x in 0..m[y].len() {
            if is_lowest(y, x, m) {
                low_points.push((y, x));
            }
        }
    }

    let mut traversed: HashMap<(usize, usize), bool> = HashMap::new();
    let mut basin_sizes: Vec<i32> = low_points
        .iter()
        .map(|(y, x)| {
            return 1 + traverse_basin(*y, *x, m, &mut traversed);
        })
        .collect::<Vec<i32>>();
    basin_sizes.sort();
    basin_sizes.reverse();
    let score = basin_sizes[0..3].iter().fold(1, |acc, v| acc * v);

    println!("day9 part 2: {}", score);
}

fn traverse_basin(
    y: usize,
    x: usize,
    m: &Vec<Vec<u32>>,
    traversed: &mut HashMap<(usize, usize), bool>,
) -> i32 {
    let val = m[y][x];
    if val == 9 {
        return 0;
    }

    let adjs = get_adjacents(y, x, m);

    let mut basin_count = 0;
    for (y1, x1) in adjs {
        let p = m[y1][x1];
        let did_traverse = traversed.contains_key(&(y1, x1));
        let valid_point = p > val && p < 9;
        if did_traverse || !valid_point {
            continue;
        }
        traversed.insert((y1, x1), true);
        basin_count += 1 + traverse_basin(y1, x1, m, traversed);
    }
    return basin_count;
}

fn is_lowest(y: usize, x: usize, m: &Vec<Vec<u32>>) -> bool {
    let adjs = get_adjacents(y, x, m);
    let val = m[y][x];
    return adjs.iter().all(|(y1, x1)| val < m[*y1][*x1]);
}

fn get_adjacents(y: usize, x: usize, m: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut adjs: Vec<(usize, usize)> = Vec::new();
    if y > 0 {
        adjs.push((y - 1, x));
    }
    if x > 0 {
        adjs.push((y, x - 1));
    }
    if y < m.len() - 1 {
        adjs.push((y + 1, x));
    }
    if x < m[0].len() - 1 {
        adjs.push((y, x + 1));
    }
    return adjs;
}
