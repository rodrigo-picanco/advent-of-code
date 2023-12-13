fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    input.split("\n\n").map(process_pattern).sum()
}

const HORIZONTAL_FACTOR: usize = 100;
const VERTICAL_FACTOR: usize = 1;

fn process_pattern(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let cols = (0..lines[0].len())
        .map(|i| lines.iter().map(|x| x[i]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let horizontal = find_mirror(lines);
    let vertical = find_mirror(cols);

    [(horizontal, HORIZONTAL_FACTOR), (vertical, VERTICAL_FACTOR)]
        .iter()
        .find_map(|(x, y)| x.as_ref().map(|x| x * y))
        .expect("No pattern found")
}

fn find_mirror(input: Vec<Vec<char>>) -> Option<usize> {
    input
        .iter()
        .enumerate()
        .zip(input.iter().skip(1))
        .filter_map(|((i, a), b)| if a == b { Some(i) } else { None })
        .find_map(|x| {
            let (first, second) = input.split_at(x + 1);

            let second = second.iter().collect::<Vec<_>>();
            let first = first.iter().rev().take(second.len()).collect::<Vec<_>>();

            if first == second {
                Some(x + 1)
            } else {
                None
            }
        })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(
            process(
                "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            405
        );
    }
}
