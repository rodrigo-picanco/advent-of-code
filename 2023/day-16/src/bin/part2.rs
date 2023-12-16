use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    let chars = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut max = 0;
    for row in 0..chars.len() {
        max = max.max(calc_energized_beams(row as i32, -1, 0, 1, &chars));
        max = max.max(calc_energized_beams(
            row as i32,
            chars[0].len() as i32,
            0,
            -1,
            &chars,
        ));
    }
    for col in 0..chars[0].len() {
        max = max.max(calc_energized_beams(-1, col as i32, 1, 0, &chars));
        max = max.max(calc_energized_beams(
            chars.len() as i32,
            col as i32,
            -1,
            0,
            &chars,
        ));
    }
    max
}

fn calc_energized_beams(r: i32, c: i32, dr: i32, dc: i32, chars: &[Vec<char>]) -> usize {
    let mut queue = VecDeque::from([(r, c, dr, dc)]);
    let mut seen = HashSet::new();
    while let Some((r, c, dr, dc)) = queue.pop_front() {
        let r = r + dr;
        let c = c + dc;
        let ch = chars.get(r as usize).and_then(|row| row.get(c as usize));
        let mut insert = |(r, c, dr, dc)| {
            if !seen.contains(&(r, c, dr, dc)) {
                seen.insert((r, c, dr, dc));
                queue.push_back((r, c, dr, dc));
            }
        };
        match ch {
            Some(&ch) => {
                match ch {
                    '.' => insert((r, c, dr, dc)),
                    '/' => {
                        let (dr, dc) = (-dc, -dr);
                        insert((r, c, dr, dc));
                    }
                    '\\' => {
                        let (dr, dc) = (dc, dr);
                        insert((r, c, dr, dc));
                    }
                    '|' if dr != 0 => insert((r, c, dr, dc)),
                    '|' => {
                        for (dr, dc) in [(1, 0), (-1, 0)] {
                            insert((r, c, dr, dc));
                        }
                    }
                    '-' if dc != 0 => insert((r, c, dr, dc)),
                    '-' => {
                        for (dr, dc) in [(0, 1), (0, -1)] {
                            insert((r, c, dr, dc));
                        }
                    }
                    _ => panic!("Unknown char: {ch}"),
                };
            }
            None => continue,
        }
    }
    seen.iter()
        .map(|(r, c, _, _)| (*r, *c))
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        assert_eq!(
            process_input(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            ),
            51
        )
    }
}
