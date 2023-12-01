fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> u32 {
    input
        .trim_end_matches('\n')
        .split(',')
        .map(hash)
        .sum::<u32>()
}

fn hash(input: &str) -> u32 {
    input
        .bytes()
        .fold(0, |hash, ascii| (hash + ascii as u32) * 17 % 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn hash_test(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, hash(input))
    }
    #[test]
    fn test_process() {
        assert_eq!(
            process_input("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }
}
