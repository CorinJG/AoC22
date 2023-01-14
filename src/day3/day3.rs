static INPUT: &'static str = include_str!("input.txt");

/// Returns char common to first and second half of a str slice of even length
fn common_item(s: &str) -> char {
    let (left, right) = s.split_at(s.len() / 2);
    assert_eq!(left.len(), right.len(), "invalid input - odd number of items in a bag");
    for c in left.chars() {
        if right.contains(c) {
            return c
        }
    } 
    panic!("invalid input - no item present in both compartments");
}

fn priority(c: char) -> u32 {
    match c {
        c @ 'a'..='z' => c as u32 - 96, // a-z is ASCII 97-122
        c @ 'A'..='Z' => c as u32 - 38, // A-Z is ASCII 65-90
        _ => panic!("invalid input - unexpected character"),
    }
}

/// Return the item which is common to all 3 elves' bags
fn determine_badge(a: &str, b: &str, c: &str) -> char {
    for i in a.chars() {
        if b.contains(i) && c.contains(i) {
            return i
        }
    }
    panic!("invalid input - no item common to all 3 bags");
}

fn main() {
    let result1 = INPUT
        .trim()
        .lines()
        .map(|items| common_item(items))
        .map(|common_item| priority(common_item))
        .sum::<u32>();

    println!("answer1: {}", result1);

    let result2 = INPUT
        .trim()
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|bag_triplet| {
            // necessary to eliminate 'refutable pattern' error
            if let [a,b,c] = bag_triplet {
                determine_badge(a, b, c)
            } else {
                panic!("invalid inpuit - number of bags not a multiple of 3")
            }
        })
        .map(|badge| priority(badge))
        .sum::<u32>();

        println!("answer2: {}", result2);
}