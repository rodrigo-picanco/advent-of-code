use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Reverse;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| character.to_digit(10).expect("invalid input") as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut seen: HashSet<(i32, i32, i32, i32, i32)> = HashSet::new();
    let mut queue = BinaryHeap::new();

    queue.push(Reverse((0, 0, 0, 0, 0, 0)));

    let mut result = 0_i32;
    while let Some(item) = queue.pop() {
        let Reverse((hl, r, c, dr, dc, n)) = item;

        if r == (grid.len() - 1) as i32 && c == (grid[0].len() - 1) as i32 {
            result = hl;
            break;
        }

        if seen.contains(&(r, c, dr, dc, n)) {
            continue;
        } 

        seen.insert((r, c, dr, dc, n));

        if n < 3 && (dr, dc) != (0, 0) {
            let nx = r + dr;
            let ny = c + dc;

            if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 {
                queue.push(Reverse((
                    hl + grid[nx as usize][ny as usize],
                    nx,
                    ny,
                    dr,
                    dc,
                    n + 1,
                )));
            }
        }

        for (ndr, ndc) in [(0,  1), (0, -1), (1, 0), (-1, 0)] {
            if (ndr, ndc) != (dr, dc) && (ndr, ndc) != (-dr, -dc) {
                let nx = r + ndr;
                let ny = c + ndc;

                if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 {
                    queue.push(Reverse((
                        hl + grid[nx as usize][ny as usize],
                        nx,
                        ny,
                        ndr,
                        ndc,
                        1,
                    )));
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        assert_eq!(
            process_input(
                "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            ),
            102
        );
    }
}
