use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

#[derive(Debug, PartialEq)]
struct Card {
    winning: HashSet<usize>,
    draw: HashSet<usize>,
}

fn process_input(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|x| parse_line(x))
        .map(|x| x.winning.intersection(&x.draw).copied().collect::<HashSet<usize>>())
        .map(|x| {
            x.iter().enumerate().fold(0, |acc, (i, _)| {
                match i {
                    0 => acc + 1,
                    _ => acc * 2,
                }
            })
        })
        .sum::<usize>()
}


fn parse_num_seq(input: &str) -> HashSet<usize> {
    input
        .split(' ')
        .filter_map(|x| {
            if x.parse::<usize>().is_ok() {
                Some(x.parse::<usize>().unwrap())
            } else {
                None
            }
        })
        .collect::<HashSet<usize>>()
}

fn parse_line(input: &str) -> Card {
    let mut seqs = input.split(":").nth(1).unwrap().split("|");
    let winning = seqs.nth(0).unwrap();
    let draw = seqs.nth(0).unwrap();

    Card {
        winning: parse_num_seq(winning),
        draw: parse_num_seq(draw),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_num_seq() {
        assert_eq!(parse_num_seq("41 48 83"), HashSet::from([41, 48, 83]))
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Card {
                winning: HashSet::from([41, 48, 83, 86, 17]),
                draw: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
            }
        )
    }

    #[test]
    fn test_process() {
        assert_eq!(
            process_input(
                "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
            ),
            13
        );
    }
}
