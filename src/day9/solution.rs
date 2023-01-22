use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");

// Position of H relative to T (the rope "state") is represented as a value:
//   |1|2|3|
//   |4|5|6|  T is always in the centre (5)
//   |7|8|9|

// Movements can be represented by values on the same grid, where
// 2 = Up, 8 = Down, 4 = Left, 1 = Diagonally-Up-And-Left etc and 5 means remain in place

/// Given a state and a H move direction, return the new state and trailing knot's movement 
/// as a tuple
fn rope_move(mut rope_state: u8, mut dir: u8) -> (u8, u8) {
    // for all state and H moves, the property f(x,y) = f(y,x) holds, where f maps the inputs
    // to new state and T movement 
    // so to simplify match arms, wlog rope_state <= dir
    if rope_state > dir { 
        std::mem::swap(&mut rope_state, &mut dir);
    };
    match (rope_state, dir) {
        // if H doesn't move then no state change, T also doesn't move
        (n, 5) => (n, 5),
        // if H overlaps T at start then state updates to match H movement, T doesn't move
        (5, n) => (n, 5),
        // if H movement matches state, same state again and T movement matches H movement
        (s, d) if s == d => (s, d),

        (1, 2) => (2, 1),
        (1, 3) => (2, 2),
        (1, 4) => (4, 1),
        (1, 6) => (2, 5),
        (1, 7) => (4, 4),
        (1, 8) => (4, 5),
        (1, 9) => (5, 5),

        (2, 3) => (2, 3),
        (2, 4) => (1, 5),
        (2, 6) => (3, 5),
        (2, 7) => (4, 5),
        (2, 8) => (5, 5),
        (2, 9) => (6, 5),

        (3, 4) => (2, 5),
        (3, 6) => (6, 3),
        (3, 7) => (5, 5),
        (3, 8) => (6, 5),
        (3, 9) => (6, 6),

        (4, 6) => (5, 5),
        (4, 7) => (4, 7),
        (4, 8) => (7, 5),
        (4, 9) => (8, 5),

        (6, 7) => (8, 5),        
        (6, 8) => (9, 5),        
        (6, 9) => (6, 9),

        (7, 8) => (8, 7),
        (7, 9) => (8, 8),

        (8, 9) => (8, 9),
        
        _ => panic!("unexpected rope state"),
    }
}

fn main() {
    let instructions = INPUT
        .trim()
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(direction, n)| (
            match direction.chars().next().unwrap() {
                'L' => 4, 
                'R' => 6, 
                'U' => 2, 
                'D' => 8, 
                _ => panic!("unexpected instruction"),
            }, 
            n.parse::<u8>().unwrap())
        )
        .collect::<Vec<_>>();

    let mut rope_state: u8 = 5; // H and T begin overlapping
    let mut t_coords: (i32, i32) = (0, 0);
    let mut t_coord_set = HashSet::new(); // set of coords T visits, starting at (0,0)
    for (d, n) in instructions.iter() {
        for _ in 0..*n {
            let (new_rope_state, t_move) = rope_move(rope_state, *d);
            rope_state = new_rope_state;
            match t_move {
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

    let mut t_coord_set = HashSet::new();
    let mut t_coords: (i32, i32) = (0, 0);
    let mut rope_state = [5; 9]; // initially all knots overlap
    for (d, n) in instructions {
        for _ in 0..n {
            let mut next_knot_movement = d;
            for knot in &mut rope_state {
                (*knot, next_knot_movement) = rope_move(*knot, next_knot_movement);
            }
            match next_knot_movement {
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
    println!("Part 2: {}", t_coord_set.len());
}
