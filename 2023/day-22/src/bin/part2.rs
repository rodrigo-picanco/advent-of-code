use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    let mut bricks = parse(input);
    bricks.sort_by(|a, b| a[2].cmp(&b[2]));

    for x in 0..bricks.len() {
        let mut max_z = 1;
        for y in 0..x {
            let (brick, check) = (&bricks[x], &bricks[y]);
            if overlaps(brick, check) {
                max_z = max_z.max(check[5] + 1);
            }
        }
        let brick = &mut bricks[x];
        brick[5] -= brick[2] - max_z;
        brick[2] = max_z;
    }

    bricks.sort_by(|a, b| a[2].cmp(&b[2]));

    let mut k_supports_v: HashMap<usize, HashSet<usize>> =
        HashMap::from_iter(bricks.iter().enumerate().map(|(i, _)| (i, HashSet::new())));
    let mut v_supports_k: HashMap<usize, HashSet<usize>> =
        HashMap::from_iter(bricks.iter().enumerate().map(|(i, _)| (i, HashSet::new())));

    for j in 0..bricks.len() {
        for i in 0..j {
            let (upper, lower) = (&bricks[j], &bricks[i]);

            if overlaps(lower, upper) && upper[2] == (lower[5] + 1) {
                k_supports_v.entry(i).and_modify(|x| {
                    x.insert(j);
                });

                v_supports_k.entry(j).and_modify(|x| {
                    x.insert(i);
                });
            }
        }
    }

    let mut total = 0;

    for i in 0..bricks.len() {
        let mut queue = VecDeque::new();

        for &j in &k_supports_v[&i] {
            if v_supports_k[&j].len() == 1 {
                queue.push_back(j);
            }
        }

        let mut falling = HashSet::new();
        falling.extend(&queue);
        falling.insert(i);

        while let Some(j) = queue.pop_front() {
            for k in k_supports_v[&j].difference(&falling.clone()) {
                if v_supports_k[k].is_subset(&falling) {
                    queue.push_back(*k);
                    falling.insert(*k);
                }
            }
        }
        total += falling.len() - 1;
    }

    total
}

fn overlaps(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    a[0].max(b[0]) <= a[3].min(b[3]) && a[1].max(b[1]) <= a[4].min(b[4])
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.replace("~", ",")
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        assert_eq!(
            process_input(
                "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
            ),
            7
        );
    }
}
