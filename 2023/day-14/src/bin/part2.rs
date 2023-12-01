fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let mut count = 0;
    let mut result = input.to_string();
    let mut seen = vec![result.clone()];

    loop {
        count += 1;
        result = cycle(&result);
        if seen.contains(&result) {
            break;
        }
        seen.push(result.clone());
    }

    let first = seen.iter().position(|x| x == &result).expect("acc empty");

    let pos = (1_000_000_000 - first) % (count - first) + first;

    result = seen[pos].clone();

    println!("{}", result);

    calc_north_load(encode(&result))
}

fn cycle(input: &str) -> String {
    let mut result = tilt(input, Direction::North);
    result = tilt(&result, Direction::West);
    result = tilt(&result, Direction::South);
    result = tilt(&result, Direction::East);
    result
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
        North | South => decode(rotate(
            rotate(encode(input))
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

        East | West => decode(
            encode(input)
                .into_iter()
                .map(|col| {
                    col.split(|x| x == &'#')
                        .map(|group| group.to_vec())
                        .map(|mut group| {
                            group.sort();
                            match direction {
                                East => group.iter().collect::<String>(),
                                West => group.iter().rev().collect::<String>(),
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

fn decode(grid: Vec<Vec<char>>) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn encode(input: &str) -> Vec<Vec<char>> {
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
            64
        );
    }
}
