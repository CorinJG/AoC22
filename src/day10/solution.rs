static INPUT: &str = include_str!("input.txt");


fn main() {
    let mut reg = 1;
    let mut cycle = 0;
    let mut target_cycles = (20..221).step_by(40);
    let mut target_cycle = target_cycles.next().unwrap();
    let mut result1 = 0;
    for line in INPUT.trim().lines() {
        cycle += 1;
        if cycle == target_cycle {
            result1 += target_cycle * reg;
            target_cycle = match target_cycles.next() {
                Some(v) => v,
                None => break,
            }
        }
        if line.starts_with('n') {
            continue;
        } 
        cycle += 1;
        reg += str::parse::<i32>(&line[5..]).unwrap();
        if cycle == target_cycle {
            result1 += target_cycle * reg;
            target_cycle = match target_cycles.next() {
                Some(v) => v,
                None => break,
            }
        }
    }
    println!("result1 = {result1}");

    let mut reg = 1i32;
    let mut crt = [['.'; 40]; 6];
    // crt position starting at (0,0) in top-left
    let mut i = 0;
    let mut j = 0;
    for line in INPUT.trim().lines() {
        if (j as i32 - reg).abs() <= 1 {
            crt[i][j] = '#';
        }
        if j < 39 { j += 1 } else { i += 1; j = 0; }
        if line.starts_with('n') {
            continue;
        }
        if (j as i32 - reg).abs() <= 1 {
            crt[i][j] = '#';
        }
        if j < 39 { j += 1 } else { i += 1; j = 0; }
        reg += str::parse::<i32>(&line[5..]).unwrap();
    }
    for row in crt {
        println!("{:?}", row);
    }
}