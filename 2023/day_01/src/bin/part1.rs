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
            let mut iterator = x
                .chars()
                .filter_map(|c| if c.is_numeric() { Some(c) } else { None });
            let first = iterator.next().expect("should be a number");
            let last = iterator.last();

            match last {
                Some(last) => format!("{}{}", first, last),
                None => format!("{}{}", first, first),
            }
            .parse::<usize>()
            .expect("should be a number")
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
                "
                    1abc2
                    pqr3stu8vwx
                    a1b2c3d4e5f
                    treb7uchet
                "
            ),
            142
        );
    }
}
