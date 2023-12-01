fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let grid = parse(input);
    let start = grid
        .iter()
        .flatten()
        .find(|tile| tile.is_start)
        .expect("no start found");
    let mut queue = vec![start.clone()];
    let mut visited = vec![];
    while let Some(tile) = queue.pop() {
        let moves = tile.moves();
        for (x, y) in moves {
            let tile = grid
                .get(y)
                .and_then(|row| row.get(x))
                .expect("tile not found");
            if !visited.contains(tile) && tile.symbol != Symbol::Ground {
                queue.push(tile.clone());
                visited.push(tile.clone());
            }
        }
    }
    visited.len() / 2
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| Tile::new(char, x, y))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Symbol {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Tile {
    char: char,
    is_start: bool,
    symbol: Symbol,
    x: usize,
    y: usize,
}
impl Tile {
    fn new(char: char, x: usize, y: usize) -> Self {
        use Symbol::*;
        Self {
            char,
            is_start: char == 'S',
            symbol: match char {
                '|' => Vertical,
                '-' => Horizontal,
                'L' => NorthEast,
                'J' => NorthWest,
                'F' => SouthEast,
                '7' => SouthWest,
                '.' => Ground,
                'S' => Start,
                _ => panic!("invalid symbol"),
            },
            x,
            y,
        }
    }
    fn moves(&self) -> Vec<(usize, usize)> {
        use Direction::*;
        use Symbol::*;
        vec![
            (North, (self.x, if self.y > 0 { self.y - 1 } else { 0 })),
            (South, (self.x, self.y + 1)),
            (East, (self.x + 1, self.y)),
            (West, ((if self.x > 0 { self.x - 1 } else { 0 }), self.y)),
        ]
        .into_iter()
        .filter(|(direction, _)| match self.symbol {
            Vertical => match direction {
                North | South => true,
                _ => false,
            },
            Horizontal => match direction {
                East | West => true,
                _ => false,
            },
            NorthEast => match direction {
                North | East => true,
                _ => false,
            },
            NorthWest => match direction {
                North | West => true,
                _ => false,
            },
            SouthEast => match direction {
                South | East => true,
                _ => false,
            },
            SouthWest => match direction {
                South | West => true,
                _ => false,
            },
            Start => true,
            Ground => false,
        })
        .map(|(_, (x, y))| (x, y))
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_moves() {
        let tiles = vec![
            Tile::new('|', 1, 1),
            Tile::new('-', 1, 1),
            Tile::new('L', 1, 1),
            Tile::new('J', 1, 1),
            Tile::new('F', 1, 1),
            Tile::new('7', 1, 1),
        ];
        let expected = vec![
            vec![(1, 0), (1, 2)],
            vec![(2, 1), (0, 1)],
            vec![(1, 0), (2, 1)],
            vec![(1, 0), (0, 1)],
            vec![(1, 2), (2, 1)],
            vec![(1, 2), (0, 1)],
        ];
        tiles
            .into_iter()
            .zip(expected)
            .for_each(|(tile, expected)| {
                assert_eq!(tile.moves(), expected);
            });
    }
    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse(
                ".....
.S-7.
.|.|.
.L-J.
....."
            ),
            vec![
                vec![
                    Tile::new('.', 0, 0),
                    Tile::new('.', 1, 0),
                    Tile::new('.', 2, 0),
                    Tile::new('.', 3, 0),
                    Tile::new('.', 4, 0),
                ],
                vec![
                    Tile::new('.', 0, 1),
                    Tile::new('S', 1, 1),
                    Tile::new('-', 2, 1),
                    Tile::new('7', 3, 1),
                    Tile::new('.', 4, 1),
                ],
                vec![
                    Tile::new('.', 0, 2),
                    Tile::new('|', 1, 2),
                    Tile::new('.', 2, 2),
                    Tile::new('|', 3, 2),
                    Tile::new('.', 4, 2),
                ],
                vec![
                    Tile::new('.', 0, 3),
                    Tile::new('L', 1, 3),
                    Tile::new('-', 2, 3),
                    Tile::new('J', 3, 3),
                    Tile::new('.', 4, 3),
                ],
                vec![
                    Tile::new('.', 0, 4),
                    Tile::new('.', 1, 4),
                    Tile::new('.', 2, 4),
                    Tile::new('.', 3, 4),
                    Tile::new('.', 4, 4),
                ]
            ]
        )
    }
    #[test]
    fn test_process() {
        assert_eq!(
            process(
                ".....
.S-7.
.|.|.
.L-J.
....."
            ),
            4
        );
    }
}
