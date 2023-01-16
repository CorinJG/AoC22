use std::iter::Peekable;
use std::str::Lines;

static INPUT: &'static str = include_str!("input.txt");

/// To find the sum of the sizes of all files in this dir and nested subdirs,
/// iterate over the lines in the file, updating the sum for dirs <= 100_000.
fn dir_size(lines: &mut Peekable<Lines<'_>>, sum: &mut u64) -> u64 {
    let mut size = 0;
    while let Some(line) = lines.next() {
        match &line[2..3] {
            "c" => {
                // either '$ cd ..' or '$ cd xyz'
                match &line[5..6] {
                    "." => {
                        // cd ..
                        break;
                    }
                    _ => {
                        // cd xyz
                        size += dir_size(lines, sum)
                    }
                }
            }
            "l" => {
                // ls
                // advance through output until we encounter a new command
                size = std::iter::from_fn(|| lines.next_if(|line| &line[0..1] != "$"))
                    .filter(|line| &line[0..1] != "d")
                    .map(|line| {
                        line.split(' ')
                            .next()
                            .expect("unexpected ls output line")
                            .parse::<u64>()
                            .expect("non-numerical file size")
                    })
                    .sum();
            }
            _ => {
                panic!("unexpected line in input: {line}");
            }
        };
    }
    if size <= 100_000 {
        *sum += size;
    }
    size
}

fn main() {
    let mut sum = 0;
    let mut lines = INPUT.trim().lines().peekable();
    lines.next(); // skip the initial 'cd /'
    dir_size(&mut lines, &mut sum);
    println!("part 1: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = include_str!("test_input.txt");

    #[test]
    fn test_input1() {
        let mut sum = 0;
        let mut lines = TEST_INPUT.trim().lines().peekable();
        lines.next();
        dir_size(&mut lines, &mut sum);
        assert_eq!(sum, 110);
    }
}
