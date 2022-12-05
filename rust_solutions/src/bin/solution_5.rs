use anyhow::Result;

fn main() -> Result<()> {
    let mut input: Vec<Vec<String>> = std::fs::read_to_string("input5.txt")?
        .split("\n\n")
        .map(|chunk| {
            chunk
                .to_string()
                .split('\n')
                .map(|line| line.to_string())
                .collect()
        })
        .collect();

    let stacks = input[0][0].chars().count() / 4 + 1;
    let mut cargo: Vec<Vec<char>> = Vec::with_capacity(stacks);
    cargo.resize(stacks, Vec::with_capacity(stacks * stacks));

    for line in input[0].iter().rev().skip(1) {
        for (i, c) in line.chars().skip(1).enumerate().step_by(4) {
            if c != ' ' {
                cargo[i / 4].push(c);
            }
        }
    }

    input[1].pop();

    for line in input[1].iter() {
        let mov: Vec<&str> = line.split(' ').collect();
        let n = mov[1].parse::<usize>()?;
        let idx = mov[3].parse::<usize>()? - 1;
        let len = cargo[idx].len();

        let mut crates: Vec<char> = cargo[idx].drain(len - n..len).collect();
        cargo[mov[5].parse::<usize>()? - 1].append(&mut crates);
    }

    println!(
        "{:?}",
        cargo.iter().map(|c| c.last().unwrap()).collect::<String>()
    );

    Ok(())
}
