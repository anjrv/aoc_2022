fn main() {
    let mut input: Vec<u32> = std::fs::read_to_string("input1.txt")
        .unwrap()
        .split("\n\n")
        .map(|line| line.split('\n').flat_map(|num| num.parse::<u32>()).sum())
        .collect();

    input.sort_by(|a, b| b.cmp(a));

    println!("{:?}", input.iter().take(3).sum::<u32>());
}
