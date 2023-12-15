fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    calc_north_load(string_to_grid(&tilt(input, Direction::North)))
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn tilt(input: &str, direction: Direction) -> String {
    use Direction::*;

    match direction {
        North | South => grid_to_string(rotate(
            rotate(string_to_grid(input))
                .into_iter()
                .map(|col| {
                    col.split(|x| x == &'#')
                        .map(|group| group.to_vec())
                        .map(|mut group| {
                            group.sort();
                            match direction {
                                North => group.iter().rev().collect::<String>(),
                                South => group.iter().collect::<String>(),
                                _ => panic!("invalid direction"),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("#")
                })
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )),

        East | West => grid_to_string(
            string_to_grid(input)
                .into_iter()
                .map(|col| {
                    col.split(|x| x == &'#')
                        .map(|group| group.to_vec())
                        .map(|mut group| {
                            group.sort();
                            match direction {
                                East => group.iter().rev().collect::<String>(),
                                West => group.iter().collect::<String>(),
                                _ => panic!("invalid direction"),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("#")
                })
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        ),
    }
}

fn rotate(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..input.len())
        .map(|y| input.iter().map(move |line| line[y]).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn grid_to_string(grid: Vec<Vec<char>>) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn string_to_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn calc_north_load(input: Vec<Vec<char>>) -> usize {
    input.iter()
        .enumerate()
        .map(|(r, row)| row.iter().filter(|x| **x == 'O').count() * (input.len() - r))
        .sum()
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
