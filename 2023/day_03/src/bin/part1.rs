fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let mut y = 0;
    let x = input
        .trim()
        .lines()
        .map(|x| {
            let result = parse_line(y, x);
            y += 1;
            result
        })
        .reduce(|mut acc, mut x| {
            acc.symbols.append(&mut x.symbols);
            acc.parts.append(&mut x.parts);
            acc
        })
        .unwrap();

    let parts = x.parts;
    let symbols = x.symbols;

    parts
        .iter()
        .filter_map(|part| {
            let upper_row = part.position.y.checked_sub(1).unwrap_or(0);
            let lower_row = part.position.y + 1;
            let left_col = part.position.x.checked_sub(1).unwrap_or(0);
            let right_col = part.position.x + part.number.to_string().len();

            let mut is_part = false;

            symbols.iter().for_each(|symbol| {
                if symbol.position.y >= upper_row
                    && symbol.position.y <= lower_row
                    && symbol.position.x >= left_col
                    && symbol.position.x <= right_col
                {
                    is_part = true
                };
            });

            if is_part {
                Some(part.number)
            } else {
                None
            }
        })
        .sum::<usize>()
}

struct EngineMap {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Part {
    number: usize,
    position: Position,
}

#[derive(Debug)]
struct Symbol {
    position: Position,
    value: char,
}

fn parse_line(y: usize, line: &str) -> EngineMap {
    let mut symbols = vec![];
    let mut parts = vec![];
    let mut x = 0;

    while x < line.len() {
        let char = line.chars().nth(x).unwrap();
        match char {
            '.' => {
                x += 1;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let start = *&x;
                let mut end = *&x;

                while line.chars().nth(x + 1).unwrap_or('a').is_numeric() {
                    end += 1;
                    x += 1;
                }

                end += 1;
                x += 1;

                parts.push(Part {
                    number: line[start..end].parse().expect("shoud be a number"),
                    position: Position { x: start, y },
                });
            }
            _ => {
                symbols.push(Symbol {
                    position: Position { y, x },
                    value: char,
                });
                x += 1;
            }
        };
    }

    return EngineMap {
        parts,
        symbols,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_line() {
        let result = parse_line(0, "...*......");
        let symbols = result.symbols;
        assert_eq!(symbols.get(0).unwrap().value, '*');
        assert_eq!(symbols.get(0).unwrap().position.x, 3);
        assert_eq!(symbols.get(0).unwrap().position.y, 0);
        let result = parse_line(0, "467..114..");
        let parts = result.parts;
        assert_eq!(parts.get(0).unwrap().number, 467);
        assert_eq!(parts.get(0).unwrap().position.x, 0);
        assert_eq!(parts.get(0).unwrap().position.y, 0);
        assert_eq!(parts.get(1).unwrap().number, 114);
        assert_eq!(parts.get(1).unwrap().position.x, 5);
        assert_eq!(parts.get(1).unwrap().position.y, 0);
    }
    #[test]
    fn test_process() {
        assert_eq!(
            process(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            4361
        )
    }
}
