static INPUT: &'static str = include_str!("input.txt");

/// Given a row of trees, iterate once to return a mask representing which trees are visible
fn visible_mask<'a, I>(iter: I) -> Vec<bool>
where
    I: Iterator<Item = &'a char> + DoubleEndedIterator + 'a,
{
    let mut tallest_so_far = '/'; // ASCII '/' precedes all numerical digits
    iter.map(move |tree| {
        if *tree > tallest_so_far {
            tallest_so_far = *tree;
            true
        } else {
            false
        }
    })
    .collect()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn main() {
    let grid = INPUT
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let lr_mask: Vec<Vec<bool>> = grid.iter().map(|row| visible_mask(row.iter())).collect();

    let rl_mask: Vec<Vec<bool>> = grid
        .iter()
        .map(|row| visible_mask(row.iter().rev()).into_iter().rev().collect())
        .collect();

    let grid_transpose = transpose(grid);

    let tb_mask: Vec<Vec<bool>> = transpose(
        grid_transpose
            .iter()
            .map(|row| visible_mask(row.iter()))
            .collect(),
    );

    let bt_mask: Vec<Vec<bool>> = transpose(
        grid_transpose
            .iter()
            .map(|row| visible_mask(row.iter().rev()).into_iter().rev().collect())
            .collect(),
    );

    let side_len = grid_transpose.len();

    let result = (0..side_len)
        .map(|i| {
            (0..side_len)
                .filter(|&j| lr_mask[i][j] || rl_mask[i][j] || tb_mask[i][j] || bt_mask[i][j])
                .count()
        })
        .sum::<usize>();

    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible_mask() {
        let test_vec1 = vec!['a', 'b', 'c', 'd', 'e'];
        let test_vec2 = vec!['e', 'd', 'c', 'b', 'a'];
        let test_vec3 = vec!['e', 'a', 't', 'g', 'z'];
        assert_eq!(
            visible_mask(&mut test_vec1.iter()),
            vec![true, true, true, true, true]
        );
        assert_eq!(
            visible_mask(&mut test_vec2.iter()),
            vec![true, false, false, false, false]
        );
        assert_eq!(
            visible_mask(&mut test_vec3.iter()),
            vec![true, false, true, false, true]
        );
        assert_eq!(
            visible_mask(&mut test_vec1.iter().rev()),
            vec![true, false, false, false, false]
        );
        assert_eq!(
            visible_mask(&mut test_vec2.iter().rev()),
            vec![true, true, true, true, true]
        );
        assert_eq!(
            visible_mask(&mut test_vec3.iter().rev()),
            vec![true, false, false, false, false]
        );
    }

    #[test]
    fn test_transpose_mask() {
        let test_vec = vec![
            vec!['m', 'c', 'h'],
            vec!['d', 't', 'n'],
            vec!['a', 'o', 'c'],
        ];
        let test_transpose = transpose(test_vec);
        assert_eq!(
            test_transpose,
            vec!(
                vec!('m', 'd', 'a'),
                vec!('c', 't', 'o'),
                vec!('h', 'n', 'c')
            )
        );
        let top_mask = transpose(
            test_transpose
                .iter()
                .map(|row| visible_mask(&mut row.iter()))
                .collect::<Vec<Vec<bool>>>(),
        );
        assert_eq!(
            top_mask,
            vec!(
                vec!(true, true, true),
                vec!(false, true, true),
                vec!(false, false, false)
            )
        );
        let bot_mask = transpose(
            test_transpose
                .iter()
                .map(|row| {
                    visible_mask(&mut row.iter().rev())
                        .into_iter()
                        .rev()
                        .collect()
                })
                .collect::<Vec<Vec<bool>>>(),
        );
        assert_eq!(
            bot_mask,
            vec!(
                vec!(true, false, false),
                vec!(true, true, true),
                vec!(true, true, true)
            )
        );
    }
}
