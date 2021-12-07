use std::fs::read_to_string;

pub fn main() {
    let nums = read_to_string("./inputs/day6.txt")
        .expect("lol")
        .split(",")
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("day6 part 1: {}", number_of_fish(&nums, 80));
    println!("day6 part 2: {}", number_of_fish(&nums, 256));
}

fn number_of_fish(nums: &Vec<i64>, num_iters: i32) -> i64 {
    let mut map = vec![0; 9];
    //initial state
    for num in nums {
        map[*num as usize] += 1;
    }

    for _i in 0..num_iters {
        let new_fish = map[0];
        map[0] = 0;
        map.rotate_left(1);
        //spawn fish
        map[8] = new_fish;
        map[6] += new_fish;
    }

    return map.iter().fold(0, |acc, v| acc + v);
}
