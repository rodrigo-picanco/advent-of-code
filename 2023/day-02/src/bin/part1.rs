use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    let mut result = vec![];
    let mut i: usize = 0;
    input.trim().lines().map(|x| transform_line(x)).inspect(|x| println!("{:?}", x)).for_each(|x| {
        i += 1;
        if x.green <= 13 && x.red <= 12 && x.blue <= 14 {
            result.push(i);
        } 
    });

    result.iter().sum()
}

#[derive(Debug, PartialEq)]
struct Iteration {
    blue: usize,
    red: usize,
    green: usize,
}

// "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
fn transform_line(input: &str) -> Iteration {
    let mut result = Iteration {
        red: 0,
        blue: 0,
        green: 0,
    };
    Regex::new(r"(?<d>\d+) (?<c>blue|green|red)")
        .unwrap()
        .captures_iter(input)
        .map(|x| {
            let number = x
                .name("d")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("should be a number");
            let color = x.name("c").unwrap().as_str();
            (color, number)
        })
        .for_each(|(color, number)| {
            match color {
                "red" => {
                    if number > result.red {
                        result.red = number
                    }
                }
                "green" => {
                    if number > result.green {
                        result.green = number
                    }
                }
                "blue" => {
                    if number > result.blue {
                        result.blue = number
                    }
                }
                _ => {}
            };
        });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(
            transform_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Iteration {
                green: 2,
                red: 4,
                blue: 6,
            }
        );
        assert_eq!(
            transform_line("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            Iteration {
                green: 3,
                red: 1,
                blue: 4
            }
        );
        assert_eq!(
            transform_line(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            Iteration {
                green: 13,
                red: 20,
                blue: 6
            }
        );
        assert_eq!(
            transform_line(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            Iteration {
                green: 3,
                red: 14,
                blue: 15,
            }
        );
        assert_eq!(
            transform_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            Iteration {
                green: 3,
                red: 6,
                blue: 2
            }
        );
        assert_eq!(
            process_input(
                "
                Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
                "
            ),
            8
        );
    }
}
