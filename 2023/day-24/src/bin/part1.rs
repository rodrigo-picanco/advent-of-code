use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("./input1.txt");
    let range = 200000000000000..=400000000000000;
    let output = process_input(input, range);
    dbg!(output);
}

fn process_input(input: &str, test_area: RangeInclusive<isize>) -> usize {
    let mut hailstones = parse(input);

    let mut total = 0;

    for (i, hs1) in hailstones.iter().enumerate() {
        for hs2 in hailstones.iter().skip(i + 1) {
            let (a1, b1, c1) = (hs1.a, hs1.b, hs1.c);
            let (a2, b2, c2) = (hs2.a, hs2.b, hs2.c);

            // Skip parallel
            if a1 * b2 == a2 * b1 {
                continue;
            }

            let x = (c1 * b2 - c2 * b1) as f64 / (a1 * b2 - a2 * b1) as f64;
            let y = (c2 * a1 - c1 * a2) as f64 / (a1 * b2 - a2 * b1) as f64;

            if x >= *test_area.start() as f64
                && x <= *test_area.end() as f64
                && y >= *test_area.start() as f64
                && y <= *test_area.end() as f64
            {
                if [hs1, hs2]
                    .iter()
                    .all(|hs| (x - hs.sx) * hs.vx >= 0 as f64 && (y - hs.sy) * hs.vy >= 0 as f64)
                {
                    total += 1;
                }
            }
        }
    }

    total
}

#[derive(Debug)]
struct Hailstone {
    sx: f64,
    sy: f64,
    sz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    a: f64,
    b: f64,
    c: f64,
}
impl Hailstone {
    fn new(sx: isize, sy: isize, sz: isize, vx: isize, vy: isize, vz: isize) -> Self {
        Self {
            sx: sx as f64,
            sy: sy as f64,
            sz: sz as f64,
            vx: vx as f64,
            vy: vy as f64,
            vz: vz as f64,
            a: vy as f64,
            b: -vx as f64,
            c: (vy * sx - vx * sy) as f64,
        }
    }
}

fn parse(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let values = line
                .replace("@", ",")
                .split(",")
                .map(|s| s.trim().parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            Hailstone::new(
                values[0], values[1], values[2], values[3], values[4], values[5],
            )
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(process_input(input, 7..=27), 2);
    }
}
