fn main() {
    let input: u32 = std::fs::read_to_string("input4.txt")
        .unwrap()
        .split('\n')
        .map(|line| {
            line.split(|c| c == '-' || c == ',')
                .flat_map(|num| num.parse::<u32>())
                .collect::<Vec<u32>>()
        })
        .map(|vec| {
            if vec.is_empty() {
                return 0;
            }

            if (vec[0] >= vec[2] && vec[0] <= vec[3])
                || (vec[1] >= vec[2] && vec[1] <= vec[3])
                || (vec[2] >= vec[0] && vec[2] <= vec[1])
                || (vec[3] >= vec[0] && vec[3] <= vec[1])
            {
                return 1;
            }

            0
        })
        .sum();

    println!("{input}");
}
