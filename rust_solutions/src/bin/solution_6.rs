use anyhow::Result;
use std::collections::HashMap;

const NUM_DISTINCT: usize = 14;

fn increment(c: &char, mut s: HashMap<char, u32>) -> HashMap<char, u32> {
    s.entry(*c).and_modify(|counter| *counter += 1).or_insert(1);

    s
}

fn main() -> Result<()> {
    let input: Vec<char> = std::fs::read_to_string("input6.txt")?.chars().collect();
    let mut signal: HashMap<char, u32> = HashMap::new();

    for char in input.iter().take(NUM_DISTINCT) {
        signal = increment(char, signal);
    }

    for (idx, char) in input.iter().skip(NUM_DISTINCT).enumerate() {
        if signal.len() == NUM_DISTINCT {
            println!("{}", idx + NUM_DISTINCT);
            return Ok(());
        }

        signal = increment(char, signal);
        let old = signal.get_mut(&input[idx]).unwrap();

        if *old > 1 {
            *old -= 1;
        } else {
            signal.remove(&input[idx]);
        }
    }

    Ok(())
}
