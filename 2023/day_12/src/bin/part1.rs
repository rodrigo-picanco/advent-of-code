fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let (cfg, nums) = line.split_once(' ').expect("cannot split");
        let num = nums
            .split(',')
            .map(|x| x.parse::<usize>().expect("should be a number"))
            .collect();
        total += count(cfg.chars().collect(), num);
    }
    total
}

fn count(cfg: Vec<char>, nums: Vec<usize>) -> usize {
    if cfg.len() == 0 {
        if nums.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    if nums.len() == 0 {
        if cfg.contains(&'#') {
            return 0;
        } else {
            return 1;
        }
    }

    let mut result = 0;

    if ['?', '.'].contains(&cfg[0]) {
        result += count(cfg[1..].to_vec(), nums.clone());
    }

    if ['?', '#'].contains(&cfg[0]) {
        if (nums[0] <= cfg.len())
            && !cfg[..nums[0]].contains(&'.')
            && (nums[0] == cfg.len() || cfg[nums[0]] != '#')
        {
            let start = cfg
                .get(nums[0] + 1)
                .and_then(|_| Some(cfg[nums[0] + 1..].to_vec()))
                .unwrap_or(vec![]);
            result += count(start, nums[1..].to_vec());
        }
    }

    result
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
