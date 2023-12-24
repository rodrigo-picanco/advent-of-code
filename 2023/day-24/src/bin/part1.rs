use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("./input1.txt");
    let range = 200000000000000..=400000000000000;
    let output = process_input(input, range);
    dbg!(output);
}

fn process_input(input: &str, test_area: RangeInclusive<isize>) -> usize {
    let points = parse(input);
    let mut crossings = 0;
    for x in 0..points.len() {
        for y in (x+1)..points.len() {
            if is_crossing(points[x].clone(), points[y].clone(), &test_area) {
                crossings += 1;
            }
        }
    }
    crossings
}

#[derive(Clone, Debug)]
struct Coordinate {
    position: isize,
    velocity: isize,
}

#[derive(Clone, Debug)]
struct Point {
    x: Coordinate,
    y: Coordinate,
}
impl Point {
    fn new((x_pos, y_pos, x_vecty, y_vecty): (isize, isize, isize, isize)) -> Self {
        Self {
            x: Coordinate {
                position: x_pos, 
                velocity: x_vecty,
            },
            y: Coordinate {
                position: y_pos,
                velocity: y_vecty,
            },
        }
    }
}


fn parse_line(input: &str) -> Point {
    let parts = input.split(" @ ").collect::<Vec<_>>();
    let position = parts[0].split(", ").collect::<Vec<_>>();
    let velocity = parts[1].split(", ").collect::<Vec<_>>();

    Point::new((
        position[0].trim().parse().unwrap(),
        position[1].trim().parse().unwrap(),
        velocity[0].trim().parse().unwrap(),
        velocity[1].trim().parse().unwrap(),
    ))
}
fn parse(input: &str) -> Vec<Point> {
    input.lines().map(parse_line).collect()
}

fn time(point: Coordinate, area: &RangeInclusive<isize>) -> usize {
    if point.velocity > 0 {
        ((point.position - *area.end()) / point.velocity).abs() as usize
    } else {
        ((point.position - *area.start()) / point.velocity).abs() as usize 
    }
}

fn smallest_time(a: Point, b: Point, area: &RangeInclusive<isize>) -> usize {
    let a_x = time(a.x, &area); 
    let b_x = time(b.x, &area);
    let a_y = time(a.y, &area);
    let b_y = time(b.y, &area);
    std::cmp::min(std::cmp::min(a_x, b_x), std::cmp::min(a_y, b_y))
}

fn position_in_time(point: &Point, time: usize) -> Point {
    Point {
        x: Coordinate {
            position: point.x.position + (point.x.velocity * time as isize),
            velocity: point.x.velocity,
        },
        y: Coordinate {
            position: point.y.position + (point.y.velocity * time as isize),
            velocity: point.y.velocity,
        },
    }
}

fn is_crossing(a: Point, b: Point, area: &RangeInclusive<isize>) -> bool {
    let time = smallest_time(a.clone(), b.clone(), area);

    let a_end = position_in_time(&a, time);
    let b_end = position_in_time(&b, time);

    let range_start_x = a.x.position.min(a_end.x.position);
    let range_end_x = a.x.position.max(a_end.x.position);
    let range_start_y = a.y.position.min(a_end.y.position);
    let range_end_y = a.y.position.max(a_end.y.position);

    let range_x = range_start_x..=range_end_x;
    let range_y = range_start_y..=range_end_y;

    range_x.contains(&b_end.x.position) && range_y.contains(&b_end.y.position)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        Coordinate {
                position: 19,
                velocity: -2,
            },
        6
        )]
    #[case(
        Coordinate {
                position: 13,
                velocity: 1,
            },
        14 
        )]
    fn test_time(#[case] coord: Coordinate, #[case] expected: usize) {
        let area = 7..=27;
        assert_eq!(time(coord, &area), expected);
    }

    #[test]
    fn test_smallest_time() {
        let a = Point {
            x: Coordinate {
                position: 19,
                velocity: -2,
            },
            y: Coordinate {
                position: 13,
                velocity: 1,
            },
        };
        let b = Point {
            x: Coordinate {
                position: 18,
                velocity: -1,
            },
            y: Coordinate {
                position: 19,
                velocity: -1,
            },
        };
        let area = 7..=27;
        assert_eq!(smallest_time(a, b, &area), 6);
    }

    #[rstest]
    #[case((19, 13, -2, 1), (18, 19, -1, -1), true)]
    #[case((19, 13, -2, 1), (20, 25, -2, -2), true)]
    #[case((19, 13, -2, 1), (20, 19, 1, -5), false)]
    #[case((18, 19, -1, -1), (12, 31, -1, -2), false)]
    #[case((18, 19, -1, -1), (20, 19, 1, -5), false)]
    #[case((20, 25, -2, -2), (12, 31, -1, -2), false)]
    #[case((20, 25, -2, -2), (20, 19, 1, -5), false)]
    #[case((12, 31, -1, -2), (20, 19, 1, -5), false)]
    fn test_is_crossing(#[case] a: (isize, isize, isize, isize), #[case] b: (isize, isize, isize, isize), #[case] expected: bool) {
        let area = 7..=27;
        assert_eq!(is_crossing(
Point::new(a),
Point::new(b),
                &area), expected);
    }

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
