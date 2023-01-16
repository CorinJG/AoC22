static INPUT: &'static str = include_str!("input.txt");

fn move_crates_9000(stacks: &mut [Vec<char>], num: u32, src: usize, dest: usize) {
    for _ in 0..num {
        // input stack indexes begin at 1 so must subtract 1
        let popped_crate = stacks[src - 1]
            .pop()
            .expect("instruction invalid - stack has insufficient crates");
        stacks[dest - 1].push(popped_crate);
    }
}

fn move_crates_9001(stacks: &mut [Vec<char>], num: u32, src: usize, dest: usize) {
    let mut popped_crates: Vec<char> = Vec::new();
    for _ in 0..num {
        popped_crates.push(
            stacks[src - 1]
                .pop()
                .expect("instruction invalid - stack has insufficient crates"),
        );
    }
    // empty the stack of popped crates, popping onto dest stack
    while let Some(c) = popped_crates.pop() {
        stacks[dest - 1].push(c);
    }
}

fn main() {
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(9);
    stacks.push(['D', 'B', 'J', 'V'].to_vec());
    stacks.push(['P', 'V', 'B', 'W', 'R', 'D', 'F'].to_vec());
    stacks.push(['R', 'G', 'F', 'L', 'D', 'C', 'W', 'Q'].to_vec());
    stacks.push(['W', 'J', 'P', 'M', 'L', 'N', 'D', 'B'].to_vec());
    stacks.push(['H', 'N', 'B', 'P', 'C', 'S', 'Q'].to_vec());
    stacks.push(['R', 'D', 'B', 'S', 'N', 'G'].to_vec());
    stacks.push(['Z', 'B', 'P', 'M', 'Q', 'F', 'S', 'H'].to_vec());
    stacks.push(['W', 'L', 'F'].to_vec());
    stacks.push(['S', 'V', 'F', 'M', 'R'].to_vec());

    // copy of initial crate state for part 2
    let mut stacks2 = stacks.clone();

    for instruction in INPUT.trim().lines() {
        let instruction_vec: Vec<&str> = instruction.split(' ').collect();
        let [_, num, _, src, _, dest] = instruction_vec.as_slice() else {
            panic!("instruction of unexpected format");
        };
        let num: u32 = num.parse().expect("number of crates isn't a number");
        let src: usize = src.parse().expect("source stack isn't a number");
        let dest: usize = dest.parse().expect("dest stack isn't a number");
        move_crates_9000(&mut stacks, num, src, dest);
        move_crates_9001(&mut stacks2, num, src, dest);
    }

    let solution1 = stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>();
    println!("part 1: {}", solution1);

    let solution2 = stacks2
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>();
    println!("part 2: {}", solution2);
}
