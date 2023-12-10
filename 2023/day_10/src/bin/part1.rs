fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let matrix = Matrix::new(input);
    let moves = matrix
        .start()
        .moves()
        .iter()
        .map(|(_, (x, y))| matrix.get(*x, *y))
        .filter_map(|tile| {
            if tile.moves().iter().len() == 2 {
                Some(tile)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    let farthest = std::iter::successors(Some(moves.clone()), |last_moves| {
        count += 1;
        let symbols = last_moves.iter().map(|tile| tile.symbol()).collect::<Vec<_>>();
        let next_moves = last_moves
            .iter()
            .map(|tile| tile.moves())
            .map(|new_moves| {
                new_moves
                    .iter()
                    .map(|(_, (x, y))| matrix.get(*x, *y))
                    .filter(|tile| {
                        !symbols.contains(&tile.symbol())
                    })
                    .filter(|tile| {
                        tile.symbol() != Symbol::Ground
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

    }).collect::<Vec<_>>();

    count

}

struct Matrix {
    values: Vec<Vec<Tile>>,
}

impl Matrix {
    fn new(map: &str) -> Self {
        Self {
            values: Matrix::parse(map),
        }
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

    fn start(&self) -> &Tile {
        let tile = self
            .values
            .iter()
            .flatten()
            .find(|tile| tile.is_start)
            .expect("no start found");

        tile
    }

    fn get(&self, x: usize, y: usize) -> &Tile {
        self.values
            .get(y)
            .and_then(|row| row.get(x))
            .expect("tile not found")
    }
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
    x: usize,
    y: usize,
}
impl Tile {
    fn new(char: char, x: usize, y: usize) -> Self {
        Self {
            char,
            is_start: char == 'S',
            x,
            y,
        }
    }

    fn symbol(&self) -> Symbol {
        use Symbol::*;

        match self.char {
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

    fn is_move_valid(&self, direction: &Direction) -> bool {
        use Direction::*;
        use Symbol::*;

        match self.symbol() {
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
        }
    }

    fn connections(&self) -> Vec<(Direction, (usize, usize))> {
        use Direction::*;

        let north = (North, (self.x, if self.y > 0 { self.y - 1 } else { 0 }));
        let west = (West, ((if self.x > 0 { self.x - 1 } else { 0 }), self.y));

        vec![
            north,
            (South, (self.x, self.y + 1)),
            (East, (self.x + 1, self.y)),
            west,
        ]
    }

    fn moves(&self) -> Vec<(Direction, (usize, usize))> {
        self.connections()
            .into_iter()
            .filter(|(direction, _)| self.is_move_valid(direction))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_moves() {
        use Direction::*;

        let tiles = vec![
            Tile::new('|', 1, 1),
            Tile::new('-', 1, 1),
            Tile::new('L', 1, 1),
            Tile::new('J', 1, 1),
            Tile::new('F', 1, 1),
            Tile::new('7', 1, 1),
        ];

        let expected = vec![
            vec![(North, (1, 0)), (South, (1, 2))],
            vec![(East, (2, 1)), (West, (0, 1))],
            vec![(North, (1, 0)), (East, (2, 1))],
            vec![(North, (1, 0)), (West, (0, 1))],
            vec![(South, (1, 2)), (East, (2, 1))],
            vec![(South, (1, 2)), (West, (0, 1))],
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
            Matrix::parse(
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
