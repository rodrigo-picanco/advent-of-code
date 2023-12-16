use std::collections::{VecDeque, HashSet};

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

    let mut queue = VecDeque::from([(0, -1, 0, 1)]);
    let mut seen = HashSet::new();

    while let Some((r, c, dr, dc)) = queue.pop_front() {
        let r = r + dr;
        let c = c + dc;

        // Skip Out of bounds
        if r < 0 || r >= chars.len() as i32 || c < 0 || c >= chars[0].len() as i32 {
            continue;
        }

        let ch = chars[r as usize][c as usize];

        if ch == '.' || (ch == '-' && dc != 0) || (ch == '|' && dr != 0) {
            if !seen.contains(&(r, c, dr, dc)) {
                seen.insert((r, c, dr, dc));
                queue.push_back((r, c, dr, dc));
            }
        } else if ch == '/' {
            let (dr, dc) = (-dc, -dr);
            if !seen.contains(&(r, c, dr, dc)) {
                seen.insert((r, c, dr, dc));
                queue.push_back((r, c, dr, dc));
            }
        } else if ch == '\\' {
            let (dr, dc) = (dc, dr);
            if !seen.contains(&(r, c, dr, dc)) {
                seen.insert((r, c, dr, dc));
                queue.push_back((r, c, dr, dc));
            }
        } else {
            let moves = if ch == '|' {
                [(1, 0), (-1, 0)]
            } else {
                [(0, 1), (0, -1)]
            };
            
            for (dr, dc) in moves {
                if !seen.contains(&(r, c, dr, dc)) {
                    seen.insert((r, c, dr, dc));
                    queue.push_back((r, c, dr, dc));
                }
            }
        }
    }

    let coors = seen.iter().map(|(r, c, _, _)| (*r, *c)).collect::<HashSet<_>>();

    coors.len()
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
            46
        )
    }
}
