use array_tool::vec::Intersect;
use itertools::Itertools;
use std::io::{stdin, BufRead};

fn main() {
    let mut priority = 0;

    for lines in &stdin().lock().lines().chunks(3) {
        let shared = lines
            .reduce(|a, b| {
                Ok(a.unwrap()
                    .chars()
                    .collect::<Vec<char>>()
                    .intersect(b.unwrap().chars().collect())
                    .into_iter()
                    .collect::<String>())
            })
            .unwrap();

        if let Ok(val) = shared {
            let p = val.chars().take(1).last().unwrap() as u32;
            priority += if p > 90 { p - 96 } else { p - 38 };
        }
    }

    println!("{priority}");
}
