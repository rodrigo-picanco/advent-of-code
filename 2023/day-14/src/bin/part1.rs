fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let grid = parse(input);
    grid.cols()
        .iter()
        .map(|col| {
            col.split(|tile| tile.char == '#')
                .map(|group| {
                    let empty = group.iter().filter(|tile| tile.char == '.').count();
                    group
                        .iter()
                        .rev()
                        .skip(empty)
                        .map(|tile| tile.power)
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    power: usize,
    char: char,
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    height: usize,
}
impl Grid {
    fn cols(&self) -> Vec<Vec<Tile>> {
        (0..self.height)
            .map(|y| self.tiles.iter().map(|x| x[y]).collect())
            .collect()
    }
}

fn parse(input: &str) -> Grid {
    let height = input.lines().count();
    Grid {
        tiles: input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .map(|char| Tile {
                        power: height - y,
                        char,
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect(),
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        assert_eq!(
            process(
                "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            ),
            136
        );
    }
}
