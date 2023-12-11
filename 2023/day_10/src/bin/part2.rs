use std::collections::BTreeSet;

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

    let mut crossings = 0;
    let mut score = 0;

    let clear_grid = grid.iter().map(|row| {
        row.iter()
            .map(|tile| {
                if visited.iter().find(|v| v.x == tile.x && v.y == tile.y).is_some() {
                   return Tile {
                        char: '.',
                        symbol: Symbol::Ground,
                        is_start: false,
                        x: tile.x,
                        y: tile.y,
                    }
                }

                tile.clone()
            })
            .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    for rows in clear_grid {
        for tile in rows {
            match tile.symbol {
                Symbol::Vertical | Symbol::SouthEast | Symbol::SouthWest => {
                     {
                        crossings += 1;
                    }
                }

                _ => {
                    if crossings % 2 != 0 {
                        score += 1;
                    }
                }
            }
        }
    }

    score
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    let lines = input.lines().enumerate();
    lines
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == 'S' {
                        let char = neighbors(x, y)
                            .into_iter()
                            .filter_map(|(direction, (x, y))| {
                                use Direction::*;
                                let mut set = BTreeSet::new();
                                let move_char = input
                                    .lines()
                                    .map(|x| x.chars().collect())
                                    .collect::<Vec<Vec<char>>>()[y][x];
                                match direction {
                                    North => match move_char {
                                        '7' | 'F' | '|' => {
                                            set.insert('|');
                                            set.insert('L');
                                            set.insert('J');
                                            Some(set)
                                        }
                                        _ => None,
                                    },
                                    South => match move_char {
                                        'L' | 'J' | '|' => {
                                            set.insert('|');
                                            set.insert('F');
                                            set.insert('7');
                                            Some(set)
                                        }
                                        _ => None,
                                    },
                                    East => match move_char {
                                        '-' | 'J' | '7' => {
                                            set.insert('-');
                                            set.insert('F');
                                            set.insert('L');
                                            Some(set)
                                        }
                                        _ => None,
                                    },
                                    West => match move_char {
                                        '-' | 'F' | 'L' => {
                                            set.insert('-');
                                            set.insert('J');
                                            set.insert('7');
                                            Some(set)
                                        }
                                        _ => None,
                                    },
                                }
                            })
                            .fold(
                                vec!['-', 'F', 'J', 'L', '7', '|']
                                    .into_iter()
                                    .collect::<BTreeSet<_>>(),
                                |acc, x| {
                                    acc.intersection(&x).map(|x| *x).collect::<BTreeSet<char>>()
                                },
                            )
                            .first()
                            .expect("not found")
                            .clone();

                        return Tile {
                            char,
                            symbol: get_symbol(char),
                            is_start: true,
                            x,
                            y,
                        };
                    }
                    Tile::new(char, x, y)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn get_symbol(char: char) -> Symbol {
    use Symbol::*;
    match char {
        '|' => Vertical,
        '-' => Horizontal,
        'L' => NorthEast,
        'J' => NorthWest,
        'F' => SouthEast,
        '7' => SouthWest,
        '.' => Ground,
        'S' => Start,
        _ => panic!("invalid symbol"),
    }
}

fn neighbors(x: usize, y: usize) -> Vec<(Direction, (usize, usize))> {
    use Direction::*;
    vec![
        (North, (x, if y > 0 { y - 1 } else { 0 })),
        (South, (x, y + 1)),
        (East, (x + 1, y)),
        (West, ((if x > 0 { x - 1 } else { 0 }), y)),
    ]
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
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

        neighbors(self.x, self.y)
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
                    Tile {
                        char: 'F',
                        symbol: Symbol::SouthEast,
                        is_start: true,
                        x: 1,
                        y: 1,
                    },
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
    fn test_process_large() {
        assert_eq!(
            process(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            10
        )
    }
    #[test]
    fn test_process() {
        assert_eq!(
            process(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            8
        );
    }

    #[test]
    fn test_process_small() {
        assert_eq!(
            process(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"
            ),
            4
        );
    }
}
