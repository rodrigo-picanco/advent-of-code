use std::cmp;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let empty_cols = grid[0]
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .filter_map(|y| {
            if grid.iter().find(|row| row[y] == '#').is_some() {
                None
            } else {
                Some(y)
            }
        })
        .collect::<Vec<_>>();

    let empty_rows = grid
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            if row.iter().all(|char| *char == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let galaxies: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, c)| match *c == '#' {
                    true => Some((x, y)),
                    false => None,
                })
        })
        .collect();

    println!("{:?}", &galaxies);

    let mut result = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (ya, xa) = galaxies[i];
            let (yb, xb) = galaxies[j];

            let x1 = xa.min(xb);
            let x2 = xa.max(xb);
            let y1 = ya.min(yb);
            let y2 = ya.max(yb);

            let er = (x1..x2)
                .filter(|i| empty_rows.contains(&i))
                .collect::<Vec<usize>>()
                .len();

            let ec = (y1..y2)
                .filter(|j| empty_cols.contains(&j))
                .collect::<Vec<usize>>()
                .len();

            let nr = (x2 - x1) - er;
            let nc = (y2 - y1) - ec;

            result += nr + nc + (2 * (ec + er));
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
            process(
                "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            ),
            374
        );
    }
}
