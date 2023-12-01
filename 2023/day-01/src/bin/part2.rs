fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    input.lines().map(process_line).sum()
}

fn process_line(input: &str) -> usize {
    let mut result = String::new();

    dbg!(input);

    let nums: Vec<char> = input
        .chars()
        .enumerate()
        .flat_map(|(i, _)| {
            input
                .chars()
                .enumerate()
                .skip(i)
                .map(|(j, _)| input[i..=j].to_string())
                .collect::<Vec<String>>()
        })
        .filter_map(|x| match x.as_str() {
            "one" => Some('1'),
            "two" => Some('2'),
            "three" => Some('3'),
            "four" => Some('4'),
            "five" => Some('5'),
            "six" => Some('6'),
            "seven" => Some('7'),
            "eight" => Some('8'),
            "nine" => Some('9'),
            _ => {
                if x.len() == 1 {
                    if x.chars().next().unwrap().is_numeric() {
                        Some(x.chars().next().unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        })
        .collect();

    result.push(*nums.first().unwrap());
    result.push(*nums.last().unwrap());
    result.parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process_line("two1nine"), 29);
        assert_eq!(process_line("eightwothree"), 83);
        assert_eq!(process_line("abcone2threexyz"), 13);
        assert_eq!(process_line("xtwone3four"), 24);
        assert_eq!(process_line("4nineeightseven2"), 42);
        assert_eq!(process_line("zoneight234"), 14);
        assert_eq!(process_line("7pqrstsixteen"), 76);
    }
}
