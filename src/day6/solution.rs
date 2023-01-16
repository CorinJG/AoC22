use std::collections::HashSet;
use std::hash::Hash;

static INPUT: &'static str = include_str!("input.txt");

/// Returns true if an iterator yields no duplicate items, false otherwise.
fn unique_elements<I>(iter: I) -> bool
where
    I: IntoIterator,
    I::Item: Eq + Hash,
{
    let mut set = HashSet::new();
    iter.into_iter().all(|item| set.insert(item))
}

fn main() {
    let input_bytes = INPUT.trim().as_bytes();

    let packet_marker = input_bytes
        .windows(4)
        .position(unique_elements)
        .expect("no packets found")
        + 4;

    println!("packet marker: {}", packet_marker);

    let message_marker = input_bytes
        .windows(14)
        .skip(packet_marker)
        .position(unique_elements)
        .expect("no messages found")
        + packet_marker
        + 14;

    println!("message marker: {}", message_marker);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(unique_elements([1, 2, 3]));
        assert!(!unique_elements([1, 2, 2]));
    }
}
