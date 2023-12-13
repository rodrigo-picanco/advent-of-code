fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
}

fn count(cfg: Vec<char>, nums: Vec<usize>) -> usize {

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
