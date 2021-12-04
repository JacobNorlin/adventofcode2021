use std::fs;

pub fn read_file_vec(path: &str, split: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("broken lol");
    let lines = contents
        .split(split)
        .map(|v| String::from(v))
        .collect::<Vec<String>>();
    return lines;
}
