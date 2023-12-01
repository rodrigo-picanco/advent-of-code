use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input, 26501365);
    dbg!(output);
}

// 621289922886145 too low???

fn fill(sr: i64, sc: i64, ss: i64, grid: &Vec<Vec<char>>) -> i64 {
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    seen.insert((sc, sr));
    let mut ans = HashSet::new();
    let mut queue = VecDeque::from([(sr, sc, ss)]);

    while let Some((r, c, s)) = queue.pop_front() {
        if s % 2 == 0 {
            ans.insert((r, c));
        }

        if s == 0 {
            continue;
        }

        for (nc, nr) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)].iter() {
            if *nr < 0
                || *nr >= grid.len() as i64
                || *nc < 0
                || *nc >= grid[0].len() as i64
                || grid[*nr as usize][*nc as usize] == '#'
                || seen.contains(&(*nc, *nr))
            {
                continue;
            } else {
                seen.insert((*nc, *nr));
                queue.push_back((*nc, *nr, s - 1));
            }
        }
    }

    ans.len() as i64
}

fn process_input(input: &str, steps: i64) -> i64 {
    let grid = parse_input(input);
    let (sr, sc) = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, c)| {
                if *c == 'S' {
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .expect("No starting position found");

    let size = grid.len() as i64;

    assert_eq!(grid.len(), grid[0].len());
    assert_eq!(sr, (grid.len() as i64 / 2));
    assert_eq!(sc, (grid.len() as i64 / 2));

    let width = steps as i64 / size - 1;

    let odd = (width / 2 * 2 + 1).pow(2);
    let even = ((width + 1) / 2 * 2).pow(2);

    let odd_points = fill(sr, sc, size * 2 + 1, &grid);
    let even_points = fill(sr, sc, size * 2, &grid);

    let corner_t = fill(size - 1, sc, size - 1, &grid);
    let corner_r = fill(sr, 0, size - 1, &grid);
    let corner_b = fill(0, sc, size - 1, &grid);
    let corner_l = fill(sr, size - 1, size - 1, &grid);

    let small_tr = fill(size - 1, 0, size / 2 - 1, &grid);
    let small_tl = fill(size - 1, size - 1, size / 2 - 1, &grid);
    let small_br = fill(0, 0, size / 2 - 1, &grid);
    let small_bl = fill(0, size - 1, size / 2 - 1, &grid);

    let large_tr = fill(size - 1, 0, size * 3 / 2 - 1, &grid);
    let large_tl = fill(size - 1, size - 1, size * 3 / 2 - 1, &grid);
    let large_br = fill(0, 0, size * 3 / 2 - 1, &grid);
    let large_bl = fill(0, size - 1, size * 3 / 2 - 1, &grid);

    odd * odd_points
        + even * even_points
        + corner_t
        + corner_r
        + corner_b
        + corner_l
        + (width + 1) * (small_tr + small_tl + small_br + small_bl)
        + width * (large_tr + large_tl + large_br + large_bl)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
