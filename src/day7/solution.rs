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

/// Sum the lines of the form '123 xyz' to return the total size of the files on the fs
fn disk_space_used(lines: &mut Peekable<Lines<'_>>) -> u64 {
    lines
        .filter_map(|line| {
            line.split(' ')
                .next()
                .unwrap_or_else(|| panic!("unexpected input line: {line}"))
                .parse::<u64>()
                .ok()
        })
        .sum()
}

/// Similar to dir_size, but update the smallest directory size exceeding the space_required
fn dir_size2(lines: &mut Peekable<Lines<'_>>, space_required: u64, current_min: &mut u64) -> u64 {
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
                        size += dir_size2(lines, space_required, current_min)
                    }
                }
            }
            "l" => {
                // ls
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
    if size >= space_required && size < *current_min {
        *current_min = size;
    }
    size
}

fn main() {
    let mut sum = 0;
    let mut lines = INPUT.trim().lines().peekable();
    lines.next(); // skip the initial 'cd /'
    dir_size(&mut lines, &mut sum);
    println!("part 1: {sum}");

    const TOTAL_DISK_SPACE: u64 = 70_000_000;
    let mut lines = INPUT.trim().lines().peekable();
    let unused_space = TOTAL_DISK_SPACE - disk_space_used(&mut lines);
    let space_required = 30_000_000 - unused_space;
    let mut smallest_delete_dir = TOTAL_DISK_SPACE; // some sufficiently large starting value

    let mut lines = INPUT.trim().lines().peekable();
    dir_size2(&mut lines, space_required, &mut smallest_delete_dir);
    println!("part 2: {smallest_delete_dir}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = include_str!("test_input.txt");

    #[test]
    fn test_dir_size() {
        let mut sum = 0;
        let mut lines = TEST_INPUT.trim().lines().peekable();
        lines.next();
        dir_size(&mut lines, &mut sum);
        assert_eq!(sum, 110);
    }

    #[test]
    fn test_disk_space_used() {
        let mut lines = TEST_INPUT.trim().lines().peekable();
        lines.next();
        let total = disk_space_used(&mut lines);
        assert_eq!(total, 60);
    }
}
