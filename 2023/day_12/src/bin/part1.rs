use itertools::Itertools;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let mut map = parse(input);
    let mut acc = vec![];

    for (springs, _) in map.iter_mut() {
        for x in 0..springs.len() {
            let char = springs[x];
            if char == '?' {
                springs[x] = '.';
                acc.push(springs.clone());
                springs[x] = '#';
                acc.push(springs.clone());
                springs[x] = char;
            } 
        }
    }

    dbg!(acc);

        

    todo!()
}

fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    // split input into lines
    input
        .lines()
        .map(|line| {
            assert!(line.len() > 0, "line is empty");
            let mut parts = line.split(' ');
            (
                parts.next().expect("cannot parse part1").chars().collect(),
                parts
                    .next()
                    .expect("cannot parse part2")
                    .split(",")
                    .map(|x| x.parse::<usize>().expect("should be a number"))
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(
            process(
                "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            ),
            21
        );
    }
}
