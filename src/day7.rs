use std::fs::read_to_string;

pub fn main() {
    let nums = read_to_string("./inputs/day7.txt")
        .expect("lol")
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    part1(&nums);
    part2(&nums);
}

fn part1(nums: &Vec<i32>) {
    let max = max32(nums);

    let mut min_fuel = 2147483647;
    for i in 0..max {
        let fuel = nums.iter().fold(0, |acc, v| acc + (i - v).abs());
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    println!("day7 part 1: {}", min_fuel);
}

fn part2(nums: &Vec<i32>) {
    let max = max32(nums);

    let mut min_fuel = 2147483647;
    for i in 0..max {
        let fuel = nums.iter().fold(0, |acc, v| {
            //triangle number
            let num_steps = (i - v).abs();
            acc + (num_steps * num_steps + num_steps) / 2
        });
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    println!("day7 part 2: {}", min_fuel);
}

fn max32(arr: &Vec<i32>) -> i32 {
    let mut res = arr[0];
    for &x in arr {
        if x > res {
            res = x;
        }
    }
    res
}
