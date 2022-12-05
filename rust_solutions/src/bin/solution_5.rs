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

    // Get rid of index row and empty line
    input[0].pop();
    input[1].pop();

    let stacks = input[0][0].chars().count() / 4 + 1;
    let mut cargo: Vec<Vec<char>> = Vec::with_capacity(stacks);
    cargo.resize(stacks, Vec::with_capacity(stacks * stacks));

    for line in input[0].iter() {
        for (i, c) in line.chars().skip(1).enumerate().step_by(4) {
            if c != ' ' {
                cargo[i / 4].push(c);
            }
        }
    }

    for line in input[1].iter() {
        let mov: Vec<&str> = line.split(' ').collect();
        let crates: Vec<char> = cargo[mov[3].parse::<usize>()? - 1]
            .drain(..mov[1].parse::<usize>()?)
            .collect();

        // Splice is expensive, might be worth doing in reverse or using u8
        cargo[mov[5].parse::<usize>()? - 1].splice(0..0, crates.iter().cloned());
    }

    println!(
        "{:?}",
        cargo.iter().map(|c| c.first().unwrap()).collect::<String>()
    );

    Ok(())
}
