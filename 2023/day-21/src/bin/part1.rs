use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input, 64);
    dbg!(output);
}

fn process_input(input: &str, steps: u8) -> usize {
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

    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    seen.insert((sc, sr));
    let mut ans = HashSet::new();
    let mut queue = VecDeque::from([(sr, sc, steps)]);

    while let Some((r, c, s)) = queue.pop_front() {
        if s % 2 == 0 {
            ans.insert((r, c));
        }

        if s == 0 {
            continue;
        }

        for (nc, nr) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)].iter() {
            dbg!((nc, nr));
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

    ans.len()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        assert_eq!(
            process_input(
                "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
                6
            ),
            16
        );
    }
}
