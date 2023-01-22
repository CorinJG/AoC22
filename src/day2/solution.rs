static INPUT: &str = include_str!("input.txt");

fn score_map1(s: &str) -> u32 {
    match s {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => panic!("invalid input"),
    }
}

fn score_map2(s: &str) -> u32 {
    match s {
        "A X" => score_map1("A Z"),
        "A Y" => score_map1("A X"),
        "A Z" => score_map1("A Y"),
        "B X" => score_map1("B X"),
        "B Y" => score_map1("B Y"),
        "B Z" => score_map1("B Z"),
        "C X" => score_map1("C Y"),
        "C Y" => score_map1("C Z"),
        "C Z" => score_map1("C X"),
        _ => panic!("invalid input"),
    }
}

fn main() {
    let score1 = INPUT.trim().lines().map(score_map1).sum::<u32>();

    println!("part 1 score: {}", score1);

    let score2 = INPUT.trim().lines().map(score_map2).sum::<u32>();

    println!("part 2 score: {}", score2);
}
