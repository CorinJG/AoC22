static INPUT: &'static str = include_str!("input.txt");

fn main() {
    let mut triplet_sum: Vec<u32> = INPUT
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|num| num.parse::<u32>().expect("failed to parse number"))
                .sum::<u32>()
        })
        .collect();

    triplet_sum.sort();

    println!("part1: {}", triplet_sum.last().expect("empty input"));
    println!(
        "part2: {}",
        triplet_sum[(triplet_sum.len() - 3)..].iter().sum::<u32>()
    );
}
