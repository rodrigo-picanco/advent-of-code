use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> isize {
    let input = parse(input);
    input.iter().map(|row| extrapolate(row.clone())).sum()
}

fn extrapolate(input: Vec<isize>) -> isize {
    if input.iter().all(|&x| x == 0) {
        return 0;
    }
    let deltas = input.iter().tuple_windows().map(|(left, right)| right - left).collect();
    let diff = extrapolate(deltas);
    input.last().expect("input is empty") + diff
}

fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().expect("not a number"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45]
            ]
        )
    }
    #[test]
    fn test_process() {
        assert_eq!(
            process(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            114
        );
    }
}
