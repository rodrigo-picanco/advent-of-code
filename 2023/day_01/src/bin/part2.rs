fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    input
        .trim()
        .to_string()
        .split("\n")
        .map(|x| process_line(x))
        .sum()
}

fn process_line(input: &str) -> usize {
    let mut result = String::new();
    let mut subarrays = Vec::new();

    for x in 0..input.len() {
        for y in x..input.len() {
            subarrays.push(&input[x..y + 1]);
        }
    }

    let mut buffer: Vec<char> = subarrays.iter().map(|x| map(x)).collect();

    buffer = buffer
        .iter()
        .filter(|x| **x != '0')
        .map(|x| *x as char)
        .collect();

    result.push(buffer[0]);
    result.push(buffer.pop().unwrap());
    result.parse::<usize>().unwrap()
}

fn map(input: &str) -> char {
    match input {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => {
            if input.len() == 1 && input.chars().nth(0).unwrap().is_numeric() {
                input.chars().nth(0).unwrap()
            } else {
                '0'
            }
        }
    }
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
