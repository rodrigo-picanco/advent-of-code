use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let mut total = 0;
    let mut cache = BTreeMap::new();

    for line in input.lines() {
        let (cfg, nums) = line.split_once(' ').expect("cannot split");
        let cfg = ([cfg; 5]).join("?");
        let num: Vec<usize> = nums
            .split(',')
            .map(|x| x.parse::<usize>().expect("should be a number"))
            .collect();
        let num = num.repeat(5);

        total += count(cfg.chars().collect(), num, &mut cache)
    }
    total
}

fn count(cfg: Vec<char>, nums: Vec<usize>, mut cache: &mut BTreeMap<(Vec<char>, Vec<usize>), usize>) -> usize {
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

    if let Some(&result) = cache.get(&(cfg.clone(), nums.clone())) {
        return result;
    }

    let mut result = 0;

    if ['?', '.'].contains(&cfg[0]) {
        result += count(cfg[1..].to_vec(), nums.clone(), &mut cache);
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
            result += count(start, nums[1..].to_vec(), &mut cache);
        }
    }

    
    cache.insert((cfg, nums), result.clone());
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
            525152
        );
    }
}
