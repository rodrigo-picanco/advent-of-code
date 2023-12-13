fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|x| {
            let chars: Vec<_> = x
                .chars()
                .filter_map(|c| c.is_numeric().then_some(c))
                .collect();
            let first = chars.first().expect("should be a number");
            let last = chars.last();
            format!("{}{}", first, last.unwrap_or(first)).parse::<usize>().expect("should be a number")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        assert_eq!(
            process_input(
                "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }
}
