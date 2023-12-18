use nom::IResult;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    let (_, instructions) = parse(input).unwrap();

    let mut boundaries = 0;

    let points = instructions
        .iter()
        .fold(vec![(0, 0)], |mut acc, instruction| {
            boundaries += instruction.distance as usize;

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
        .sum::<i32>()
        .abs() as usize
        / 2;

    let i = area - boundaries / 2 + 1;

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
    distance: i32,
}
fn parse_line(input: &str) -> IResult<&str, Instruction> {
    let (input, (direction, distance)) = nom::sequence::separated_pair(
        nom::character::complete::one_of("UDLR"),
        nom::character::complete::space0,
        nom::character::complete::digit1,
    )(input)?;
    let (input, _) = nom::branch::alt((
        nom::bytes::complete::take_until("eof"),
        nom::bytes::complete::take_until("\n"),
    ))(input)?;
    let (input, _) = nom::bytes::complete::tag("\n")(input)?;
    let distance = distance.parse::<i32>().unwrap();
    let direction = match direction {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => unreachable!(),
    };
    assert!(distance > 0);
    Ok((
        input,
        Instruction {
            direction,
            distance,
        },
    ))
}
fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, output) = nom::multi::many1(parse_line)(input)?;
    Ok((input, output))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("R 6 (#70c710)\n", Instruction { direction: Direction::Right, distance: 6 })]
    #[case("D 5 (#70c710)\n", Instruction { direction: Direction::Down, distance: 5 })]
    #[case("U 4 (#70c710)\n", Instruction { direction: Direction::Up, distance: 4 })]
    #[case("L 3 (#70c710)\n", Instruction { direction: Direction::Left, distance: 3 })]
    fn test_parse_line(#[case] input: &str, #[case] expected: Instruction) {
        let (_, output) = parse_line(input).unwrap();
        assert_eq!(output, expected)
    }

    #[test]
    fn test_parse() {
        let input = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
";
        let expected = vec![
            Instruction {
                direction: Direction::Right,
                distance: 6,
            },
            Instruction {
                direction: Direction::Down,
                distance: 5,
            },
            Instruction {
                direction: Direction::Left,
                distance: 2,
            },
        ];

        let (_, output) = parse(input).unwrap();
        assert_eq!(output, expected)
    }

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
            62
        );
    }
}
