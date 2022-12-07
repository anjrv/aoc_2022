use anyhow::Result;
use std::collections::HashMap;

const TOTAL_SPACE: i32 = 70000000;
const REQUIRED_SPACE: i32 = 30000000;

fn add(
    size: &i32,
    curr: Vec<String>,
    mut directories: HashMap<String, i32>,
) -> (Vec<String>, HashMap<String, i32>) {
    let mut dir_string: String = "".to_owned();
    for dir in curr.iter() {
        dir_string.push_str(dir);
        directories
            .entry(dir_string.to_owned())
            .and_modify(|s| *s += size)
            .or_insert(*size);
    }

    (curr, directories)
}

fn main() -> Result<()> {
    let mut directories: HashMap<String, i32> = HashMap::with_capacity(100);
    let mut current: Vec<String> = Vec::with_capacity(100);
    let mut size: i32 = 0;

    let mut input: Vec<String> = std::fs::read_to_string("input7.txt")?
        .split('\n')
        .map(|line| line.to_string())
        .collect();

    input.pop();

    for line in input.iter() {
        let l: Vec<String> = line.split(' ').map(|token| token.to_string()).collect();

        if let Ok(num) = l[0].parse::<i32>() {
            size += num;
        }

        if l[1].eq("cd") {
            (current, directories) = add(&size, current, directories);
            size = 0;

            match l[2].as_str() {
                ".." => {
                    current.pop();
                }
                "/" => {
                    current.clear();
                    current.push("/".to_owned());
                }
                _ => {
                    current.push(l[2].to_owned());
                }
            }
        }
    }

    (_, directories) = add(&size, current, directories);

    // Part 1
    // directories.retain(|_, v| *v <= 100000);
    // println!("{:?}", directories.values().sum::<i32>());

    let total_size = directories.get("/").unwrap().to_owned();
    directories.retain(|_, v| *v >= REQUIRED_SPACE - (TOTAL_SPACE - total_size));

    println!(
        "{:?}",
        directories.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap()
    );

    Ok(())
}
