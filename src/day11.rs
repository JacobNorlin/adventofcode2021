use crate::utils::read_file_vec;

pub fn main() {
    let input = read_file_vec("./inputs/day11.txt", "\n")
        .iter()
        .map(|row| {
            row.chars()
                .map(|v| v.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    part1(&mut input.clone());
    part2(&mut input.clone());
}

fn part1(m: &mut Vec<Vec<i32>>) {
    let mut flash_count = 0;
    for _i in 0..100 {
        do_step(m, &mut flash_count);
    }
    println!("day11 part 1: {}", flash_count);
}

fn part2(m: &mut Vec<Vec<i32>>) {
    let mut i = 0;

    loop {
        i += 1;
        let mut flash_count = 0;
        do_step(m, &mut flash_count);
        if flash_count == 100 {
            break;
        }
    }
    println!("day11 part 2: {}", i);
}

fn do_step(m: &mut Vec<Vec<i32>>, flash_count: &mut i32) {
    for y in 0..m.len() {
        for x in 0..m[y].len() {
            if m[y][x] == -1 {
                m[y][x] = 0;
            }
            m[y][x] += 1;
        }
    }

    for y in 0..m.len() {
        for x in 0..m[y].len() {
            let v = m[y][x];

            if v > 9 {
                do_flash(y, x, m, flash_count)
            }
        }
    }
}

fn do_flash(y: usize, x: usize, m: &mut Vec<Vec<i32>>, flash_count: &mut i32) {
    m[y][x] = -1;
    *flash_count += 1;

    for dy in -1..2 {
        for dx in -1..2 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let ay = y as i32 + dy;
            let ax = x as i32 + dx;

            let col_len = m.len() as i32;
            let row_len = m[0].len() as i32;
            if ax >= 0 && ax < row_len && ay >= 0 && ay < col_len {
                let adj_val = m[ay as usize][ax as usize];
                if adj_val >= 0 {
                    m[ay as usize][ax as usize] += 1;
                    if adj_val >= 9 {
                        do_flash(ay as usize, ax as usize, m, flash_count);
                    }
                }
            }
        }
    }
}
