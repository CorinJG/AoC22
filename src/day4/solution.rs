static INPUT: &str = include_str!("input.txt");

fn main() {
    let range_bounds = INPUT
        .trim()
        .lines()
        .map(|ranges| {
            let (range1, range2) = ranges.split_once(',').expect("multiple ranges on line");
            let (low1, high1) = range1.split_once('-').expect("invalid range");
            let (low2, high2) = range2.split_once('-').expect("invalid range");
            let low1: u32 = low1.parse().expect("non-integral range bound");
            let high1: u32 = high1.parse().expect("non-integral range bound");
            let low2: u32 = low2.parse().expect("non-integral range bound");
            let high2: u32 = high2.parse().expect("non-integral range bound");
            (low1, high1, low2, high2)
        })
        .collect::<Vec<_>>();

    let solution1 = range_bounds
        .iter()
        .filter(|(low1, high1, low2, high2)| {
            (low1 <= low2 && high1 >= high2) || (low2 <= low1 && high2 >= high1)
        })
        .count();

    println!("part1: {}", solution1);

    let solution2 = range_bounds
        .iter()
        .filter(|(low1, high1, low2, high2)| {
            (low2 >= low1 && low2 <= high1)
                || (high2 >= low1 && high2 <= high1)
                || (low1 >= low2 && high1 <= high2)
        })
        .count();

    println!("part2: {}", solution2);
}
