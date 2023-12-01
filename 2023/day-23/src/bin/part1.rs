use std::collections::{BTreeMap, BTreeSet, HashSet};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    let grid = parse(input);
    let last = grid.iter().last().unwrap();
    let start = (0, grid[0].iter().position(|&x| x == '.').unwrap());
    let end = (grid.len() - 1, last.iter().position(|&x| x == '.').unwrap());
    let mut points = vec![start, end];
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == '#' {
                continue;
            }
            let mut neighbors = 0;
            let r = r as isize;
            let c = c as isize;
            for (nr, nc) in &[(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
                if *nr < 0
                    || *nc < 0
                    || *nr >= grid.len() as isize
                    || *nc >= row.len() as isize
                    || grid[*nr as usize][*nc as usize] == '#'
                {
                    continue;
                }
                neighbors += 1;
                if neighbors >= 3 {
                    points.push((r as usize, c as usize));
                }
            }
        }
    }

    let mut graph = BTreeMap::new();
    for point in points.clone() {
        graph.insert(point, BTreeMap::new());
    }
    for (sr, sc) in points.clone() {
        let mut stack = vec![(0, sr, sc)];
        let mut seen = HashSet::new();
        seen.insert((sr, sc));
        while let Some((n, r, c)) = stack.pop() {
            if n != 0 && points.contains(&(r, c)) {
                graph.get_mut(&(sr, sc)).unwrap().insert((r, c), n);
                continue;
            }
            for (dr, dc) in dirs(grid[r][c]) {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if in_bounds(&grid, nr, nc)
                    && grid[nr as usize][nc as usize] != '#'
                    && !seen.contains(&(nr as usize, nc as usize))
                {
                    stack.push((n + 1, nr as usize, nc as usize));
                    seen.insert((nr as usize, nc as usize));
                }
            }
        }
    }
    dfs(start, graph, BTreeSet::new(), end)
}

fn dfs(
    pt: (usize, usize),
    graph: BTreeMap<(usize, usize), BTreeMap<(usize, usize), usize>>,
    mut seen: BTreeSet<(usize, usize)>,
    end: (usize, usize),
) -> usize {
    if pt == end {
        return 0;
    }

    let mut max = 0;
    seen.insert(pt);

    for nx in graph.get(&pt).unwrap().keys() {
        max = std::cmp::max(
            max,
            dfs(*nx, graph.clone(), seen.clone(), end) + graph.get(&pt).unwrap().get(nx).unwrap(),
        );
    }

    max
}

fn in_bounds(grid: &[Vec<char>], r: isize, c: isize) -> bool {
    r >= 0 && c >= 0 && r < grid.len() as isize && c < grid[0].len() as isize
}

fn dirs(dir: char) -> Vec<(isize, isize)> {
    match dir {
        '^' => vec![(-1, 0)],
        'v' => vec![(1, 0)],
        '<' => vec![(0, -1)],
        '>' => vec![(0, 1)],
        '.' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        _ => vec![],
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(process_input(input), 94);
    }
}
