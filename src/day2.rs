use crate::utils::read_file_vec;

pub fn main() {
    let lines = read_file_vec("./inputs/day2.txt");
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut depth = 0;
    let mut hori = 0;
    for line in lines {
        let mut tup = line.splitn(2, " ");
        let dir = tup.next().unwrap();
        let num = tup.next().unwrap().parse::<i32>().unwrap();
        match dir {
            "forward" => hori = hori + num,
            "down" => depth = depth + num,
            "up" => depth = depth - num,
            _ => println!("lmao"),
        }
    }
    println!("day1 part 1: {}", depth * hori);
}

fn part2(lines: &Vec<String>) {
    let mut aim = 0;
    let mut hori = 0;
    let mut depth = 0;

    for line in lines {
        let mut tup = line.splitn(2, " ");
        let dir = tup.next().unwrap();
        let num = tup.next().unwrap().parse::<i32>().unwrap();
        match dir {
            "forward" => {
                hori = hori + num;
                depth = depth + aim * num;
            }
            "down" => aim = aim + num,
            "up" => aim = aim - num,
            _ => println!("lmao"),
        }
    }
    println!("day1 part 2: {}", depth * hori);
}
