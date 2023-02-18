use std::collections::HashMap;

fn resolve(line: &str, points: &HashMap<&str, i32>) -> i32 {
    let mut sum = 0;

    if let Some(pair) = line.split_once(' ') {
        match pair.1 {
            "X" => {
                let opp = points.get(pair.0).unwrap();
                sum += if *opp == 1 { 3 } else { opp - 1 };
            }
            "Y" => {
                sum += 3 + points.get(pair.0).unwrap();
            }
            _ => {
                let opp = points.get(pair.0).unwrap();
                sum += 6 + if *opp == 3 { 1 } else { opp + 1 };
            }
        }
    }

    sum
}

fn main() {
    let points = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);

    let input: i32 = std::fs::read_to_string("input2.txt")
        .unwrap()
        .split('\n')
        .map(|line| resolve(line, &points))
        .sum();

    println!("{:?}", input);
}
