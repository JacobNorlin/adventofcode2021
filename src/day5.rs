use crate::utils::read_file_vec;

pub fn main() {
    let lines = read_file_vec("./inputs/day5.txt", "\n");
    let vecs: Vec<(Point, Point)> = lines
        .iter()
        .map(|v| to_points(v))
        .collect::<Vec<(Point, Point)>>();
    let p1 = count_intersections(&vecs, |p1: &Point, p2: &Point| !is_parallel(p1, p2));
    println!("day5 part 1: {}", p1);
    let p2 = count_intersections(&vecs, |p1: &Point, p2: &Point| {
        !is_parallel(p1, p2) && !is_diagonal(p1, p2)
    });
    println!("day5 part 2: {}", p2);
}

fn count_intersections(vecs: &Vec<(Point, Point)>, pred: fn(&Point, &Point) -> bool) -> i32 {
    let mut m: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    let mut count = 0;
    for (p1, p2) in vecs {
        if pred(p1, p2) {
            continue;
        }

        let mut x = p1.x;
        let mut y = p1.y;
        let xd = get_delta(p1.x, p2.x);
        let yd = get_delta(p1.y, p2.y);
        while x != p2.x || y != p2.y {
            if m[y as usize][x as usize] == 1 {
                count += 1;
            }
            m[y as usize][x as usize] += 1;

            x += xd;
            y += yd;
        }

        if m[p2.y as usize][p2.x as usize] == 1 {
            count += 1;
        }
        m[p2.y as usize][p2.x as usize] += 1;
    }

    return count;
}

fn get_delta(a: i32, b: i32) -> i32 {
    return if a < b {
        1
    } else if a > b {
        -1
    } else {
        0
    };
}

fn is_parallel(a: &Point, b: &Point) -> bool {
    return a.x == b.x || a.y == b.y;
}

fn is_diagonal(a: &Point, b: &Point) -> bool {
    return (b.x - a.x).abs() == (b.y - a.y).abs();
}

fn to_points(line: &String) -> (Point, Point) {
    let mut points = line.split(" -> ").map(|v| {
        let mut spl = v.split(",").map(|k| k.parse::<i32>().unwrap());
        (spl.next().unwrap(), spl.next().unwrap())
    });
    let p1 = points.next().unwrap();
    let p2 = points.next().unwrap();
    return (Point { x: p1.0, y: p1.1 }, Point { x: p2.0, y: p2.1 });
}
struct Point {
    x: i32,
    y: i32,
}
