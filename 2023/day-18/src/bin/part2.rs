use nom::{sequence::terminated, IResult};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> i64 {
    let (_, instructions) = parse(input).unwrap();

    let mut boundaries = 0_i64;

    let points = instructions
        .iter()
        .fold(vec![(0, 0)], |mut acc, instruction| {
            boundaries += instruction.distance;
            let (y, x) = acc.last().unwrap();
            let (y, x) = match instruction.direction {
                Direction::Up => (y - instruction.distance, *x),
                Direction::Down => (y + instruction.distance, *x),
                Direction::Left => (*y, x - instruction.distance),
                Direction::Right => (*y, x + instruction.distance),
            };
            acc.push((y, x));
            acc
        });

    // shoelace formula
    let area = points
        .iter()
        .enumerate()
        .map(|(i, (y, x))| {
            let (y2, x2) = points[(i + 1) % points.len()];
            (x2 - x) * (y2 + y)
        })
        .sum::<i64>()
        .abs() as i64
        / 2;

    // Pick's theorem
    let i = area - boundaries / 2_i64 + 1_i64;

    i + boundaries
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    distance: i64,
}
fn parse_line(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = nom::bytes::complete::take_until("(")(input)?;

    let (input, hex) = nom::sequence::delimited(
        nom::bytes::complete::tag("("),
        nom::sequence::preceded(
            nom::bytes::complete::tag("#"),
            nom::character::complete::hex_digit1,
        ),
        nom::bytes::complete::tag(")"),
    )(input)?;

    Ok((
        input,
        Instruction {
            direction: match hex.chars().last().unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => unreachable!(),
            },
            distance: i64::from_str_radix(&hex.chars().take(hex.len() - 1).collect::<String>(), 16)
                .unwrap(),
        },
    ))
}
fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, output) = nom::multi::many1(terminated(
        parse_line,
        nom::branch::alt((nom::character::complete::line_ending, nom::combinator::eof)),
    ))(input)?;

    Ok((input, output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(
            process_input(
                "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"
            ),
            952408144115
        );
    }
}
