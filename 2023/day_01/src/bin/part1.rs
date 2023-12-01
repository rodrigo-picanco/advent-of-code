fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> i32 {
    let mut buffer: Vec<i32> = vec![];
    let lines = input.to_string();

    for line in lines.split("\n") {
        buffer.push(process_line(line).parse().unwrap());
    }

    buffer.iter().sum()
}

fn process_line(input: &str) -> String {
    let mut buffer = vec![];
    let mut result = String::new();

    for c in input.chars() {
        if c.is_numeric() {
            buffer.push(c);
        }
    }

    if buffer.len() == 0 {
        return "00".to_string()
    }

    result.push(buffer[0]);
    result.push(buffer.pop().unwrap());

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process_line("1abc2"), "12");
        assert_eq!(process_line("pqr3stu8vwx"), "38");
        assert_eq!(process_line("a1b2c3d4e5f"), "15");
        assert_eq!(process_line("treb7uchet"), "77");
        assert_eq!(
            process_input(
                "treb7uchet
a1b2c3d4e5f"
            ),
            92
        );
        assert_eq!(
            process_input(
                "treb7uchet
abcdef"
            ),
            77
        );
    }
}
