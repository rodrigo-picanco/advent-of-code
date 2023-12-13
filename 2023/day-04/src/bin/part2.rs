use std::{collections::HashSet, vec};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

#[derive(Debug, PartialEq)]
struct Card {
    winning: HashSet<u32>,
    draw: HashSet<u32>,
}

fn process_input(input: &str) -> u32 {
    let mut played = vec![0; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        played[i] += 1;

        let won = parse_line(line)
            .winning
            .intersection(&parse_line(line).draw)
            .copied()
            .collect::<Vec<u32>>();

        for w in 0..won.len() {
            played[w+i+1] += played[i];
        }
    }
    played.iter().sum::<u32>()
}

fn parse_num_seq(input: &str) -> HashSet<u32> {
    input
        .split_ascii_whitespace()
        .filter_map(|x| {
            if x.parse::<u32>().is_ok() {
                Some(x.parse::<u32>().unwrap())
            } else {
                None
            }
        })
        .collect::<HashSet<u32>>()
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
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            30
        );
    }
}
