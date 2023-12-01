fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    input.trim().to_string().split("\n").map(|x| process_line(x)).sum()
}

fn process_line(input: &str) -> usize {
    let mut result = String::new();
    let buffer = input.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();

    result.push(*buffer.first().unwrap());
    result.push(*buffer.last().unwrap());

    result.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process_line("1abc2"), 12);
         assert_eq!(process_line("pqr3stu8vwx"), 38);
         assert_eq!(process_line("a1b2c3d4e5f"), 15);
         assert_eq!(process_line("treb7uchet"), 77);
         assert_eq!(
             process_input(
                 "treb7uchet
 a1b2c3d4e5f"
             ),
             92
         );
         assert_eq!(
             process_input(
                 "treb7uchet"
             ),
             77
         );
    }
}
