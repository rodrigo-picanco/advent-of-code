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


    let mut max = 0;
    for row in 0..chars.len() {
        max = max.max(calc_energized_beams(row as i32, -1, 0, 1, &chars));
        max = max.max(calc_energized_beams(row as i32, chars[0].len() as i32, 0, -1, &chars));
    }

    for col in 0..chars[0].len() {
        max = max.max(calc_energized_beams(-1, col as i32, 1, 0, &chars));
        max = max.max(calc_energized_beams(chars.len() as i32, col as i32, -1, 0, &chars));
    }

    max
    

}

fn calc_energized_beams(r: i32, c: i32, dr: i32, dc: i32, chars: &Vec<Vec<char>>) -> usize {
    let mut queue = VecDeque::from([(r, c, dr, dc)]);
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
            51
        )
    }
}
