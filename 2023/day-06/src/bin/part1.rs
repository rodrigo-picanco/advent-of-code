use nom::bytes::complete::is_not;
use nom::character::complete::{self, line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::ParserExt;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    parser(input)
        .iter()
        .map(|x| x.record_beater_calculator().len() as usize)
        .product::<usize>()
}

#[derive(Debug, PartialEq)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn new(time: usize, record: usize) -> Self {
        Self { time, record }
    }
    fn record_beater_calculator(&self) -> Vec<usize> {
        (0..self.time)
            .filter_map(|x| {
                if ((self.time - x) * x) > self.record {
                    Some(x)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>()
    }
}

fn parse_times(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(
        is_not("0123456789").precedes(separated_list1(space1, complete::u32)),
        line_ending,
        is_not("0123456789").precedes(separated_list1(space1, complete::u32)),
    )
    .parse(input)
}

fn parser(input: &str) -> Vec<Race> {
    let (_, (times, records)) = parse_times(input).expect("Failed to parse input");
    times
        .iter()
        .zip(records)
        .map(|(time, record)| Race::new(*time as usize, record as usize))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() {
        assert_eq!(Race::new(7, 9).record_beater_calculator(), vec![2, 3, 4, 5])
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            parser(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]
        )
    }

    #[test]
    fn test_process() {
        assert_eq!(
            process(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            288
        );
    }
}
