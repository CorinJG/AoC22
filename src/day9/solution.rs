use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");

// Position of H relative to T (the rope "state") is represented as a value:
//   |1|2|3|
//   |4|5|6|  T is always in the centre (5)
//   |7|8|9|

// T movements can be represented by values on the same grid, where
// 2 = Up, 8 = Down, 4 = Left, 1 = Diagonally-Up-And-Left etc and 5 means remain in place

/// Given a state and a H movement, update the state and return T's movement
fn rope_move(rope_state: &mut u8, direction: char) -> u8 {
    match (&rope_state, direction) {
        (1, 'L') => { *rope_state = 4; 1 }
        (1, 'R') => { *rope_state = 2; 5 }
        (1, 'U') => { *rope_state = 2; 1 }
        (1, 'D') => { *rope_state = 4; 5 }
        (2, 'L') => { *rope_state = 1; 5 }
        (2, 'R') => { *rope_state = 3; 5 }
        (2, 'U') => { *rope_state = 2; 2 }
        (2, 'D') => { *rope_state = 5; 5 }
        (3, 'L') => { *rope_state = 2; 5 }
        (3, 'R') => { *rope_state = 6; 3 }
        (3, 'U') => { *rope_state = 2; 3 }
        (3, 'D') => { *rope_state = 6; 5 }
        (4, 'L') => { *rope_state = 4; 4 }
        (4, 'R') => { *rope_state = 5; 5 }
        (4, 'U') => { *rope_state = 1; 5 }
        (4, 'D') => { *rope_state = 7; 5 }
        (5, 'L') => { *rope_state = 4; 5 }
        (5, 'R') => { *rope_state = 6; 5 }
        (5, 'U') => { *rope_state = 2; 5 }
        (5, 'D') => { *rope_state = 8; 5 }
        (6, 'L') => { *rope_state = 5; 5 }
        (6, 'R') => { *rope_state = 6; 6 }
        (6, 'U') => { *rope_state = 3; 5 }
        (6, 'D') => { *rope_state = 9; 5 }
        (7, 'L') => { *rope_state = 4; 7 }
        (7, 'R') => { *rope_state = 8; 5 }
        (7, 'U') => { *rope_state = 4; 5 }
        (7, 'D') => { *rope_state = 8; 7 }
        (8, 'L') => { *rope_state = 7; 5 }
        (8, 'R') => { *rope_state = 9; 5 }
        (8, 'U') => { *rope_state = 5; 5 }
        (8, 'D') => { *rope_state = 8; 8 }
        (9, 'L') => { *rope_state = 8; 5 }
        (9, 'R') => { *rope_state = 6; 9 }
        (9, 'U') => { *rope_state = 6; 5 }
        (9, 'D') => { *rope_state = 8; 9 }
        _ => panic!("unexpected rope_state or direction"),
    }
}

fn main() {
    let instructions = INPUT
        .trim()
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(direction, n)| (direction.chars().next().unwrap(), n.parse::<u32>().unwrap()));

    let mut rope_state: u8 = 5; // H and T begin overlapping
    let mut t_coords: (i32, i32) = (0, 0);
    let mut t_coord_set = HashSet::new(); // set of coords T visits, starting at (0,0)
    for (d, n) in instructions {
        for _ in 0..n {
            match rope_move(&mut rope_state, d) {
                1 => { t_coords.0 -= 1; t_coords.1 += 1; }
                2 => { t_coords.1 += 1; }
                3 => { t_coords.0 += 1; t_coords.1 += 1; }
                4 => { t_coords.0 -= 1; }
                5 => {}
                6 => { t_coords.0 += 1; }
                7 => { t_coords.0 -= 1; t_coords.1 -= 1; }
                8 => { t_coords.1 -= 1; }
                9 => { t_coords.0 += 1; t_coords.1 -= 1; }
                _ => panic!("unexpected T movement"),
            };
            t_coord_set.insert(t_coords);
        }
    }
    println!("Part 1: {}", t_coord_set.len());
}
