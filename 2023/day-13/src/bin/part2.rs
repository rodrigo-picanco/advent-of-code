fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    input.split("\n\n").map(process_pattern).sum()
}

fn process_pattern(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let cols = (0..lines[0].len())
        .map(|i| lines.iter().map(|x| x[i]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;
    let horizontal = find_mirror(lines);
    let vertical = find_mirror(cols);

    result += horizontal * 100;
    result += vertical;
    result

}

fn find_mirror(input: Vec<Vec<char>>) -> usize {
    for r in 0..input.len() {
        let (first, second) = input.split_at(r);

        let mut count = 0;
        for (x, y) in first.iter().rev().zip(second) {
            for (a, b) in x.iter().zip(y.iter()) {
                if a != b {
                    count += 1;
                }
            }
        }

        if count == 1 {
            return r;
        }
    }

    0
}

// LOW 36500

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
            400
        );
    }
}
